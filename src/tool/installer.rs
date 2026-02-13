
pub trait Installer: std::fmt::Debug {
	fn install(&self, packages: &[String]) -> Result<(), InstallerError>;
	fn is_installed(&self, package: &str) -> Result<bool, InstallerError>;
}

#[derive(Debug, thiserror::Error)]
pub enum InstallerError {
	#[error("error running command: {0} - {0:?}")]
	Command(#[from] super::CommandError),

	#[error("error executing command in shell: {0} - {0:?}")]
	Shell(#[from] std::io::Error),
}


#[derive(Debug)]
pub struct Pacman;

impl Installer for Pacman {
	fn install(&self, packages: &[String]) -> Result<(), InstallerError> {
		Ok(super::run_command("pacman", &["-S", "--noconfirm"], packages)?)
	}

	fn is_installed(&self, package: &str) -> Result<bool, InstallerError> {
		Ok(
			std::process::Command::new("pacman")
				.arg("-Q")
				.arg(package)
				.output()?
				.status
				.success()
		)
	}
}

