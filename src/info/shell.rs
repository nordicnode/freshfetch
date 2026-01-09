use crate::mlua;

use super::kernel;
use crate::errors;

use std::env;
use std::path::Path;
use std::process::Command;

use mlua::prelude::*;

use crate::Inject;
use kernel::Kernel;

pub(crate) struct Shell {
	pub name: String,
	pub version: String,
}

impl Shell {
	pub fn new(k: &Kernel) -> errors::Result<Self> {
		let name;
		let version;
		match k.name.as_str() {
			"Linux" | "BSD" | "Windows" => {
				let shell_path = env::var("SHELL").map_err(|e| {
                    errors::FreshfetchError::General(format!("Failed to get $SHELL: {}", e))
                })?;
                
				let shell_bin = Path::new(&shell_path)
					.file_name()
					.ok_or_else(|| errors::FreshfetchError::General(format!("$SHELL path is invalid: {}", shell_path)))?
					.to_string_lossy()
					.into_owned();
                    
				name = shell_bin;
				match name.as_str() {
					"zsh" => {
						let output = Command::new("zsh")
							.arg("-c")
							.arg("printf $ZSH_VERSION")
							.output()
                            .map_err(|e| errors::FreshfetchError::Command("zsh -c printf $ZSH_VERSION".to_string(), e.to_string()))?;
                            
                        version = String::from_utf8(output.stdout)
                            .map_err(|e| errors::FreshfetchError::General(format!("Invalid UTF8 from zsh version: {}", e)))?;
					}
					_ => version = String::new(),
				}
			}
			_ => {
				name = String::new();
				version = String::new();
			}
		}
		Ok(Shell {
			name: name,
			version: version,
		})
	}
}

impl Inject for Shell {
	fn inject(&self, lua: &mut Lua) -> errors::Result<()> {
		let globals = lua.globals();

		let t = lua.create_table().map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        t.set("name", self.name.as_str()).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        t.set("version", self.version.as_str()).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        globals.set("shell", t).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        Ok(())
	}
}
