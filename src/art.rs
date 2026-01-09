use crate::mlua;

use crate::assets::ascii_art;
use crate::errors;
use crate::info;
use crate::assets;
use info::distro;

use std::fs;
use std::path::PathBuf;

use mlua::prelude::*;

use crate::{ Inject, Arguments };
use info::{ Info };
use distro::{ DistroColors };
use assets::{ ANSI, PRINT };

pub(crate) struct Art {
	inner: String,
	width: i32,
	height: i32,
	logo: bool,
}

impl Art {
	pub fn new(info: &mut Info, arguments: &Arguments) -> errors::Result<Self> {
		let mut to_return = Art {
			inner: String::new(),
			width: 0,
			height: 0,
			logo: false,
		};

		// Get inner & distro colors.
		{
			match arguments.ascii_distro.clone() {
				None => {
					let art = dirs::home_dir()
						.unwrap_or_else(|| PathBuf::from("."))
						.join(".config/freshfetch/art.lua");
					if art.exists() {
						let file = fs::read_to_string(&art).map_err(|e| {
                            errors::FreshfetchError::Io(art.to_string_lossy().into_owned(), e.to_string())
                        })?;
                        
                        to_return.inner = {
                            let ctx = Lua::new();
                            ctx.load(PRINT).exec().map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
                            ctx.load(ANSI).exec().map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
                            ctx.load(&file).exec().map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
                            
                            let result = ctx.globals().get::<&str, String>("__freshfetch__").map_err(|e| {
                                errors::FreshfetchError::Lua(e.to_string())
                            })?;
                            result
                        };
					} else {
						let got = ascii_art::get(&info.distro.short_name);
						to_return.inner = String::from(got.0);
						info.distro.colors = DistroColors::from(got.1);
					}
				}
				Some(a) => {
					let got = ascii_art::get(&a);
					to_return.inner = String::from(got.0);
					info.distro.colors = DistroColors::from(got.1);
				}
			}
		}

		// Get width and height
		{
			let (w, h) = crate::utils::get_dimensions(&to_return.inner);
			to_return.width = w;
			to_return.height = h;
		}

		to_return.logo = arguments.logo;

		Ok(to_return)
	}
}

impl Inject for Art {
	fn inject(&self, lua: &mut Lua) -> errors::Result<()> {
		let globals = lua.globals();

		globals.set("art", self.inner.as_str()).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
		globals.set("artWidth", self.width).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
		globals.set("artHeight", self.height).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
		globals.set("logo", self.logo).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        Ok(())
	}
}
