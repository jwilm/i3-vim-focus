//! A rust implementation of i3_vim_focus
//!
//! Original version from
//! https://faq.i3wm.org/question/3042/change-the-focus-of-windows-within-vim-and-i3-with-the-same-keystroke/
//!
//! Usage:
//!    i3-vim-focus [left|right|up|down]
//!
//! Requires that libxdo is installed

extern crate jwilm_xdo as xdo;
extern crate i3ipc;

use std::env;
use std::ascii::AsciiExt;
use std::str::FromStr;

enum Direction {
    Up, Left, Down, Right
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
}

impl FromStr for Direction {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "left" => Direction::Left,
            "right" => Direction::Right,
            "up" => Direction::Up,
            "down" => Direction::Down,
            _ => return Err("must specify one of left, right, up, or down"),
        })
    }
}

fn main() {
    let name = env::args().nth(1)
        .expect("direction was specified")
        .to_ascii_lowercase();

    let direction = Direction::from_str(&name).unwrap();

    let xdo = xdo::Xdo::new().expect("create xdo");
    let window = xdo.get_active_window();

    if let Ok(window) = window {
        let window_name = window.get_name();

        if let Ok(window_name) = window_name {

            if window_name.contains("VIM") {
                let sequence = format!("g+w+{}", direction.to_vim_direction());
                let mods = xdo.get_active_modifiers().expect("get_active_modifiers");
                window.clear_active_modifiers(&mods).expect("clear_active_modifiers");
                window.send_keysequence("Escape", None)
                    .expect("send escape");
                window.send_keysequence(&sequence, None)
                    .expect("send gw{}");
                window.set_active_modifiers(&mods).expect("set_active_modifiers");
                return;
            }
        }
    }
    let mut conn = i3ipc::I3Connection::connect().expect("connect i3");
    let command = format!("focus {}", name);
    println!("sending command: {}", command);
    conn.command(&command).expect("send i3 message");
}
