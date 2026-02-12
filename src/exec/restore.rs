use fs_extra::dir::CopyOptions;


impl crate::conf::Module {
	pub fn restore(&self, ctx: &crate::exec::Context) -> Result<(), super::ExecutorError> {
		ctx.installer.install(&self.dependencies)?;

		for c in self.configs.iter() {
			std::fs::create_dir_all(&ctx.config_to)?;
			fs_extra::copy_items(
				&[&ctx.config_from.join(c)],
				&ctx.config_to,
				&CopyOptions::new().overwrite(true)
			)?;
		}

		if let Some(ref loader) = self.loader {
			crate::tool::run_command("bash", &["-c"], &[loader])?;
		}

		for s in self.services.iter() {
			let unit_file = ctx.services_path.join(s);
			if std::fs::exists(&unit_file)? {
				ctx.services.install(unit_file)?;
				ctx.services.reload()?;
			}
			ctx.services.enable(s)?;
			ctx.services.start(s)?;
		}

		if let Some(ref user) = self.user {
			// TODO allow configuring groups and wether it's a system user
			ctx.users.create_user(user, &[], true)?;
		}

		Ok(())
	}
}
