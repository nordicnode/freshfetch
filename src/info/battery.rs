use crate::mlua;
use crate::errors;

use std::fs;
use std::path::Path;

use mlua::prelude::*;
use crate::Inject;

/// Battery information for laptops
use serde::Serialize;

/// Battery information for laptops
#[derive(Clone, Debug, Serialize)]
pub(crate) struct Battery {
    pub capacity: u8,
    pub status: String,
}

impl Battery {
    pub fn new() -> Option<Self> {
        let power_supply = Path::new("/sys/class/power_supply");
        
        if !power_supply.exists() {
            return None;
        }
        
        // Find first battery (BAT0, BAT1, etc.)
        let entries = fs::read_dir(power_supply).ok()?;
        
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            
            if name_str.starts_with("BAT") {
                let bat_path = entry.path();
                
                // Read capacity
                let capacity_path = bat_path.join("capacity");
                let capacity = fs::read_to_string(&capacity_path)
                    .ok()?
                    .trim()
                    .parse::<u8>()
                    .ok()?;
                
                // Read status
                let status_path = bat_path.join("status");
                let status = fs::read_to_string(&status_path)
                    .ok()?
                    .trim()
                    .to_string();
                
                return Some(Battery { capacity, status });
            }
        }
        
        None
    }
}

impl Inject for Battery {
    fn inject(&self, lua: &mut Lua) -> errors::Result<()> {
        let globals = lua.globals();
        
        let t = lua.create_table().map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        t.set("capacity", self.capacity).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        t.set("status", self.status.as_str()).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        globals.set("battery", t).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        
        Ok(())
    }
}
