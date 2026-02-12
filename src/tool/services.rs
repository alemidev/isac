
#[allow(unused)]
pub trait ServiceManager {
	fn enable(&self, service: &str) -> Result<(), ServiceError>;
	fn disable(&self, service: &str) -> Result<(), ServiceError>;

	fn start(&self, service: &str) -> Result<(), ServiceError>;
	fn stop(&self, service: &str) -> Result<(), ServiceError>;
}

#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
	#[error("error executing command: {0} - {0:?}")]
	Command(#[from] super::CommandError),
}

pub struct Systemd;

impl ServiceManager for Systemd {
	fn start(&self, service: &str) -> Result<(), ServiceError> {
		Ok(super::run_command("systemctl", &["start"], &[service])?)
	}

	fn stop(&self, service: &str) -> Result<(), ServiceError> {
		Ok(super::run_command("systemctl", &["stop"], &[service])?)
	}

	fn enable(&self, service: &str) -> Result<(), ServiceError> {
		Ok(super::run_command("systemctl", &["enable"], &[service])?)
	}

	fn disable(&self, service: &str) -> Result<(), ServiceError> {
		Ok(super::run_command("systemctl", &["disable"], &[service])?)
	}
}
