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
use std::thread;
use std::sync::mpsc;
use nix::sys::signal::{self, SigHandler, Signal};

use crate::config::*;


fn main() {
    
    let (tx, rx) = mpsc::channel::<bool>();

    let xlib = Rc::new(XlibWrapper::new());
    let window_manager = WindowManager::new(xlib.clone());
    // Avoid zombies by ignoring SIGCHLD
    unsafe { signal::signal(Signal::SIGCHLD, SigHandler::SigIgn) }.unwrap();
    call_commands(ExecTime::Pre);
    thread::spawn(move || {
        match rx.recv() {
            Ok(true) => call_commands(ExecTime::Post),
            _ => { return }, 
        }
    });
    Runner::new(xlib, window_manager).run(tx);
}

fn call_commands(exec_time: ExecTime) {
    
    let commands = match &CONFIG.commands {
        Some(commands) => commands.clone(),
        None => { return }
    };

    let commands: Vec<Command> = commands
        .iter()
        .filter(|cmd| {
            cmd.exec_time == exec_time
        })
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
