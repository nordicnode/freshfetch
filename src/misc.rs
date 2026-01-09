use crate::term_size;
use crate::mlua;

use crate::errors;

use mlua::prelude::*;

use crate::Inject;

pub(crate) struct Terminal {
	pub width: i32,
	pub height: i32,
}

impl Terminal {
	pub fn new() -> Self {
		let (w, h) = term_size::dimensions().expect("Failed to get terminal dimensions.");
		Terminal {
			width: w as i32,
			height: h as i32,
		}
	}
}

impl Inject for Terminal {
	fn inject(&self, lua: &mut Lua) -> errors::Result<()> {
		let globals = lua.globals();
		let t = lua.create_table().map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        t.set("width", self.width).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        t.set("height", self.height).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        globals.set("terminal", t).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        Ok(())
	}
}

