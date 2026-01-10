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
    pub cycle_count: Option<u32>,
    pub health: Option<u8>,  // Percentage of original capacity
    pub power_draw: Option<f32>,  // Watts
}

impl Battery {
    pub fn new() -> Option<Self> {
        let power_supply = Path::new("/sys/class/power_supply");
        
        if !power_supply.exists() {
            return None;
        }
        
        let entries = fs::read_dir(power_supply).ok()?;
        
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            
            if name_str.starts_with("BAT") {
                let bat_path = entry.path();
                
                // Read capacity
                let capacity = fs::read_to_string(bat_path.join("capacity"))
                    .ok()?
                    .trim()
                    .parse::<u8>()
                    .ok()?;
                
                // Read status
                let status = fs::read_to_string(bat_path.join("status"))
                    .ok()?
                    .trim()
                    .to_string();
                
                // Read cycle count (optional)
                let cycle_count = fs::read_to_string(bat_path.join("cycle_count"))
                    .ok()
                    .and_then(|s| s.trim().parse::<u32>().ok());
                
                // Calculate health from energy_full vs energy_full_design
                let health = Self::calculate_health(&bat_path);
                
                // Calculate power draw (optional)
                let power_draw = Self::calculate_power_draw(&bat_path);
                
                return Some(Battery { capacity, status, cycle_count, health, power_draw });
            }
        }
        
        None
    }
    
    fn calculate_health(bat_path: &std::path::PathBuf) -> Option<u8> {
        let energy_full = fs::read_to_string(bat_path.join("energy_full"))
            .ok()
            .and_then(|s| s.trim().parse::<u64>().ok())?;
        let energy_full_design = fs::read_to_string(bat_path.join("energy_full_design"))
            .ok()
            .and_then(|s| s.trim().parse::<u64>().ok())?;
        
        if energy_full_design > 0 {
            Some(((energy_full as f64 / energy_full_design as f64) * 100.0) as u8)
        } else {
            None
        }
    }
    
    fn calculate_power_draw(bat_path: &std::path::PathBuf) -> Option<f32> {
        // power_now is in microwatts
        let power_now = fs::read_to_string(bat_path.join("power_now"))
            .ok()
            .and_then(|s| s.trim().parse::<u64>().ok())?;
        
        Some(power_now as f32 / 1_000_000.0)  // Convert to watts
    }
}

impl Inject for Battery {
    fn inject(&self, lua: &mut Lua) -> errors::Result<()> {
        let globals = lua.globals();
        
        let t = lua.create_table().map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        t.set("capacity", self.capacity).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        t.set("status", self.status.as_str()).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        
        if let Some(cycles) = self.cycle_count {
            t.set("cycles", cycles).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        }
        if let Some(health) = self.health {
            t.set("health", health).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        }
        if let Some(power) = self.power_draw {
            t.set("power", power).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        }
        
        globals.set("battery", t).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        Ok(())
    }
}
