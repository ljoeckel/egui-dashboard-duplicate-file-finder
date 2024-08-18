use crate::scanner::mediatype::Control;

use std::sync::{Arc, Mutex, MutexGuard};

#[derive(Clone, Debug)]
pub struct Messenger {
    scanner_control: Arc<Mutex<Control>>,
    stdlog: Arc<Mutex<Vec<String>>>,
    errlog: Arc<Mutex<Vec<String>>>,
    reslog: Arc<Mutex<Vec<String>>>,
    checked: Arc<Mutex<Vec<bool>>>,
    info: Arc<Mutex<String>>,
    progress: Arc<Mutex<f32>>,
    group_lock: Arc<Mutex<bool>>, // Used to protect reslog+checked updates
}

impl Messenger {
    pub fn new() -> Messenger {
        Messenger {
            scanner_control: Arc::new(Mutex::new(Control::INFO)),
            stdlog: Arc::new(Mutex::new(Vec::new())),
            errlog: Arc::new(Mutex::new(Vec::new())),
            reslog: Arc::new(Mutex::new(Vec::new())),
            checked: Arc::new(Mutex::new(Vec::new())),
            info: Arc::new(Mutex::new(String::new())),
            progress: Arc::new(Mutex::new(0.0)),
            group_lock: Arc::new(Mutex::new(true)),
        }
    }

    pub fn clear(&self) {
        *self.scanner_control.lock().unwrap() = Control::INFO;
        self.stdlog.lock().unwrap().clear();
        self.errlog.lock().unwrap().clear();
        self.reslog.lock().unwrap().clear();
        self.checked.lock().unwrap().clear();
        *self.info.lock().unwrap() = "".to_owned();
        *self.progress.lock().unwrap() = 0.0;
    }

    pub fn stop(&self) {
        *self.scanner_control.lock().unwrap() = Control::STOP;
        self.set_progress(0, 0, "");
    }

    pub fn is_stopped(&self) -> bool {
        match *self.scanner_control.lock().unwrap() {
            Control::STOP => true,
            _ => false,
        }
    }

    pub fn push_stdlog(&self, str: String) {
        self.stdlog.lock().unwrap().push(str.clone());
    }

    pub fn stdlog(&self) -> MutexGuard<Vec<String>> {
        self.stdlog.lock().unwrap()
    }

    pub fn push_reslog(&self, str: String) {
        let _l = self.group_lock.lock();
        self.reslog.lock().unwrap().push(str.clone());
        self.checked.lock().unwrap().push(false);
    }

    pub fn reslog(&self) -> MutexGuard<Vec<String>> {
        self.reslog.lock().unwrap()
    }

    pub fn push_errlog(&self, str: String) {
        self.errlog.lock().unwrap().push(str.clone());
    }

    pub fn errlog(&self) -> MutexGuard<Vec<String>> {
        self.errlog.lock().unwrap()
    }

    pub fn set_info(&self, str: String) {
        *self.info.lock().unwrap() = str;
    }

    pub fn checked(&self) -> MutexGuard<Vec<bool>> {
        self.checked.lock().unwrap()
    }

    pub fn info(&self) -> String {
        return self.info.lock().unwrap().to_string();
    }

    pub fn cntstd(&self) -> usize {
        self.stdlog.lock().unwrap().len()
    }

    pub fn cnterr(&self) -> usize {
        self.errlog.lock().unwrap().len()
    }

    pub fn cntres(&self) -> usize {
        self.reslog.lock().unwrap().len()
    }

    pub fn set_progress(&self, max: usize, current: usize, info: &str) {
        *self.progress.lock().unwrap() = current as f32 / max as f32;
        if !info.is_empty() {
            *self.info.lock().unwrap() = String::from(info);
        }
    }
    pub fn progress(&self) -> f32 {
        *self.progress.lock().unwrap()
    }
}
