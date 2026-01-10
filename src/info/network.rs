use crate::mlua;
use crate::errors;

use std::fs;
use std::process::Command;

use mlua::prelude::*;
use crate::Inject;

/// Network interface information
use serde::Serialize;

/// Network interface information
#[derive(Clone, Debug, Serialize)]
pub(crate) struct Network {
    pub interface: String,
    pub ip: String,
}

impl Network {
    pub fn new() -> Option<Self> {
        // Try to find first non-loopback interface with an IP
        let net_dir = std::path::Path::new("/sys/class/net");
        
        if !net_dir.exists() {
            return None;
        }
        
        let entries = fs::read_dir(net_dir).ok()?;
        
        for entry in entries.flatten() {
            let iface = entry.file_name().to_string_lossy().to_string();
            
            // Skip loopback
            if iface == "lo" {
                continue;
            }
            
            // Check if interface is up
            let operstate = entry.path().join("operstate");
            if let Ok(state) = fs::read_to_string(&operstate) {
                if state.trim() != "up" {
                    continue;
                }
            }
            
            // Get IP address using ip command
            if let Some(ip) = Self::get_ip_for_interface(&iface) {
                return Some(Network { interface: iface, ip });
            }
        }
        
        None
    }
    
    fn get_ip_for_interface(iface: &str) -> Option<String> {
        let output = Command::new("ip")
            .args(["addr", "show", iface])
            .output()
            .ok()?;
        
        let stdout = String::from_utf8(output.stdout).ok()?;
        
        // Parse for inet (IPv4) address
        for line in stdout.lines() {
            let line = line.trim();
            if line.starts_with("inet ") {
                // Format: "inet 192.168.1.100/24 ..."
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    // Remove subnet mask
                    let ip = parts[1].split('/').next()?;
                    return Some(ip.to_string());
                }
            }
        }
        
        None
    }
}

impl Inject for Network {
    fn inject(&self, lua: &mut Lua) -> errors::Result<()> {
        let globals = lua.globals();
        
        let t = lua.create_table().map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        t.set("interface", self.interface.as_str()).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        t.set("ip", self.ip.as_str()).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        globals.set("network", t).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        
        Ok(())
    }
}
