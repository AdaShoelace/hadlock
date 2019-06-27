extern crate x11;
extern crate x11_dl;
extern crate libc;
extern crate simplelog;
extern crate lazy_static;

mod windowmanager;
//mod windowmanager_bup;
mod xlibwrapper;
mod models;

//use windowmanager_bup as windowmanager;
use windowmanager::*;

fn main() {

    let mut window_manager = WindowManager::new();
    window_manager.run();
    loop {}
}
