extern crate x11;
extern crate x11_dl;
extern crate libc;
extern crate simplelog;
extern crate lazy_static;

mod windowmanager;
//mod windowmanager_bup;
mod xlibwrapper;
mod models;
mod runner;
mod callbacks;

//use windowmanager_bup as windowmanager;
use windowmanager::*;
use runner::*;
use xlibwrapper::core::*;
use std::rc::Rc;

fn main() {

    let xlib = Rc::new(XlibWrapper::new());
    let window_manager = WindowManager::new(xlib.clone());
    Runner::new(xlib, window_manager).run();
    loop {}
}
