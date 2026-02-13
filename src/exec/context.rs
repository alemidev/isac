use std::str::FromStr;


#[derive(Debug)]
pub struct Context {
	pub cfg: crate::conf::Isac,
	pub path: Paths,
	pub installer: Box<dyn crate::tool::Installer>,
	pub services: Box<dyn crate::tool::ServiceManager>,
	pub users: Box<dyn crate::tool::UserManager>,
}

#[derive(Debug)]
pub struct Paths {
	pub configs: std::path::PathBuf,
	pub configs_local: std::path::PathBuf,
	pub services: std::path::PathBuf,
	pub services_local: std::path::PathBuf,
	pub data: std::path::PathBuf,
	pub compile: std::path::PathBuf,
}

impl Context {
	// TODO should gather system info and construct a proper context
	pub fn new(cfg: crate::conf::Isac, root: std::path::PathBuf) -> Self {
		Self {
			path: Paths {
				configs: std::path::PathBuf::from_str(&cfg.system.configs).expect("infallible"),
				configs_local: root.join("config"),
				services: std::path::PathBuf::from_str(&cfg.system.services).expect("infallible"),
				services_local: root.join("services"),
				data: root.join("data"),
				compile: root.join("compile"),
			},
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
