use crate::mlua;
use crate::sysinfo;
use crate::errors;

use mlua::prelude::*;
use sysinfo::Disks;

use crate::Inject;

/// Disk usage information
#[derive(Clone, Debug)]
pub(crate) struct Disk {
    pub name: String,
    pub mount_point: String,
    pub total: u64,
    pub used: u64,
    pub fs_type: String,
}

impl Disk {
    pub fn new() -> Option<Self> {
        let disks = Disks::new_with_refreshed_list();
        
        // Find root partition or first disk
        for disk in disks.iter() {
            let mount = disk.mount_point().to_string_lossy().to_string();
            
            // Prefer root partition
            if mount == "/" {
                let total = disk.total_space();
                let available = disk.available_space();
                let used = total.saturating_sub(available);
                
                return Some(Disk {
                    name: disk.name().to_string_lossy().to_string(),
                    mount_point: mount,
                    total,
                    used,
                    fs_type: disk.file_system().to_string_lossy().to_string(),
                });
            }
        }
        
        // Fallback to first disk if no root found
        disks.iter().next().map(|disk| {
            let total = disk.total_space();
            let available = disk.available_space();
            let used = total.saturating_sub(available);
            
            Disk {
                name: disk.name().to_string_lossy().to_string(),
                mount_point: disk.mount_point().to_string_lossy().to_string(),
                total,
                used,
                fs_type: disk.file_system().to_string_lossy().to_string(),
            }
        })
    }
}

impl Inject for Disk {
    fn inject(&self, lua: &mut Lua) -> errors::Result<()> {
        let globals = lua.globals();
        
        let t = lua.create_table().map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        t.set("name", self.name.as_str()).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        t.set("mount_point", self.mount_point.as_str()).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        // Convert to GB for easier display
        t.set("total_gb", (self.total as f64 / 1_073_741_824.0) as u64).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        t.set("used_gb", (self.used as f64 / 1_073_741_824.0) as u64).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        t.set("fs_type", self.fs_type.as_str()).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        globals.set("disk", t).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        
        Ok(())
    }
}
