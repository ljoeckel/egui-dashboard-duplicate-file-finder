use crate::scanner::mediatype::Control;

use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub struct Messenger {
    pub scanner_control: Arc<Mutex<Control>>,
    pub stdlog: Arc<Mutex<Vec<String>>>,
    pub errlog: Arc<Mutex<Vec<String>>>,
    pub reslog: Arc<Mutex<Vec<String>>>,
    pub info: Arc<Mutex<String>>,
    pub cntmax: Arc<Mutex<usize>>,
    pub cntcur: Arc<Mutex<usize>>,
}

impl Messenger {
    pub fn new() -> Messenger {
        Messenger {
            scanner_control: Arc::new(Mutex::new(Control::INFO)),
            stdlog: Arc::new(Mutex::new(Vec::new())),
            errlog: Arc::new(Mutex::new(Vec::new())),
            reslog: Arc::new(Mutex::new(Vec::new())),
            info: Arc::new(Mutex::new(String::new())),
            cntmax: Arc::new(Mutex::new(0)),
            cntcur: Arc::new(Mutex::new(0)),
        }
    }

    pub fn clear(&mut self) {
        *self.scanner_control.lock().unwrap() = Control::INFO;
        self.stdlog.lock().unwrap().clear();
        self.errlog.lock().unwrap().clear();
        self.reslog.lock().unwrap().clear();
        *self.info.lock().unwrap() = String::from("");
        *self.cntmax.lock().unwrap() = 0;
        *self.cntcur.lock().unwrap() = 0;
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

    pub fn push_reslog(&self, str: String) {
        self.reslog.lock().unwrap().push(str.clone());
    }

    pub fn push_errlog(&self, str: String) {
        self.errlog.lock().unwrap().push(str.clone());
    }

    pub fn info(&self, str: String) {
        *self.info.lock().unwrap() = str;
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

    pub fn cntmax(&self, cntmax: usize) {
        *self.cntmax.lock().unwrap() = cntmax;
    }

    pub fn cntcur(&self, cntcur: usize) {
        *self.cntcur.lock().unwrap() = cntcur;
    }
}
