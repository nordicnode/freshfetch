use crate::mlua;

use super::kernel;
use crate::errors;

use std::env;
use std::path::Path;
use std::process::Command;

use mlua::prelude::*;

use crate::Inject;
use kernel::Kernel;

use serde::Serialize;

#[derive(Serialize)]
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
					"bash" => {
						// bash --version outputs: GNU bash, version 5.1.16(1)-release ...
						let output = Command::new("bash")
							.arg("--version")
							.output()
							.map_err(|e| errors::FreshfetchError::Command("bash --version".to_string(), e.to_string()))?;
						
						let stdout = String::from_utf8(output.stdout)
							.map_err(|e| errors::FreshfetchError::General(format!("Invalid UTF8 from bash version: {}", e)))?;
						
						// Parse version from first line: "GNU bash, version X.Y.Z..."
						version = stdout
							.lines()
							.next()
							.and_then(|line| {
								line.split("version ")
									.nth(1)
									.map(|v| v.split(&[' ', '(', '-'][..]).next().unwrap_or(""))
							})
							.unwrap_or("")
							.to_string();
					}
					"fish" => {
						let output = Command::new("fish")
							.arg("--version")
							.output()
							.map_err(|e| errors::FreshfetchError::Command("fish --version".to_string(), e.to_string()))?;
						
						let stdout = String::from_utf8(output.stdout)
							.map_err(|e| errors::FreshfetchError::General(format!("Invalid UTF8 from fish version: {}", e)))?;
						
						// Parse: "fish, version X.Y.Z"
						version = stdout
							.split("version ")
							.nth(1)
							.map(|v| v.trim())
							.unwrap_or("")
							.to_string();
					}
					"nu" | "nushell" => {
						let output = Command::new(&name)
							.arg("--version")
							.output()
							.map_err(|e| errors::FreshfetchError::Command(format!("{} --version", name), e.to_string()))?;
						
						version = String::from_utf8(output.stdout)
							.map_err(|e| errors::FreshfetchError::General(format!("Invalid UTF8 from {} version: {}", name, e)))?
							.trim()
							.to_string();
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
			name,
			version,
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
