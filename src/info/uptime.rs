use crate::chrono;
use crate::sysinfo;
use crate::mlua;

use crate::errors;
use super::kernel;
use super::utils;

use std::path::{ Path };

use mlua::prelude::*;
use chrono::{ Utc, DateTime, Datelike, Timelike, TimeZone };
use sysinfo::{ System };

use crate::{ Inject };
use kernel::{ Kernel };
use utils::{ get_system };

pub(crate) struct Uptime ( pub DateTime<Utc> );

impl Uptime {
	pub fn new(k: &Kernel) -> errors::Result<Self> {
		let uptime_seconds;
		match k.name.as_str() {
			"Linux"|"Windows"|"MINIX" => {
				if Path::new("/proc/uptime").exists() {
					uptime_seconds = System::uptime() as i64;
				} else {
					let boot_time = System::boot_time() as i64;
					let now_time = Utc::now().timestamp();
					uptime_seconds = boot_time - now_time;
				}
			}
			_ => { uptime_seconds = 0; }
		}
		Ok(Uptime(Utc.timestamp_opt(uptime_seconds, 0)
            .single()
            .ok_or_else(|| errors::FreshfetchError::General(format!("Failed to create timestamp for uptime: {}", uptime_seconds)))?))
	}
}

impl Inject for Uptime {
	fn inject(&self, lua: &mut Lua) -> errors::Result<()> {
		let globals = lua.globals();

		let t = lua.create_table().map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        t.set("days", self.0.ordinal0()).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        t.set("hours", self.0.hour()).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        t.set("minutes", self.0.minute()).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        t.set("seconds", self.0.second()).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        globals.set("uptime", t).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        Ok(())
	}
}
