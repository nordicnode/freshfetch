use crate::mlua;

use crate::errors;

use mlua::prelude::*;

use crate::{ Inject };

use std::fs::{ read_to_string };
use std::env::{ var };

use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub(crate) struct Context {
	pub user: String,
	pub host: String,
}

impl Context {
	pub fn new() -> Option<Self> {
		Some(Context {
			user: match var("USER") {
				Ok(v) => v,
				Err(_) => return None,
			},
			host: match read_to_string("/etc/hostname") {
				Ok(v) => v,
				Err(_) => return None,
			}
		})
	} 
}

impl Inject for Context {
	fn inject(&self, lua: &mut Lua) -> errors::Result<()> {
		let globals = lua.globals();
        
		let t = lua.create_table().map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        t.set("user", self.user.as_str()).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        t.set("host", self.host.as_str()).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        globals.set("context", t).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        Ok(())
	}
}
