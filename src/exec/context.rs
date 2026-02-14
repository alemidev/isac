
#[derive(Debug)]
pub struct Context {
	pub cfg: crate::conf::Isac,
	pub root: std::path::PathBuf,
	pub installer: Box<dyn crate::tool::Installer>,
	pub services: Box<dyn crate::tool::ServiceManager>,
	pub users: Box<dyn crate::tool::UserManager>,
}

impl Context {
	// TODO should gather system info and construct a proper context
	pub fn new(cfg: crate::conf::Isac, root: std::path::PathBuf) -> Self {
		Self {
			root,
			installer: Box::new(crate::tool::installer::Pacman),
			services: Box::new(crate::tool::services::Systemd),
			users: Box::new(crate::tool::user::Usermod),
			cfg,
		}
	}

	pub fn restore(self, filter: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
		for (name, module) in self.cfg.r#mod.iter() {
			if !filter.as_ref().map(|m| m == name).unwrap_or(true) {
				continue;
			}
			println!(" > restoring '{name}'");
			module.restore(name.as_str(), &self)?;
		}

		Ok(())
	}

	pub fn snapshot(self, filter: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
		for (name, module) in self.cfg.r#mod.iter() {
			if !filter.as_ref().map(|m| m == name).unwrap_or(true) {
				continue;
			}
			println!(" > snapshotting '{name}'");
			module.snapshot(name.as_str(), &self)?;
		}

		Ok(())
	}
}
