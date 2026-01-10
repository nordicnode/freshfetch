use crate::mlua;
use crate::errors;
use super::kernel::Kernel;

use std::fs;
use std::path::Path;

use mlua::prelude::*;
use crate::Inject;
use serde::Serialize;

/// Monitor information parsed from EDID
#[derive(Clone, Debug, Serialize)]
pub(crate) struct Monitor {
    pub name: String,
    pub width_mm: Option<u32>,
    pub height_mm: Option<u32>,
}

/// Collection of detected monitors
#[derive(Clone, Debug, Serialize)]
pub(crate) struct Monitors {
    pub monitors: Vec<Monitor>,
}

impl Monitors {
    pub fn new(k: &Kernel) -> Option<Self> {
        if k.name != "Linux" {
            return None;
        }

        let drm_path = Path::new("/sys/class/drm");
        if !drm_path.exists() {
            return None;
        }

        let mut monitors = Vec::new();

        if let Ok(entries) = fs::read_dir(drm_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                let edid_path = path.join("edid");
                
                if edid_path.exists() {
                    if let Ok(edid_bytes) = fs::read(edid_path) {
                        if edid_bytes.len() >= 128 {
                            if let Some(monitor) = Self::parse_edid(&edid_bytes) {
                                monitors.push(monitor);
                            }
                        }
                    }
                }
            }
        }

        if monitors.is_empty() {
            None
        } else {
            Some(Monitors { monitors })
        }
    }

    fn parse_edid(edid: &[u8]) -> Option<Monitor> {
        // Basic EDID parsing
        // Monitor Name Descriptor is 0x00 0x00 0x00 0xFC 0x00
        
        let mut name = None;

        // Check the 4 descriptors (offsets 54, 72, 90, 108)
        for offset in [54, 72, 90, 108] {
            if edid.len() < offset + 18 {
                break;
            }
            let descriptor = &edid[offset..offset + 18];
            if descriptor.starts_with(&[0x00, 0x00, 0x00, 0xFC, 0x00]) {
                // Monitor names are up to 13 bytes, often ended with 0x0A (LF)
                let name_bytes = &descriptor[5..18];
                let name_str = String::from_utf8_lossy(name_bytes)
                    .trim_end_matches(|c: char| c == '\n' || c == '\r' || c == ' ' || c == '\0')
                    .to_string();
                if !name_str.is_empty() {
                    name = Some(name_str);
                }
            }
        }

        // Get physical dimensions (cm to mm)
        let width_mm = (edid[21] as u32) * 10;
        let height_mm = (edid[22] as u32) * 10;

        let name = name.unwrap_or_else(|| "Unknown Monitor".to_string());

        Some(Monitor {
            name,
            width_mm: if width_mm > 0 { Some(width_mm) } else { None },
            height_mm: if height_mm > 0 { Some(height_mm) } else { None },
        })
    }
}

impl Inject for Monitors {
    fn inject(&self, lua: &mut Lua) -> errors::Result<()> {
        let globals = lua.globals();
        
        let t = lua.create_table().map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        
        let monitors_table = lua.create_table().map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        for (i, monitor) in self.monitors.iter().enumerate() {
            let m_t = lua.create_table().map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
            m_t.set("name", monitor.name.as_str()).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
            if let Some(w) = monitor.width_mm {
                m_t.set("width_mm", w).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
            }
            if let Some(h) = monitor.height_mm {
                m_t.set("height_mm", h).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
            }
            monitors_table.raw_set((i + 1) as i64, m_t).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        }
        
        t.set("monitors", monitors_table).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        t.set("count", self.monitors.len()).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        
        globals.set("monitors", t).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        Ok(())
    }
}
