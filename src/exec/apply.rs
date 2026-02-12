use fs_extra::dir::CopyOptions;


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

impl crate::conf::Module {
	pub fn apply(&self, ctx: &crate::exec::Context) -> Result<(), ExecutorError> {
		ctx.installer.install(&self.dependencies)?;

		fs_extra::copy_items(&[&ctx.config_from], &ctx.config_to, &CopyOptions::new().overwrite(true))?;

		if let Some(ref loader) = self.loader {
			crate::tool::run_command("bash", &["-c"], &[loader])?;
		}

		for s in self.services.iter() {
			ctx.services.enable(s)?;
			ctx.services.start(s)?;
		}

		Ok(())
	}
}
