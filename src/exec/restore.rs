use fs_extra::dir::CopyOptions;


impl crate::conf::Module {
	pub fn restore(&self, ctx: &crate::exec::Context) -> Result<(), super::ExecutorError> {
		ctx.installer.install(&self.dependencies)?;

		for c in self.configs.iter() {
			fs_extra::copy_items(
				&[&ctx.config_from.join(c)],
				ctx.config_to.join(c),
				&CopyOptions::new().overwrite(true)
			)?;
		}

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
