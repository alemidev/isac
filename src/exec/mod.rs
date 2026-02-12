mod restore;
mod snapshot;

mod context;
pub use context::Context;

#[derive(Debug, thiserror::Error)]
pub enum ExecutorError {
	#[error("error copying files: {0} - {0:?}")]
	IO(#[from] fs_extra::error::Error),

	#[error("error installing packages: {0} - {0:?}")]
	Installation(#[from] crate::tool::installer::InstallerError),

	#[error("error managing system services: {0} - {0:?}")]
	Services(#[from] crate::tool::services::ServiceError),

	#[error("error executing custom shell command: {0} - {0:?}")]
	Command(#[from] crate::tool::CommandError),
}
