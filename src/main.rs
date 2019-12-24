#[macro_use]
extern crate log;

#[macro_use]
extern crate derivative;

//mod windowmanager;
mod wm;
mod xlibwrapper;
mod models;
//mod runner;
mod config;
//mod layout;
mod reducers;
mod state;
mod hdl_dispatcher;
mod hdl_reactor;

//use windowmanager::*;
//use runner::*;
use xlibwrapper::{
    core::*,
    xlibmodels::*,
};
use std::rc::Rc;
use std::process::Command;
use std::thread;
use std::sync::mpsc;
use fern;
use chrono;
use nix::sys::signal::{self, SigHandler, Signal};
use crate::config::*;
use reducer::*;
use state::State;

pub type HadlockResult<T> = Result<T, Box<dyn std::error::Error>>;
pub type HadlockOption<T> = Option<T>;

fn main() -> HadlockResult<()> {
    init_logger()?;
    let (tx, rx) = mpsc::channel::<bool>();

    let xlib = Rc::new(XlibWrapper::new());
    info!("Screens on startup: {:?}", xlib.get_screens());


    // Avoid zombies by ignoring SIGCHLD
    unsafe { signal::signal(Signal::SIGCHLD, SigHandler::SigIgn) }.unwrap();
    call_commands(ExecTime::Pre);
    thread::spawn(move || {
        match rx.recv() {
            Ok(true) => call_commands(ExecTime::Post),
            _ => { return },
        }
    });
    
    hdl_dispatcher::run(xlib);

    //Runner::new(xlib.clone(), WindowManager::new(xlib.clone())).run(tx);

    Ok(())
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

fn init_logger() -> HadlockResult<()> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!("{}[{}][{}] {}",
                    chrono::Local::now()
                    .format("[%Y-%m-%d][%H:%M:%S]"),
                    record.target(),
                    record.level(),
                    message))
        })
    .level(log::LogLevelFilter::Debug)
        .chain(std::io::stderr())
        //.chain(fern::log_file("/var/log/hadlock/output.log")?)
        .apply()?;
    Ok(())
}
