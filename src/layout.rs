use crate::mlua;
use crate::errors;

use crate::misc;
use crate::art;
use crate::info;

use mlua::prelude::*;

use crate::{ Inject, Arguments };
use misc::{ Terminal };
use art::{ Art };
use info::{ Info };

pub(crate) struct Layout {
	pub art: Art,
	pub info: Info,
	pub terminal: Terminal,
}

impl Layout {
	pub fn new(args: &Arguments) -> errors::Result<Self> {
		let mut info = Info::new()?;
		let art = Art::new(&mut info, args)?;
		let terminal = Terminal::new();
		Ok(Layout {
			art,
			info,
			terminal,
		})
	}
}

impl Inject for Layout {
	fn prep(&mut self) -> errors::Result<()> {
		self.info.prep()?;
		self.art.prep()?;
		self.terminal.prep()?;
        Ok(())
	}
	fn inject(&self, lua: &mut Lua) -> errors::Result<()> {
		self.art.inject(lua)?;
		self.terminal.inject(lua)?;
		self.info.inject(lua)?;
        Ok(())
	}
}
