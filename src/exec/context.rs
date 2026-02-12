use std::str::FromStr;


pub struct Context {
	pub cfg: crate::conf::Isac,
	pub configs_path: std::path::PathBuf,
	pub configs_local_path: std::path::PathBuf,
	pub services_path: std::path::PathBuf,
	pub services_local_path: std::path::PathBuf,
	pub data_local_path: std::path::PathBuf,

	pub installer: Box<dyn crate::tool::Installer>,
	pub services: Box<dyn crate::tool::ServiceManager>,
	pub users: Box<dyn crate::tool::UserManager>,
}

impl Context {
	// TODO should gather system info and construct a proper context
	pub fn new(cfg: crate::conf::Isac, root: std::path::PathBuf) -> Self {
		Self {
			configs_path: std::path::PathBuf::from_str(&cfg.system.configs).expect("infallible"),
			configs_local_path: root.join("config"),
			services_path: std::path::PathBuf::from_str(&cfg.system.services).expect("infallible"),
			services_local_path: root.join("services"),
			data_local_path: root.join("data"),
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
