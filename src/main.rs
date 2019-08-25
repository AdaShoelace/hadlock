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
use crate::config::*;
use crate::config::config_data::*;

fn main() {


    /*println!("BorderWidth: {}", CONFIG.border_width);
      println!("DecorationHeight: {}", CONFIG.decoration_height);
      println!("InnerBorderWidth: {}", CONFIG.inner_border_width);
      println!("Workspaces: {:?}", CONFIG.workspaces);
      */

    let xlib = Rc::new(XlibWrapper::new());
    let window_manager = WindowManager::new(xlib.clone());
    Runner::new(xlib, window_manager).run();
    loop {}
}
