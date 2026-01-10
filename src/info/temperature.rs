use crate::mlua;
use crate::sysinfo;
use crate::errors;

use mlua::prelude::*;
use sysinfo::Components;

use crate::Inject;
use serde::Serialize;

/// Temperature sensor reading
#[derive(Clone, Debug, Serialize)]
pub(crate) struct TempSensor {
    pub label: String,
    pub temp: f32,
    pub max: Option<f32>,
    pub critical: Option<f32>,
}

/// System temperature information
#[derive(Clone, Debug, Serialize)]
pub(crate) struct Temperature {
    pub sensors: Vec<TempSensor>,
}

impl Temperature {
    pub fn new() -> Option<Self> {
        let components = Components::new_with_refreshed_list();
        
        if components.is_empty() {
            return None;
        }
        
        let sensors: Vec<TempSensor> = components
            .iter()
            .map(|c| TempSensor {
                label: c.label().to_string(),
                temp: c.temperature(),
                max: Some(c.max()),
                critical: c.critical(),
            })
            .collect();
        
        if sensors.is_empty() {
            None
        } else {
            Some(Temperature { sensors })
        }
    }
    
    /// Get the highest temperature reading
    pub fn max_temp(&self) -> Option<f32> {
        self.sensors.iter().map(|s| s.temp).max_by(|a, b| a.partial_cmp(b).unwrap())
    }
    
    /// Get CPU temperature (first sensor with "cpu" in label)
    pub fn cpu_temp(&self) -> Option<f32> {
        self.sensors
            .iter()
            .find(|s| s.label.to_lowercase().contains("cpu") || s.label.to_lowercase().contains("core"))
            .map(|s| s.temp)
    }
}

impl Inject for Temperature {
    fn inject(&self, lua: &mut Lua) -> errors::Result<()> {
        let globals = lua.globals();
        
        let t = lua.create_table().map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        
        // Create sensors array
        let sensors_table = lua.create_table().map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        for (i, sensor) in self.sensors.iter().enumerate() {
            let sensor_t = lua.create_table().map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
            sensor_t.set("label", sensor.label.as_str()).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
            sensor_t.set("temp", sensor.temp).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
            if let Some(max) = sensor.max {
                sensor_t.set("max", max).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
            }
            if let Some(critical) = sensor.critical {
                sensor_t.set("critical", critical).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
            }
            sensors_table.raw_set((i + 1) as i64, sensor_t).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        }
        t.set("sensors", sensors_table).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        
        // Add convenience fields
        if let Some(cpu) = self.cpu_temp() {
            t.set("cpu", cpu).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        }
        if let Some(max) = self.max_temp() {
            t.set("max", max).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        }
        
        globals.set("temperature", t).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        Ok(())
    }
}
