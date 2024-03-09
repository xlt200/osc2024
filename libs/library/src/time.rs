#[derive(Debug, Clone, Copy)]
pub struct Time {
    sec: i64,
    nsec: i64,
}

impl Time {
    pub const fn new(sec: i64, nsec: i64) -> Self {
        Self { sec, nsec }
    }
}
