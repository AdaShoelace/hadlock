mod windowmanager;
//mod windowmanager_bup;
mod xlibwrapper;
mod models;
mod runner;
mod callbacks;
mod config;

//use windowmanager_bup as windowmanager;
use windowmanager::*;
use runner::*;
use xlibwrapper::core::*;
use std::rc::Rc;
use std::env;

fn main() {
    
    let args: Vec<String> = env::args().collect(); 
    
    match args.len() {
        2 => {
            println!("Path to config: {}", args.get(1).unwrap())
        },

        x => {
            println!("Wrong number of arguments:{}\nDefault config will be applied", x)
        }
    }

    let xlib = Rc::new(XlibWrapper::new());
    let window_manager = WindowManager::new(xlib.clone());
    Runner::new(xlib, window_manager).run();
    loop {}
}
