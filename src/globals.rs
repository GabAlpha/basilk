// global variable
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    pub static ref DATA_FILE: Mutex<Option<String>> = Mutex::new(None);  // data file name(If it is none, the default file will be used)
}
