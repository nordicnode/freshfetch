use crate::mlua;

use super::kernel;
use crate::errors;

use std::path::Path;
use std::process::Command;

use mlua::prelude::*;

use crate::Inject;
use kernel::Kernel;

use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct PackageManager {
	pub name: String,
	pub packages: i32,
}

impl PackageManager {
	pub fn new(name: &str, packages: i32) -> Self {
		PackageManager {
			name: String::from(name),
			packages,
		}
	}
}

#[derive(Serialize)]
pub(crate) struct PackageManagers(pub Vec<PackageManager>);

impl PackageManagers {
	pub fn new(k: &Kernel) -> errors::Result<Self> {
		let mut to_return = Vec::new();

		let has_bin = |package_manager: &str| -> bool {
			Path::new("/usr/bin/").join(package_manager).exists()
		};
        
		let mut add = |package_manager: &str, command: &str| -> errors::Result<()> {
			let packages = {
				let output = Command::new("sh")
					.arg("-c")
					.arg(command)
					.output()
                    .map_err(|e| errors::FreshfetchError::Command(command.to_string(), e.to_string()))?;
                    
                let stdout_string = String::from_utf8(output.stdout)
                    .map_err(|e| errors::FreshfetchError::General(format!("Invalid UTF8 from {}: {}", command, e)))?;
                
                let stdout_lines: Vec<&str> = stdout_string.trim().split("\n").collect();
                if stdout_string.trim().is_empty() {
                    0
                } else {
                    stdout_lines.len() as i32
                }
			};
            to_return.push(PackageManager::new(package_manager, packages));
            Ok(())
		};

		match k.name.as_str() {
			"Linux" | "BSD" | "iPhone OS" | "Solaris" => {
				if has_bin("kiss") { add("kiss", "kiss l")?; }
				if has_bin("pacman") { add("pacman", "pacman -Qq --color never")?; }
				if has_bin("dpkg") { add("dpkg", "dpkg-query -f '.\n' -W")?; }
				if has_bin("rpm") { add("rpm", "rpm -qa")?; }
				if has_bin("xbps-query") { add("xbps-query", "xbps-query -l")?; }
				if has_bin("apk") { add("apk", "apk info")?; }
				if has_bin("opkg") { add("opkg", "opkg list-installed")?; }
				if has_bin("pacman-g2") { add("pacman-g2", "pacman-g2 -Q")?; }
				if has_bin("lvu") { add("lvu", "lvu installed")?; }
				if has_bin("tce-status") { add("tce-status", "tce-status -i")?; }
				if has_bin("pkg-info") { add("pkg-info", "pkg_info")?; }
				if has_bin("tazpkg") { add("tazpkg", "tazpkg list")?; }
				if has_bin("sorcery") { add("sorcery", "gaze installed")?; }
				if has_bin("alps") { add("alps", "alps showinstalled")?; }
				if has_bin("butch") { add("butch", "butch list")?; }
				if has_bin("mine") { add("mine", "mine -q")?; }

				if has_bin("flatpak") { add("flatpak", "flatpak list")?; }
				if has_bin("snap") {
					let daemon_running = {
						let try_output = Command::new("sh")
							.arg("-c")
							.arg(r#"ps aux | grep -qFm 1 snapd"#)
							.output();
						match try_output {
							Ok(output) => output.status.success(),
							Err(_) => false,
						}
					};
					if daemon_running {
						add("snap", "snap list")?;
					}
				}
			}
			_ => {}
		}

		Ok(PackageManagers(to_return))
	}
}

impl Inject for PackageManagers {
	fn inject(&self, lua: &mut Lua) -> errors::Result<()> {
		let globals = lua.globals();

		let t = lua.create_table().map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        for (i, package_manager) in self.0.iter().enumerate() {
            let t2 = lua.create_table().map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
            t2.set("name", package_manager.name.as_str()).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
            t2.set("packages", package_manager.packages).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
            t.raw_insert(i as i64 + 1, t2).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        }
        globals.set("packageManagers", t).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        Ok(())
	}
}
