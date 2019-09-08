mod windowmanager;
mod xlibwrapper;
mod models;
mod runner;
mod callbacks;
mod config;
mod layout;

use windowmanager::*;
use runner::*;
use xlibwrapper::core::*;
use std::rc::Rc;
use std::process::Command;

use crate::config::*;


fn main() {

    let xlib = Rc::new(XlibWrapper::new());
    let window_manager = WindowManager::new(xlib.clone());
    call_commands();
    Runner::new(xlib, window_manager).run();
    loop {}
}

// rewrite to accomodate multiple command structs
fn call_commands() {
    
    let commands = match &CONFIG.commands {
        Some(commands) => commands.clone(),
        None => { return }
    };

    let commands: Vec<Command> = commands
        .iter()
        .map(|cmd| {
            let mut tmp_cmd = Command::new(cmd.program.clone());
            cmd.args
                .iter()
                .for_each(|arg| {
                    tmp_cmd.arg(arg);
                });
            tmp_cmd
        }).collect::<Vec<Command>>();
    
    commands.into_iter().for_each(|mut cmd| {
        match cmd.spawn() {
            Ok(_) => {},
            Err(e) => println!("Failed to run command. Error: {}", e)
        }
    })

}
