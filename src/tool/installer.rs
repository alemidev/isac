
pub trait Installer {
	fn install(&self, packages: &[String]) -> Result<(), InstallerError>;
}

#[derive(Debug, thiserror::Error)]
pub enum InstallerError {
	#[error("error running command: {0} - {0:?}")]
	Command(#[from] super::CommandError)
}


pub struct Pacman;

impl Installer for Pacman {
	fn install(&self, packages: &[String]) -> Result<(), InstallerError> {
		Ok(super::run_command("pacman", &["-S"], packages)?)
	}
}

