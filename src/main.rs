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
	
	// Build JSON manually (no serde dependency)
	let mut json = String::from("{\n");
	
	// Context
	if let Some(ref ctx) = info.context {
		json.push_str(&format!("  \"user\": \"{}\",\n", ctx.user));
		json.push_str(&format!("  \"host\": \"{}\",\n", ctx.host.trim()));
	}
	
	// Kernel
	json.push_str(&format!("  \"kernel\": {{\n    \"name\": \"{}\",\n    \"version\": \"{}\"\n  }},\n", 
		info.kernel.name, info.kernel.version));
	
	// Distro
	json.push_str(&format!("  \"distro\": {{\n    \"name\": \"{}\",\n    \"architecture\": \"{}\"\n  }},\n",
		info.distro.long_name, info.distro.architecture));
	
	// Uptime - use sysinfo for simple uptime calculation
	let uptime_secs = sysinfo::System::uptime();
	let days = uptime_secs / 86400;
	let hours = (uptime_secs % 86400) / 3600;
	let minutes = (uptime_secs % 3600) / 60;
	let seconds = uptime_secs % 60;
	json.push_str(&format!("  \"uptime\": {{\n    \"days\": {},\n    \"hours\": {},\n    \"minutes\": {},\n    \"seconds\": {}\n  }},\n",
		days, hours, minutes, seconds));
	
	// Packages
	json.push_str("  \"packages\": [");
	let pkg_strings: Vec<String> = info.package_managers.0.iter()
		.map(|pm| format!("{{\"name\": \"{}\", \"count\": {}}}", pm.name, pm.packages))
		.collect();
	json.push_str(&pkg_strings.join(", "));
	json.push_str("],\n");
	
	// Shell
	json.push_str(&format!("  \"shell\": {{\n    \"name\": \"{}\",\n    \"version\": \"{}\"\n  }},\n",
		info.shell.name, info.shell.version));
	
	// Optional fields
	if let Some(ref res) = info.resolution {
		let refresh = res.refresh.map(|r| format!("{:.1}", r)).unwrap_or_else(|| "null".to_string());
		json.push_str(&format!("  \"resolution\": {{\n    \"width\": {},\n    \"height\": {},\n    \"refresh\": {}\n  }},\n",
			res.width, res.height, refresh));
	}
	
	if let Some(ref de) = info.de {
		json.push_str(&format!("  \"de\": {{\n    \"name\": \"{}\",\n    \"version\": \"{}\"\n  }},\n",
			de.0, de.1));
	}
	
	if let Some(ref wm) = info.wm {
		json.push_str(&format!("  \"wm\": \"{}\",\n", wm.0));
	}
	
	if let Some(ref cpu) = info.cpu {
		json.push_str(&format!("  \"cpu\": {{\n    \"name\": \"{}\",\n    \"cores\": {},\n    \"freq\": {}\n  }},\n",
			cpu.name, cpu.cores, cpu.freq));
	}
	
	if let Some(ref gpus) = info.gpu {
		json.push_str("  \"gpus\": [");
		let gpu_strings: Vec<String> = gpus.0.iter()
			.map(|gpu| format!("{{\"brand\": \"{}\", \"name\": \"{}\"}}", gpu.brand, gpu.name))
			.collect();
		json.push_str(&gpu_strings.join(", "));
		json.push_str("],\n");
	}
	
	// Memory
	json.push_str(&format!("  \"memory\": {{\n    \"used\": {},\n    \"max\": {}\n  }},\n",
		info.memory.used, info.memory.max));
	
	if let Some(ref mb) = info.motherboard {
		json.push_str(&format!("  \"motherboard\": {{\n    \"vendor\": \"{}\",\n    \"name\": \"{}\"\n  }},\n",
			mb.vendor, mb.name));
	}
	
	if let Some(ref host) = info.host {
		json.push_str(&format!("  \"host_model\": \"{}\",\n", host.model));
	}
	
	if let Some(ref bat) = info.battery {
		json.push_str(&format!("  \"battery\": {{\n    \"capacity\": {},\n    \"status\": \"{}\"\n  }},\n",
			bat.capacity, bat.status));
	}
	
	if let Some(ref disk) = info.disk {
		json.push_str(&format!("  \"disk\": {{\n    \"mount_point\": \"{}\",\n    \"used\": {},\n    \"total\": {}\n  }},\n",
			disk.mount_point, disk.used, disk.total));
	}
	
	if let Some(ref net) = info.network {
		json.push_str(&format!("  \"network\": {{\n    \"interface\": \"{}\",\n    \"ip\": \"{}\"\n  }}\n",
			net.interface, net.ip));
	} else {
		// Remove trailing comma from last field
		json = json.trim_end_matches(",\n").to_string();
		json.push('\n');
	}
	
	json.push_str("}\n");
	
	print!("{}", json);
	Ok(())
}

