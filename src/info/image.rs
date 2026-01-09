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
    pub fn inject(lua: &mut Lua) {
        let globals = lua.globals();
        
        let image_fn = lua.create_function(|_, path: String| {
            let config = Config {
                restore_cursor: true,
                ..Default::default()
            };
            
            // Note: This prints directly to stdout. 
            // In freshfetch, we'll need to figure out how to integrate this 
            // with the string-based rendering if we want perfect positioning.
            // For now, it will print where the cursor is when called.
            match print_from_file(Path::new(&path), &config) {
                Ok(_) => Ok(()),
                Err(e) => {
                    errors::handle(&format!("Failed to render image: {}", e));
                    Ok(())
                }
            }
        }).unwrap();
        
        globals.set("image", image_fn).unwrap();
    }
}
