#[allow(unused_imports)]
#[macro_use]
extern crate log;

mod config;
mod hdl_dispatcher;
mod hdl_reactor;
mod layout;
mod models;
mod reducers;
mod state;
mod wm;
mod xlibwrapper;

use std::{process::Command, rc::Rc, sync::mpsc, thread};
use xlibwrapper::{core::*, DisplayServer};

use crate::config::*;
use lazy_static::initialize;
use nix::sys::signal::{self, SigHandler, Signal};

pub type HadlockResult<T> = Result<T, Box<dyn std::error::Error>>;
pub type HadlockOption<T> = Option<T>;

fn main() -> HadlockResult<()> {
    init_logger()?;
    initialize(&CONFIG);

    debug!("Keybindings: {:?}", CONFIG.key_bindings);

    let (tx, rx) = mpsc::channel::<bool>();

    let xlib = Rc::new(XlibWrapper::new());
    info!("Screens on startup: {:?}", xlib.get_screens());

    // Avoid zombies by ignoring SIGCHLD
    unsafe { signal::signal(Signal::SIGCHLD, SigHandler::SigIgn) }.unwrap();
    call_commands(ExecTime::Pre);
    thread::spawn(move || {
        if let Ok(true) = rx.recv() {
            call_commands(ExecTime::Post)
        }
    });

    hdl_dispatcher::run(Box::new(xlib), tx);
    Ok(())
}

fn call_commands(exec_time: ExecTime) {
    if CONFIG.commands.is_empty() {
        return;
    }

    let commands: Vec<Command> = CONFIG
        .commands
        .iter()
        .filter(|cmd| cmd.exec_time == exec_time)
        .map(|cmd| {
            let mut tmp_cmd = Command::new(cmd.program.clone());
            cmd.args.iter().for_each(|arg| {
                tmp_cmd.arg(arg);
            });
            tmp_cmd
        })
        .collect::<Vec<Command>>();

    commands.into_iter().for_each(|mut cmd| {
        debug!("Executing: {:?}", cmd);
        match cmd.spawn() {
            Ok(_) => {}
            Err(e) => println!("Failed to run command. Error: {}", e),
        }
    })
}

fn init_logger() -> HadlockResult<()> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LogLevelFilter::Debug)
        .chain(std::io::stderr())
        //.chain(fern::log_file("~/hadlog")?)
        .apply()?;
    Ok(())
}
