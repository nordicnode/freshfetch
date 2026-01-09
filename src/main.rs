#[macro_use]
pub(crate) extern crate lazy_static;
pub(crate) extern crate chrono;
pub(crate) extern crate clap;

pub(crate) extern crate mlua;
pub(crate) extern crate regex;
pub(crate) extern crate sysinfo;
pub(crate) extern crate term_size;
pub(crate) extern crate uname;
pub(crate) extern crate users;
pub(crate) extern crate dirs;

pub(crate) mod art;
pub(crate) mod assets;
pub(crate) mod errors;
pub(crate) mod info;
pub(crate) mod layout;
pub(crate) mod misc;
pub(crate) mod utils;

use clap::{Command, Arg};
use mlua::prelude::*;

use assets::defaults::LAYOUT;
use assets::{ANSI, HELP, PRINT};
use layout::Layout;

use std::env::var;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};

pub(crate) struct Arguments {
	pub ascii_distro: Option<String>,
	pub logo: bool,
}

pub(crate) trait Inject {
	fn prep(&mut self) -> errors::Result<()> { Ok(()) }
	fn inject(&self, _lua: &mut Lua) -> errors::Result<()> { Ok(()) }
}

fn main() {
    if let Err(e) = run() {
        errors::handle(&e);
    }
}

fn run() -> errors::Result<()> {
	let matches = Command::new("freshfetch")
		.version("0.0.1")
		.author("Jack Johannesen")
		.about("A fresh take on neofetch.")
		.override_help(HELP)
		.arg(
			Arg::new("ascii_distro")
				.long("ascii_distro")
				.short('a')
				.num_args(1)
				.value_name("ASCII_DISTRO"),
		)
		.arg(
			Arg::new("logo")
				.long("logo")
				.short('l')
				.action(clap::ArgAction::SetTrue),
		)
		.get_matches();

	let args = Arguments {
		ascii_distro: matches.get_one::<String>("ascii_distro").cloned(),
		logo: matches.get_flag("logo"),
	};

	let mut ctx = Lua::new();
    
    // Set 'logo' global for Lua layouts
    ctx.globals().set("logo", args.logo).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;

	ctx.load(PRINT).exec().map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
	ctx.load(ANSI).exec().map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;

	let mut layout = Layout::new(&args)?;
	layout.prep()?;
	layout.inject(&mut ctx)?;

	let layout_file = dirs::home_dir()
		.unwrap_or_else(|| PathBuf::from("."))
		.join(".config/freshfetch/layout.lua");

	if layout_file.exists() {
		let v = read_to_string(&layout_file).map_err(|e| {
            errors::FreshfetchError::Io(layout_file.to_string_lossy().into_owned(), e.to_string())
        })?;
        
        ctx.load(&v).exec().map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        
        let output = ctx.globals().get::<&str, String>("__freshfetch__").map_err(|e| {
            errors::FreshfetchError::Lua(e.to_string())
        })?;
        
        print!("{}", output);
	} else {
		ctx.load(LAYOUT).exec().map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        
        let output = ctx.globals().get::<&str, String>("__freshfetch__").map_err(|e| {
            errors::FreshfetchError::Lua(e.to_string())
        })?;
        
        print!("{}", output);
	}
    
    Ok(())
}
