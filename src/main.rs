extern crate x11;
extern crate x11_dl;
extern crate libc;
extern crate simplelog;
extern crate lazy_static;
#[macro_use]
extern crate bitflags;

mod windowmanager;
mod xlibwrapper;

use windowmanager::*;


fn main() {
    let mut window_manager = WindowManager::new();
    window_manager.run();
    loop {}
}
