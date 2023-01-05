use std::sync::Arc;
use std::sync::atomic::{AtomicU16, AtomicU8, Ordering};
use std::thread;
use std::time::Duration;

pub struct Timer {
    dt: Arc<AtomicU8>,
    st: Arc<AtomicU8>,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            dt: Arc::new(AtomicU8::new(0)),
            st: Arc::new(AtomicU8::new(0)),
        }
    }
    pub fn start(&mut self) {
        let dt = Arc::clone(&self.dt);
        let st = Arc::clone(&self.st);

        thread::spawn(move || loop {
            thread::sleep(Duration::new(0, 16666666));

            let dt_curr = dt.load(Ordering::Relaxed);
            let st_curr = st.load(Ordering::Relaxed);

            if dt_curr > 0 { dt.store(dt_curr - 1, Ordering::Relaxed); }
            if st_curr > 0 { st.store(st_curr - 1, Ordering::Relaxed); }
        });
    }
    pub fn set_dt(&mut self, dt_value: u8) {
        let dt = Arc::clone(&self.dt);
        dt.store(dt_value, Ordering::Relaxed);
    }
    pub fn set_st(&mut self, st_value: u8) {
        let st = Arc::clone(&self.st);
        st.store(st_value, Ordering::Relaxed);
    }
    pub fn get_dt(&mut self) -> u8 {
        self.dt.load(Ordering::Relaxed)
    }
    pub fn get_st(&mut self) -> u8 {
        self.st.load(Ordering::Relaxed)
    }
}