use core::{marker::PhantomData, ops};

pub struct MMIODerefWrapper<T> {
    mmio_start_addr: usize,
    phantom: PhantomData<T>,
}

impl<T> MMIODerefWrapper<T> {
    pub const unsafe fn new(mmio_start_addr: usize) -> Self {
        Self {
            mmio_start_addr,
            phantom: PhantomData,
        }
    }
}

impl<T> ops::Deref for MMIODerefWrapper<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.mmio_start_addr as *const T) }
    }
}
