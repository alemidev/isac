pub mod installer;
pub use installer::Installer;

pub mod services;
pub use services::ServiceManager;

#[derive(Debug, thiserror::Error)]
pub enum CommandError {
	#[error("error executing command in shell: {0} - {0:?}")]
	Shell(#[from] std::io::Error),

	#[error("command returned error: {0}")]
	Command(String),
}

pub fn run_command<T: AsRef<std::ffi::OsStr>>(cmd: &'static str, pref: &[&'static str], args: &[T]) -> Result<(), CommandError> {
	let out = std::process::Command::new(cmd)
		.args(pref)
		.args(args)
		.output()?;

	if !out.status.success() {
		return Err(CommandError::Command(
			String::from_utf8(out.stdout).unwrap_or_default() +
			&String::from_utf8(out.stderr).unwrap_or_default()
		))
	}

	Ok(())
}
