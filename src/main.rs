extern crate x11;
extern crate libc;
extern crate simplelog;

mod windowmanager;

use windowmanager::*;


fn main() {
    let mut window_manager = WindowManager::new();
    //window_manager.run();
    loop {}
}
