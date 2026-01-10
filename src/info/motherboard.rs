use crate::mlua;
use crate::regex;

use crate::errors;
use super::kernel;

use std::fs::{ read_to_string };
use std::path::{ Path };
use std::process::{ Command };

use regex::{ Regex };
use mlua::prelude::*;

use crate::{ Inject };
use kernel::{ Kernel };

use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct Motherboard {
    pub name: String,
    pub vendor: String,
    pub revision: String,
}

impl Motherboard {
    pub(crate) fn new(k: &Kernel) -> Option<Self> {
        match k.name.as_str() {
            "Linux" => {
                let sys_devices_virtual_dmi_id = Path::new("/sys/devices/virtual/dmi/id");
                // Android
                if Path::new("/system/app").is_dir()
                && Path::new("/system/priv-app").is_dir() {
                    let product_board = Command::new("getprop")
                        .arg("ro.product.board")
                        .output()
                        .ok()
                        .and_then(|o| String::from_utf8(o.stdout).ok())
                        .unwrap_or_default()
                        .trim()
                        .to_string();
                    let product_model = Command::new("getprop")
                        .arg("ro.product.model")
                        .output()
                        .ok()
                        .and_then(|o| String::from_utf8(o.stdout).ok())
                        .unwrap_or_default()
                        .trim()
                        .to_string();
                    
                    if !product_board.is_empty() {
                        return Some(Motherboard {
                            name: product_board,
                            vendor: String::from("Android"),
                            revision: product_model,
                        });
                    }
                    None
                // Standard
                } else if sys_devices_virtual_dmi_id.exists() && (
                   sys_devices_virtual_dmi_id.join("board_name").is_file()
                || sys_devices_virtual_dmi_id.join("board_vendor").is_file()
                || sys_devices_virtual_dmi_id.join("board_version").is_file()) {
                    Some(Motherboard {
                        name: read_to_string(sys_devices_virtual_dmi_id.join("board_name"))
                            .unwrap_or_default()
                            .replace("\n", " ")
                            .trim()
                            .to_string(),
                        vendor: read_to_string(sys_devices_virtual_dmi_id.join("board_vendor"))
                            .unwrap_or_default()
                            .replace("\n", " ")
                            .trim()
                            .to_string(),
                        revision: read_to_string(sys_devices_virtual_dmi_id.join("board_version"))
                            .unwrap_or_default()
                            .replace("\n", " ")
                            .trim()
                            .to_string(),
                    })
                } else {
                    // TODO: Fallback? I only have 2 computers and the previous
                    // code works on both, but thats because they're both Arch
                    // Linux. Idk about stuff like OpenBSD or whatever.
                    None
                }
            }
            "Mac OS X"|"macOS" => {
                // TODO: It looks to me like something from the output of
                // `sysctl` can be used to get info of this nature. Not sure
                // personally, and I don't own a Mac to test on.
                None
            }
            "BSD"|"MINIX" => {
                // TODO: Idk BSD or MINUX, but I think this would be something
                // with `sysctl`.
                None
            }
            "Windows" /*(ew)*/ => {
                // TODO: Get someone to test this.
                let try_wmic = Command::new("wmic")
                    .arg("baseboard")
                    .arg("get")
                    .arg("product,manufacturer")
                    .output()
                    .ok()
                    .and_then(|o| String::from_utf8(o.stdout).ok());
                try_wmic.and_then(|wmic| {
                    let lines = wmic.split("\n").collect::<Vec<&str>>();
                    if lines.len() >= 2 {
                        let regex = Regex::new(r#"(\S+)\s+(\S+)"#).unwrap();
                        regex.captures(lines[1]).map(|caps| Motherboard {
                            name: String::from(caps.get(1).unwrap().as_str()),
                            vendor: String::from(caps.get(2).unwrap().as_str()),
                            revision: String::new(),
                        })
                    } else {
                        None
                    }
                })
            }
            _ => None,
        }
    }
}

impl Inject for Motherboard {
    fn inject(&self, lua: &mut Lua) -> errors::Result<()> {
        let globals = lua.globals();
        
        let t = lua.create_table().map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        t.set("name", self.name.clone()).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        t.set("vendor", self.vendor.clone()).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        t.set("revision", self.revision.clone()).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        globals.set("motherboard", t).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        Ok(())
    }
}

