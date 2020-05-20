use std::fmt::{Debug, Formatter, Result};
use std::ops::{Drop, FnMut};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use std::thread::{self, JoinHandle};

pub trait Action: FnMut() -> () + Send + Sync {}
impl<T> Action for T where T: FnMut() -> () + Send + Sync {}

// impl std::fmt::Debug for dyn Action {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f, "Action {{..}}")
//     }
// }

pub struct Threader<F: Action> {
    action: F,
    stop: AtomicBool,
    thr: Option<JoinHandle<()>>,
}

impl<F: Action> Debug for Threader<F> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "Action {{ action: {:?}, stop: {:?}, thr: {:?} }}",
            "Action { .. }", &self.stop, &self.thr
        )
    }
}

impl<F: Action> Drop for Threader<F> {
    fn drop(&mut self) {
        println!("dropping");
        self.stop.store(true, Ordering::Relaxed);
        let _ = self.thr.take().unwrap().join();
        println!("dropped");
    }
}

impl<F: 'static + Action> Threader<F> {
    fn run(&mut self) {
        loop {
            if self.stop.load(Ordering::Relaxed) {
                println!("stopping in run");
                break;
            }
            (self.action)();
        }
    }

    pub fn new(action: F) -> Arc<Mutex<Self>> {
        let stop = AtomicBool::new(false);

        let me = Self {
            action,
            stop,
            thr: Some(thread::spawn(|| {})),
        };

        let me = Arc::new(Mutex::new(me));
        let mc = me.clone();
        {
            let mut me_u = me.lock().unwrap();
            me_u.thr = Some(thread::spawn(move || {
                let mut s = mc.lock().unwrap();
                s.run();
            }));
        }
        me.clone()
    }
}
