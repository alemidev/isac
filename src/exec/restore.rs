use fs_extra::dir::CopyOptions;


impl crate::conf::Module {
	pub fn restore(&self, name: &str, ctx: &crate::exec::Context) -> Result<(), super::ExecutorError> {
		ctx.installer.install(&self.dependencies)?;

		std::fs::create_dir_all(&ctx.configs_path)?;
		for c in self.configs.iter() {
			fs_extra::copy_items(
				&[&ctx.configs_local_path.join(c)],
				&ctx.configs_path,
				&CopyOptions::new().overwrite(true)
			)?;
		}

		if let Some(ref data) = self.data {
			let cwd = ctx.data_local_path.join(name);
			if std::fs::exists(&cwd)? {
				crate::tool::bash_exec(cwd, &data.loader)?;
			} else {
				eprintln!("<?> no data to load for {name}");
			}
		}

		if let Some(ref user) = self.user {
			ctx.users.create_user(&user.name, &user.groups, user.system)?;
		}

		for s in self.services.iter() {
			let unit_file = ctx.services_local_path.join(format!("{s}.service"));
			if std::fs::exists(&unit_file)? {
				std::fs::copy(unit_file, &ctx.services_path)?;
				ctx.services.reload()?;
			}
			ctx.services.enable(s)?;
			ctx.services.start(s)?;
		}

		Ok(())
	}
}
