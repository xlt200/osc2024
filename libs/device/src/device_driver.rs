use library::sync::mutex::Mutex;

const MAX_DRIVER_NUM: usize = 5;

pub trait DeviceDriver {
    /**
     * # Safety
     */
    unsafe fn init(&self) -> Result<(), &'static str> {
        Ok(())
    }
}

#[derive(Copy, Clone)]
pub struct DeviceDriverDescriptor {
    device_driver: &'static (dyn DeviceDriver + Send + Sync),
    post_init: Option<unsafe fn() -> Result<(), &'static str>>,
}

impl DeviceDriverDescriptor {
    pub const fn new(
        device_driver: &'static (dyn DeviceDriver + Send + Sync),
        post_init: Option<unsafe fn() -> Result<(), &'static str>>,
    ) -> Self {
        Self {
            device_driver,
            post_init,
        }
    }
}

struct DriverManagerInner {
    next_index: usize,
    descriptors: [Option<DeviceDriverDescriptor>; MAX_DRIVER_NUM],
}

impl DriverManagerInner {
    const fn new() -> Self {
        Self {
            next_index: 0,
            descriptors: [None; MAX_DRIVER_NUM],
        }
    }
}

pub struct DriverManager {
    inner: Mutex<DriverManagerInner>,
}

impl DriverManager {
    const fn new() -> Self {
        Self {
            inner: Mutex::new(DriverManagerInner::new()),
        }
    }

    pub fn register_driver(&self, driver_descriptor: DeviceDriverDescriptor) {
        let mut inner = self.inner.lock().unwrap();
        let next_index = inner.next_index;
        inner.descriptors[next_index] = Some(driver_descriptor);
        inner.next_index += 1;
    }

    /**
     * # Safety
     *
     * - During init, drivers might do stuff with system-wide impact.
     */
    pub unsafe fn init_drivers(&self) {
        let inner = self.inner.lock().unwrap();
        inner
            .descriptors
            .iter()
            .filter_map(|x| x.as_ref())
            .for_each(|descriptor| {
                if let Err(e) = descriptor.device_driver.init() {
                    panic!("Error initializing drivers: {}", e);
                }

                if let Some(callback) = &descriptor.post_init {
                    if let Err(e) = callback() {
                        panic!("Error during driver post-init callback: {}", e);
                    }
                }
            });
    }
}

static DRIVER_MANAGER: DriverManager = DriverManager::new();

pub fn driver_manager() -> &'static DriverManager {
    &DRIVER_MANAGER
}
