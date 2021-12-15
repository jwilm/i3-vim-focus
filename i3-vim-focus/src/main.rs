//! A rust implementation of i3_vim_focus
//!
//! Original version from
//! https://faq.i3wm.org/question/3042/change-the-focus-of-windows-within-vim-and-i3-with-the-same-keystroke/
//!
//! Usage:
//!    i3-vim-focus [left|right|up|down]
//!
//! Requires that libxdo is installed

extern crate i3ipc;
extern crate jwilm_xdo as xdo;

use std::process::Command;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Options {
    #[structopt(long, help = "specify a vim binary instead of /usr/local/bin/vim")]
    vim: Option<String>,

    #[structopt(subcommand)]
    direction: Direction,
}

#[derive(Debug, StructOpt, Copy, Clone)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl Direction {
    pub fn to_vim_direction(&self) -> &'static str {
        match *self {
            Direction::Up => "k",
            Direction::Down => "j",
            Direction::Left => "h",
            Direction::Right => "l",
        }
    }

    pub fn to_i3_direction(&self) -> &'static str {
        match *self {
            Direction::Up => "up",
            Direction::Down => "down",
            Direction::Left => "left",
            Direction::Right => "right",
        }
    }
}

fn main() {
    let options = Options::from_args();

    let vim = options
        .vim
        .as_ref()
        .map(|s| &s[..])
        .unwrap_or("/usr/local/bin/vim");

    let direction = options.direction;

    let xdo = xdo::Xdo::new().expect("create xdo");
    let window = xdo.get_active_window();

    if let Ok(window) = window {
        let window_name = window.get_name();

        if let Ok(window_name) = window_name {
            println!("window_name={}", window_name);

            if let Some(idx) = window_name.find("VIM:") {
                // by default, the server name appears at the end of the window title. By
                // convention, the server name starts with VIM:.
                let servername = &window_name[idx..];
                let remoteexpr = format!(
                    "execute(\"call Focus('{}', '{}')<CR>\")",
                    direction.to_i3_direction(),
                    direction.to_vim_direction()
                );
                Command::new(&vim[..]) // XXX hard-coded path
                    .args(&["--servername", servername, "--remote-expr", &remoteexpr[..]])
                    .output()
                    .expect("ran vim command");

                return;
            }
        }
    }

    let mut conn = i3ipc::I3Connection::connect().expect("connect i3");
    let command = format!("focus {}", direction.to_i3_direction());
    println!("sending command: {}", command);
    conn.run_command(&command).expect("send i3 message");
}
