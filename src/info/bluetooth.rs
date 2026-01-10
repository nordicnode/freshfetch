use crate::mlua;
use crate::errors;

use std::fs;
use std::path::Path;
use std::process::Command;

use mlua::prelude::*;
use crate::Inject;
use serde::Serialize;

/// Bluetooth device information
#[derive(Clone, Debug, Serialize)]
pub(crate) struct BluetoothDevice {
    pub name: String,
    pub mac: String,
    pub connected: bool,
}

/// Bluetooth adapter and devices
#[derive(Clone, Debug, Serialize)]
pub(crate) struct Bluetooth {
    pub adapter: Option<String>,
    pub devices: Vec<BluetoothDevice>,
}

impl Bluetooth {
    pub fn new() -> Option<Self> {
        let bt_path = Path::new("/sys/class/bluetooth");
        
        if !bt_path.exists() {
            return None;
        }
        
        // Get adapter name
        let adapter = fs::read_dir(bt_path)
            .ok()?
            .filter_map(|e| e.ok())
            .next()
            .map(|e| e.file_name().to_string_lossy().to_string());
        
        if adapter.is_none() {
            return None;
        }
        
        // Try to get paired devices via bluetoothctl
        let devices = Self::get_paired_devices();
        
        Some(Bluetooth { adapter, devices })
    }
    
    fn get_paired_devices() -> Vec<BluetoothDevice> {
        let output = Command::new("bluetoothctl")
            .args(["devices", "Paired"])
            .output()
            .ok();
        
        let Some(output) = output else {
            return Vec::new();
        };
        
        let stdout = String::from_utf8(output.stdout).unwrap_or_default();
        let mut devices = Vec::new();
        
        for line in stdout.lines() {
            // Format: "Device XX:XX:XX:XX:XX:XX Device Name"
            if line.starts_with("Device ") {
                let parts: Vec<&str> = line.splitn(3, ' ').collect();
                if parts.len() >= 3 {
                    let mac = parts[1].to_string();
                    let name = parts[2].to_string();
                    let connected = Self::is_device_connected(&mac);
                    devices.push(BluetoothDevice { name, mac, connected });
                }
            }
        }
        
        devices
    }
    
    fn is_device_connected(mac: &str) -> bool {
        let output = Command::new("bluetoothctl")
            .args(["info", mac])
            .output()
            .ok();
        
        output
            .map(|o| String::from_utf8(o.stdout).unwrap_or_default().contains("Connected: yes"))
            .unwrap_or(false)
    }
}

impl Inject for Bluetooth {
    fn inject(&self, lua: &mut Lua) -> errors::Result<()> {
        let globals = lua.globals();
        
        let t = lua.create_table().map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        
        if let Some(ref adapter) = self.adapter {
            t.set("adapter", adapter.as_str()).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        }
        
        let devices_table = lua.create_table().map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        for (i, device) in self.devices.iter().enumerate() {
            let dev_t = lua.create_table().map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
            dev_t.set("name", device.name.as_str()).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
            dev_t.set("mac", device.mac.as_str()).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
            dev_t.set("connected", device.connected).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
            devices_table.raw_set((i + 1) as i64, dev_t).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        }
        t.set("devices", devices_table).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        t.set("count", self.devices.len()).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        
        globals.set("bluetooth", t).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        Ok(())
    }
}
