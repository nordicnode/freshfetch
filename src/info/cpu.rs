use crate::mlua;
use crate::regex;

use crate::errors;
use super::kernel;

use std::fs;
use std::path::{ Path };

use mlua::prelude::*;
use regex::{ Regex };

use crate::{ Inject }; 
use kernel::{ Kernel };

use serde::Serialize;

#[derive(Debug, Serialize)]
pub(crate) struct Cpu {
	/// The name of the CPU.
	pub name: String,
	/// The name of the CPU, without any information cut off.
	pub full_name: String,
	/// The frequency of the CPU, in MHz.
	pub freq: f32,
	/// The number of cores in the CPU.
	pub cores: i32,
}

impl Cpu {
	pub fn new(k: &Kernel) -> Option<Self> {
		let mut name: Option<String> = None;
		let mut freq: Option<f32> = None;
		let mut cores: Option<i32> = None;
		match k.name.as_str() {
			"Linux"|"MINIX"|"Windows" => {
				// TODO: Neofetch has some code to handle oddball CPU
				// architectures here. Idk if rust has support for those, but
				// porting that functionality wouldn't do much harm.
				
				match fs::read_to_string("/proc/cpuinfo") {
					Ok(cpu_info) => {
						let cpu_info_lines: Vec<&str> = cpu_info.split("\n").collect();

						// Get CPU name.
						name = {
							let mut to_return = None;
							let mut skip = false;
							for line in cpu_info_lines.iter() {
								if !skip
									&& (line.starts_with("model name")
									|| line.starts_with("Hardware")
									|| line.starts_with("Processor")
									|| line.starts_with("cpu model")
									|| line.starts_with("chip type")
									|| line.starts_with("cpu type")) {
										let split: Vec<&str> = line.split(": ").collect();
										to_return = Some(String::from(split[1]));
										skip = true;
									}
							}
							to_return
						};

						// Get CPU frequency.
						freq = {
							if Path::new("/sys/devices/system/cpu/cpu0/cpufreq/").exists() {
								let mut to_return = None;
								let to_check = [
									"/sys/devices/system/cpu/cpu0/cpufreq/bios_limit",
									"/sys/devices/system/cpu/cpu0/cpufreq/scaling_max_freq",
									"/sys/devices/system/cpu/cpu0/cpufreq/cpuinfo_max_freq",
								];
								for file in to_check.iter() {
									if to_return.is_none() {
										if let Ok(mut bios_limit) = fs::read_to_string(file) {
          												bios_limit = bios_limit
          													.replace("\n", "")
          													.replace("\t", "");
          												if let Ok(freq) = bios_limit.parse::<f32>() {
          													to_return = Some(freq / 1000.0);
          												}
          											}
									}
								}
								to_return
							} else {
								let mut to_return = None;
								let mut skip = false;
								for line in cpu_info_lines.iter() {
									if !skip
										&& (line.starts_with("cpu MHz")
										|| line.starts_with("clock")) {
											let split: Vec<&str> = line.split(": ").collect();
											let to_parse = String::from(split[1]).replace("MHz", "");
											to_return = to_parse.parse::<f32>().ok().map(|f| f / 1000.0);
											skip = true;
										}
								}
								to_return
							}
						};

						// Get CPU cores.
						cores = {
							let mut to_return = 0;
							for line in cpu_info_lines.iter() { if line.starts_with("processor") { to_return += 1; } }
							Some(to_return)
						};
					}
					Err(_) => { /* /proc/cpuinfo not readable, proceed without it */ }
				}
			}
			_ => (),
		}
		if let (Some(name_val), Some(freq_val), Some(cores_val)) = (name, freq, cores) {
			Some(Cpu {
				name: Self::clean_cpu_name(&name_val),
				full_name: name_val,
				freq: freq_val,
				cores: cores_val,
			})
		} else {
			None
		}
	}

	fn clean_cpu_name(name: &str) -> String {
		let mut to_return = name
			.replace("(tm)", "")
			.replace("(TM)", "")
			.replace("(R)", "")
			.replace("(r)", "")
			.replace("CPU", "")
			.replace("Intel", "")
			.replace("AMD", "")
			.replace("Processor", "")
			.replace("Dual-Core", "")
			.replace("Quad-Core", "")
			.replace("Six-Core", "")
			.replace("Eight-Core", "")
			.replace("Quad-Core", "");
		{
			let regex = Regex::new(r#"(?i)\d\d?-Core"#).unwrap();
			to_return = String::from(regex.replace_all(&to_return, ""));
		}
		{
			let regex = Regex::new(r#"(?i), .*? Compute Cores"#).unwrap();
			to_return = String::from(regex.replace_all(&to_return, ""));
		}
		to_return = to_return.replace("Cores ", " ");
		{
			let regex = Regex::new(r#"(?i)\("AuthenticAMD".*?\)"#).unwrap();
			to_return = String::from(regex.replace_all(&to_return, ""));
		}
		{
			let regex = Regex::new(r#"(?i)with Radeon .*? Graphics"#).unwrap();
			to_return = String::from(regex.replace_all(&to_return, ""));
		}
		to_return = to_return
			.replace(", altivec supported", "")
			.replace("Technologies, Inc", "")
			.replace("Core2", "Core 2");
		{
			let regex = Regex::new(r#"FPU.*?"#).unwrap();
			to_return = String::from(regex.replace_all(&to_return, ""));
		}
		{
			let regex = Regex::new(r#"Chip Revision.*?"#).unwrap();
			to_return = String::from(regex.replace_all(&to_return, ""));
		}
		
		// Squash multiple spaces and trim
		let regex = Regex::new(r#"\s+"#).unwrap();
		to_return = String::from(regex.replace_all(&to_return, " "));
		
		String::from(to_return.trim())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_clean_cpu_name() {
		assert_eq!(Cpu::clean_cpu_name("Intel(R) Core(TM) i7-10700K CPU @ 3.80GHz"), "Core i7-10700K @ 3.80GHz");
		assert_eq!(Cpu::clean_cpu_name("AMD Ryzen 7 3700X 8-Core Processor"), "Ryzen 7 3700X");
		assert_eq!(Cpu::clean_cpu_name("Intel(R) Core(TM)2 Duo CPU E8400 @ 3.00GHz"), "Core 2 Duo E8400 @ 3.00GHz");
	}
}

impl Inject for Cpu {
	fn inject(&self, lua: &mut Lua) -> errors::Result<()> {
		let globals = lua.globals();

		let t = lua.create_table().map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        t.set("name", self.name.as_str()).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        t.set("fullName", self.full_name.as_str()).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        t.set("cores", self.cores).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        t.set("freq", self.freq).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        globals.set("cpu", t).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        Ok(())
	}
}