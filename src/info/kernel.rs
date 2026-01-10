use crate::uname;
use crate::mlua;

use crate::errors;

use mlua::prelude::*;
use uname::{ uname };

use crate::{ Inject };

use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct Kernel {
	pub name: String,
	pub version: String,
	pub architecture: String,
}

impl Kernel {
	pub fn new() -> errors::Result<Self> {
		let uname = uname().map_err(|e| errors::FreshfetchError::General(format!("Failed to run `uname()`: {}", e)))?;
		let name;
		match uname.sysname.as_str() {
			"Darwin" => { name = String::from("Darwin"); }
			"SunOS" => { name = String::from("Solaris"); }
			"Haiku" => { name = String::from("Haiku"); }
			"MINIX" => { name = String::from("MINIX"); }
			"AIX" => { name = String::from("AIX"); }
			"FreeMiNT" => { name = String::from("FreeMiNT"); }
			"Linux" => { name = String::from("Linux"); }
			"DragonFly" => { name = String::from("BSD"); }
			"Bitrig" => { name = String::from("BSD"); }
			other => {
				if other.starts_with("GNU") { name = String::from("Linux"); }
				else if other.ends_with("BSD") { name = String::from("BSD"); }
				else if other.starts_with("CYGWIN") || other.starts_with("MSYS") || other.starts_with("MINGW") {name = String::from("Windows"); }
				else {
					return Err(errors::FreshfetchError::General(format!("Unexpected OS \"{}\". Support needed.", other)));
				}
			}
		}
		Ok(Kernel {
			name,
			version: uname.release,
			architecture: uname.machine,
		})
	}
}

impl Inject for Kernel {
	fn inject(&self, lua: &mut Lua) -> errors::Result<()> {
		let globals = lua.globals();

		let t = lua.create_table().map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        t.set("name", self.name.as_str()).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        t.set("version", self.version.as_str()).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        t.set("architecture", self.architecture.as_str()).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        globals.set("kernel", t).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        Ok(())
	}
}