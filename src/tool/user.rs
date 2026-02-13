
pub trait UserManager: std::fmt::Debug {
	fn create_user(&self, name: &str, groups: &[String], system: bool) -> Result<(), UserManagerError>;
}

#[derive(Debug, thiserror::Error)]
pub enum UserManagerError {
	#[error("error executing command: {0} - {0:?}")]
	Command(#[from] super::CommandError),
}

#[derive(Debug)]
pub struct Usermod;

impl UserManager for Usermod {
	fn create_user(&self, name: &str, groups: &[String], system: bool) -> Result<(), UserManagerError> {
		let prefs: &[&str] = if system {
			&["--system"]
		} else {
			&[]
		};
		super::run_command("useradd", prefs, &[name])?;
		
		for g in groups {
			super::run_command("usermod", &["-aG"], &[g.as_ref(), name])?;
		}

		Ok(())
	}
}
