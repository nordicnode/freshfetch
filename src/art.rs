use crate::regex;
use crate::mlua;

use crate::assets::ascii_art;
use crate::errors;
use crate::info;
use crate::assets;
use info::distro;

use std::fs;
use std::env;
use std::path::{ Path, PathBuf };

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
	pub fn new(info: &mut Info, arguments: &Arguments) -> Self {
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
						match fs::read_to_string(art) {
							Ok(file) => to_return.inner = {
								let ctx = Lua::new();
								match ctx.load(PRINT).exec() {
									Ok(_) => (),
									Err(e) => { errors::handle(&format!("{}{}", errors::LUA, e)); panic!(); }
								}
								match ctx.load(ANSI).exec() {
									Ok(_) => (),
									Err(e) => { errors::handle(&format!("{}{}", errors::LUA, e)); panic!(); }
								}
								match ctx.load(&file).exec() {
									Ok(_) => (),
									Err(e) => { errors::handle(&format!("{}{}", errors::LUA, e)); panic!(); }
								}
								let value = ctx.globals().get::<&str, String>("__freshfetch__");
								match value {
									Ok(v) => v,
									Err(e) => { errors::handle(&format!("{}{}", errors::LUA, e)); panic!(); }
								}
							},
							Err(e) => {
								errors::handle(&format!("{}{file}{}{err}",
									errors::io::READ.0,
									errors::io::READ.1,
									file = "~/.config/freshfetch/art.lua",
									err = e));
								panic!();
							}
						}
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

		to_return
	}
}

impl Inject for Art {
	fn inject(&self, lua: &mut Lua) {
		let globals = lua.globals();

		match globals.set("art", self.inner.as_str()) {
			Ok(_) => (),
			Err(e) => errors::handle(&format!("{}{}", errors::LUA, e)),
		}
		match globals.set("artWidth", self.width) {
			Ok(_) => (),
			Err(e) => errors::handle(&format!("{}{}", errors::LUA, e)),
		}
		match globals.set("artHeight", self.height) {
			Ok(_) => (),
			Err(e) => errors::handle(&format!("{}{}", errors::LUA, e)),
		}
		match globals.set("logo", self.logo) {
			Ok(_) => (),
			Err(e) => errors::handle(&format!("{}{}", errors::LUA, e)),
		}
	}
}
