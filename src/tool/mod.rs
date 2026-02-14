pub mod installer;
pub use installer::Installer;

pub mod services;
pub use services::ServiceManager;

pub mod user;
pub use user::UserManager;

#[derive(Debug, thiserror::Error)]
pub enum CommandError {
	#[error("error executing command in shell: {0} - {0:?}")]
	Shell(#[from] std::io::Error),

	#[error("command returned error: {0}")]
	Command(String),
}

pub const NO_ARGS: &[&str; 0] = &[];

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

pub fn bash_exec(cwd: &std::path::Path, script: &str) -> std::io::Result<()> {
	let mut child = std::process::Command::new("bash")
		.arg("-")
		.current_dir(cwd)
		.stdin(std::process::Stdio::piped())
		.spawn()?;

	use std::io::Write;
	if let Some(mut stdin) = child.stdin.take() {
		stdin.write_all(script.as_bytes())?;
	} else {
		eprintln!("[!] error passing script via stdin");
	}

	child.wait()?;

	Ok(())
}
