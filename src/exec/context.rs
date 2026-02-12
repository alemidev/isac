use std::str::FromStr;


pub struct Context {
	pub cfg: crate::conf::Isac,
	pub config_from: std::path::PathBuf,
	pub config_to: std::path::PathBuf,

	pub installer: Box<dyn crate::tool::Installer>,
	pub services: Box<dyn crate::tool::ServiceManager>,
}

impl Context {
	// TODO should gather system info and construct a proper context
	pub fn new(cfg: crate::conf::Isac, root: std::path::PathBuf) -> Self {
		let mut config_from = root.clone();
		config_from.push(&cfg.system.configs);
		let mut config_to = std::path::PathBuf::from_str(&cfg.system.root).expect("infallible");
		config_to.push(&cfg.system.configs);
		Self {
			cfg,
			config_from,
			config_to,
			installer: Box::new(crate::tool::installer::Pacman),
			services: Box::new(crate::tool::services::Systemd),
		}
	}

	pub fn restore(self, filter: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
		for (name, module) in self.cfg.r#mod.iter() {
			if !filter.as_ref().map(|m| m == name).unwrap_or(true) {
				continue;
			}
			println!(" > restoring '{name}'");
			module.restore(&self)?;
		}

		Ok(())
	}

	pub fn snapshot(self, filter: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
		for (name, module) in self.cfg.r#mod.iter() {
			if !filter.as_ref().map(|m| m == name).unwrap_or(true) {
				continue;
			}
			println!(" > snapshotting '{name}'");
			module.snapshot(&self)?;
		}

		Ok(())
	}
}
