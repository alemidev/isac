use fs_extra::dir::CopyOptions;


impl crate::conf::Module {
	pub fn restore(&self, name: &str, ctx: &crate::exec::Context) -> Result<(), super::ExecutorError> {
		ctx.installer.install(&self.dependencies)?;

		if let Some(ref compile) = self.compile {
			let path = ctx.path.compile.join(name);
			std::fs::create_dir_all(&path)?;
			crate::tool::bash_exec(path, compile)?;
		}

		std::fs::create_dir_all(&ctx.path.configs)?;
		for c in self.configs.iter() {
			let dest = ctx.path.configs.join(c)
				.parent()
				.map(|p| p.to_path_buf());
			if let Some(ref d) = dest {
				std::fs::create_dir_all(d)?;
			}
			fs_extra::copy_items(
				&[&ctx.path.configs_local.join(c)],
				dest.as_ref().unwrap_or(&ctx.path.configs),
				&CopyOptions::new().overwrite(true).copy_inside(true)
			)?;
		}

		if let Some(ref data) = self.data {
			let cwd = ctx.path.data.join(name);
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
			let unit_file = ctx.path.services_local.join(format!("{s}.service"));
			if std::fs::exists(&unit_file)? {
				std::fs::copy(unit_file, &ctx.path.services)?;
				ctx.services.reload()?;
			}
			ctx.services.enable(s)?;
			ctx.services.start(s)?;
		}

		Ok(())
	}
}
