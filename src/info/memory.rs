use crate::mlua;
use crate::sysinfo;

use super::utils;
use crate::errors;

use mlua::prelude::*;
use sysinfo::{ System };

use crate::{ Inject };
use utils::{ get_system };

#[derive(Clone, Debug)]
pub(crate) struct Memory {
	pub max: u64,
	pub used: u64,
}

impl Memory {
	pub fn new() -> Self {
		let system = get_system();
		Memory {
			max: system.total_memory(),
			used: system.used_memory(),
		}	
	}
}

impl Inject for Memory {
	fn inject(&self, lua: &mut Lua) -> errors::Result<()> {
		let globals = lua.globals();

		let t = lua.create_table().map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        t.set("max", self.max).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        t.set("used", self.used).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        globals.set("memory", t).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        Ok(())
	}
}