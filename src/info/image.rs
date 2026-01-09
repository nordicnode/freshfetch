use crate::mlua;
use mlua::prelude::*;
use viuer::{Config, print_from_file};
use std::path::Path;
use crate::errors;

/// Manages terminal image rendering capabilities.
pub(crate) struct ImageManager;

impl ImageManager {
    /// Injects the `image(path)` function into the provided Lua environment.
    /// This allows layouts to render images directly in the terminal.
    pub fn inject(lua: &mut Lua) -> errors::Result<()> {
        let globals = lua.globals();
        
        let image_fn = lua.create_function(|_, path: String| {
            let config = Config {
                restore_cursor: true,
                ..Default::default()
            };
            
            match print_from_file(Path::new(&path), &config) {
                Ok(_) => Ok(()),
                Err(e) => {
                    // We return an error to Lua, which freshfetch will eventually catch
                    Err(mlua::Error::RuntimeError(format!("Failed to render image: {}", e)))
                }
            }
        }).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        
        globals.set("image", image_fn).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        Ok(())
    }
}
