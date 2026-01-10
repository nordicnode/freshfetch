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

use std::fs::read_to_string;
use std::path::PathBuf;

pub(crate) struct Arguments {
	pub ascii_distro: Option<String>,
	pub logo: bool,
	pub json: bool,
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
		.version("0.2.0")
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
		.arg(
			Arg::new("json")
				.long("json")
				.short('j')
				.help("Output system info as JSON")
				.action(clap::ArgAction::SetTrue),
		)
		.get_matches();

	let args = Arguments {
		ascii_distro: matches.get_one::<String>("ascii_distro").cloned(),
		logo: matches.get_flag("logo"),
		json: matches.get_flag("json"),
	};

	// JSON output mode - bypass Lua rendering
	if args.json {
		return output_json();
	}

	let mut ctx = Lua::new();
    
    // Set 'logo' global for Lua layouts
    ctx.globals().set("logo", args.logo)?;

	ctx.load(PRINT).exec()?;
	ctx.load(ANSI).exec()?;

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
        
        ctx.load(&v).exec()?;
        
        let output: String = ctx.globals().get("__freshfetch__")?;
        
        print!("{}", output);
	} else {
		ctx.load(LAYOUT).exec()?;
        
        let output: String = ctx.globals().get("__freshfetch__")?;
        
        print!("{}", output);
	}
    
    Ok(())
}

fn output_json() -> errors::Result<()> {
	use info::Info;
	
	// Gather all info
	let info = Info::new()?;
	
	// Use serde_json for automatic serialization
	let json = serde_json::to_string_pretty(&info).map_err(|e| {
		errors::FreshfetchError::General(format!("Failed to serialize info to JSON: {}", e))
	})?;
	
	println!("{}", json);
	Ok(())
}

