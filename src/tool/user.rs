
pub trait UserManager: std::fmt::Debug {
	fn esixts(&self, name: &str) -> Result<bool, UserManagerError>;
	fn create(&self, name: &str, basedir: Option<&str>, groups: &[String], system: bool) -> Result<(), UserManagerError>;
	fn change_owner(&self, path: &std::path::Path, name: &str) -> Result<(), UserManagerError>;
}

#[derive(Debug, thiserror::Error)]
pub enum UserManagerError {
	#[error("error executing command: {0} - {0:?}")]
	Command(#[from] super::CommandError),

	#[error("error accessing file: {0} - {0:?}")]
	IO(#[from] std::io::Error),
}

#[derive(Debug)]
pub struct Usermod;

impl UserManager for Usermod {
	fn esixts(&self, name: &str) -> Result<bool, UserManagerError> {
		Ok(std::fs::read_to_string("/etc/passwd")?.contains(name))
	}

	fn create(&self, name: &str, basedir: Option<&str>, groups: &[String], system: bool) -> Result<(), UserManagerError> {
		let prefs: &[&str] = if system {
			&["--system"]
		} else {
			&[]
		};
		super::run_command("useradd", prefs, &[name])?;
		
		for g in groups {
			super::run_command("usermod", &["-aG"], &[g.as_ref(), name])?;
		}

		if let Some(bd) = basedir {
			super::run_command("usermod", &["-d"], &[bd, name])?;
		}

		Ok(())
	}

	fn change_owner(&self, path: &std::path::Path, name: &str) -> Result<(), UserManagerError> {
		Ok(
			super::run_command(
				"chown",
				&["-R"],
				&[name, path.to_string_lossy().as_ref()],
			)?
		)
	}
}
