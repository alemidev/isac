use fs_extra::dir::CopyOptions;


impl crate::conf::Module {
	pub fn snapshot(&self, name: &str, ctx: &crate::exec::Context) -> Result<(), super::ExecutorError> {
		for d in self.dependencies.iter() {
			if !ctx.installer.is_installed(d.as_str())? {
				eprintln!("<?> package {d} is dep for {name} but it's not installed");
			}
		}

		std::fs::create_dir_all(&ctx.path.services_local)?;
		for s in self.services.iter() {
			let s_path = ctx.path.services.join(format!("{s}.service"));
			if std::fs::exists(&s_path)? {
				std::fs::copy(s_path, ctx.path.services_local.join(format!("{s}.service")))?;
			}
		}

		std::fs::create_dir_all(&ctx.path.configs_local)?;
		for c in self.configs.iter() {
			let dest = ctx.path.configs_local.join(c)
				.parent()
				.map(|p| p.to_path_buf());
			if let Some(ref d) = dest {
				std::fs::create_dir_all(d)?;
			}
			fs_extra::copy_items(
				&[&ctx.path.configs.join(c)],
				dest.as_ref().unwrap_or(&ctx.path.configs_local),
				&CopyOptions::new().overwrite(true).copy_inside(true)
			)?;
		}

		if let Some(ref data) = self.data {
			let cwd = ctx.path.data.join(name);
			std::fs::create_dir_all(&cwd)?;
			crate::tool::bash_exec(cwd, &data.dumper)?;
		}

		Ok(())
	}
}
