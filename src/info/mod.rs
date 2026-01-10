use crate::mlua;

use crate::errors;
use crate::assets;
use crate::assets::defaults;
pub(crate) mod kernel;
pub(crate) mod context;
pub(crate) mod distro;
pub(crate) mod uptime;
pub(crate) mod package_managers;
pub(crate) mod shell;
pub(crate) mod resolution;
pub(crate) mod wm;
pub(crate) mod de;
pub(crate) mod utils;
pub(crate) mod cpu;
pub(crate) mod gpu;
pub(crate) mod memory;
pub(crate) mod motherboard;
pub(crate) mod host;
pub(crate) mod image;
pub(crate) mod battery;
pub(crate) mod disk;
pub(crate) mod network;
pub(crate) mod temperature;
pub(crate) mod bluetooth;

use std::fs;
use std::path::PathBuf;

use mlua::prelude::*;

use crate::{ Inject };
use assets::{ ANSI, PRINT };
use defaults::{ INFO };
use utils::{ get_system };
use kernel::{ Kernel };
use context::{ Context };
use distro::{ Distro };
use uptime::{ Uptime };
use package_managers::{ PackageManagers };
use shell::{ Shell };
use resolution::{ Resolution };
use wm::{ Wm };
use de::{ De };
use cpu::{ Cpu };
use gpu::{ Gpus };
use memory::{ Memory };
use motherboard::{ Motherboard };
use host::Host;
use battery::Battery;
use disk::Disk;
use network::Network;
use temperature::Temperature;
use bluetooth::Bluetooth;

use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct Info {
	#[serde(skip)]
	ctx: Lua,
	#[serde(skip)]
	rendered: String,
	#[serde(skip)]
	width: i32,
	#[serde(skip)]
	height: i32,
	pub context: Option<Context>,
	pub distro: Distro,
	pub kernel: Kernel,
	pub uptime: Uptime,
	pub package_managers: PackageManagers,
	pub shell: Shell,
	pub resolution: Option<Resolution>,
	pub de: Option<De>,
	pub wm: Option<Wm>,
	pub cpu: Option<Cpu>,
	pub gpu: Option<Gpus>,
	pub memory: Memory,
	pub motherboard: Option<Motherboard>,
	pub host: Option<Host>,
	pub battery: Option<Battery>,
	pub disk: Option<Disk>,
	pub network: Option<Network>,
	pub temperature: Option<Temperature>,
	pub bluetooth: Option<Bluetooth>,
}

impl Info {
	pub fn new() -> errors::Result<Self> {
		{
			let mut system = get_system();
			system.refresh_cpu_usage();
			system.refresh_memory();
		}
		
		// Sequential: Kernel must be first since others depend on it
		let kernel = Kernel::new()?;
		let context = Context::new();
		let distro = Distro::new(&kernel);
		let uptime = Uptime::new(&kernel)?;
		let package_managers = PackageManagers::new(&kernel)?;
		let shell = Shell::new(&kernel)?;
		
		// Parallel: Independent info gathering using rayon
		// Use nested joins in pairs for parallel execution
		let ((resolution, de), (wm, cpu)) = rayon::join(
			|| rayon::join(
				|| Resolution::new(&kernel),
				|| De::new(&kernel, &distro),
			),
			|| rayon::join(
				|| Wm::new(&kernel),
				|| Cpu::new(&kernel),
			),
		);
		
		let ((gpu, motherboard), (host, battery)) = rayon::join(
			|| rayon::join(
				|| Gpus::new(&kernel),
				|| Motherboard::new(&kernel),
			),
			|| rayon::join(
				|| Host::new(&kernel),
				Battery::new,
			),
		);
		
		let ((disk, network), (temperature, bluetooth)) = rayon::join(
			|| rayon::join(
				Disk::new,
				Network::new,
			),
			|| rayon::join(
				Temperature::new,
				Bluetooth::new,
			),
		);
		
		let memory = Memory::new();
		
		Ok(Info {
			ctx: Lua::new(),
			rendered: String::new(),
			width: 0,
			height: 0,
			context,
			distro,
			kernel,
			uptime,
			package_managers,
			shell,
			resolution,
			de,
			wm,
			cpu,
			gpu,
			memory,
			motherboard,
			host,
			battery,
			disk,
			network,
			temperature,
			bluetooth,
		})
	}
	pub fn render(&mut self) -> errors::Result<()> {
		self.ctx.load(PRINT).exec().map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
		self.ctx.load(ANSI).exec().map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;

		let info = dirs::home_dir()
			.unwrap_or_else(|| PathBuf::from("."))
			.join(".config/freshfetch/info.lua");
		if info.exists() {
			let file = fs::read_to_string(&info).map_err(|e| {
                errors::FreshfetchError::Io(info.to_string_lossy().into_owned(), e.to_string())
            })?;
            
            self.ctx.load(&file).exec().map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
            
            self.rendered = self.ctx.globals().get::<&str, String>("__freshfetch__").map_err(|e| {
                errors::FreshfetchError::Lua(e.to_string())
            })?;
		} else {
			self.ctx.load(INFO).exec().map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
            
            self.rendered = self.ctx.globals().get::<&str, String>("__freshfetch__").map_err(|e| {
                errors::FreshfetchError::Lua(e.to_string())
            })?;
		}
        Ok(())
	}
}

impl Inject for Info {
	fn prep(&mut self) -> errors::Result<()> {
		image::ImageManager::inject(&mut self.ctx)?;
		if let Some(v) = &self.context { v.inject(&mut self.ctx)?; }
		self.kernel.inject(&mut self.ctx)?;
		self.distro.inject(&mut self.ctx)?;
		self.uptime.inject(&mut self.ctx)?;
		self.package_managers.inject(&mut self.ctx)?;
		self.shell.inject(&mut self.ctx)?;
		if let Some(v) = &self.resolution { v.inject(&mut self.ctx)?; }
		if let Some(v) = &self.wm { v.inject(&mut self.ctx)?; }
		if let Some(v) = &self.de { v.inject(&mut self.ctx)?; }
		if let Some(v) = &self.cpu { v.inject(&mut self.ctx)?; }
		if let Some(v) = &self.gpu { v.inject(&mut self.ctx)?; }
		self.memory.inject(&mut self.ctx)?;
        if let Some(v) = &self.motherboard { v.inject(&mut self.ctx)?; }
		if let Some(v) = &self.host { v.inject(&mut self.ctx)?; }
		if let Some(v) = &self.battery { v.inject(&mut self.ctx)?; }
		if let Some(v) = &self.disk { v.inject(&mut self.ctx)?; }
		if let Some(v) = &self.network { v.inject(&mut self.ctx)?; }
		self.render()?;
		{
			let (w, h) = crate::utils::get_dimensions(&self.rendered);
			self.width = w;
			self.height = h;
		}
        Ok(())
	}
	fn inject(&self, lua: &mut Lua) -> errors::Result<()> {
		let globals = lua.globals();

		globals.set("info", self.rendered.as_str()).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
		globals.set("infoWidth", self.width).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
		globals.set("infoHeight", self.height).map_err(|e| errors::FreshfetchError::Lua(e.to_string()))?;
        Ok(())
	}
}
