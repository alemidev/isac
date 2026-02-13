use fs_extra::dir::CopyOptions;


impl crate::conf::Module {
	pub fn snapshot(&self, name: &str, ctx: &crate::exec::Context) -> Result<(), super::ExecutorError> {
		for d in self.dependencies.iter() {
			if !ctx.installer.is_installed(d.as_str())? {
				eprintln!("<?> package {d} is dep for {name} but it's not installed");
			}
		}

		std::fs::create_dir_all(&ctx.services_local_path)?;
		for s in self.services.iter() {
			let s_path = ctx.services_path.join(s);
			if std::fs::exists(&s_path)? {
				std::fs::copy(s_path, &ctx.services_local_path)?;
			}
		}

		std::fs::create_dir_all(&ctx.configs_local_path)?;
		for c in self.configs.iter() {
			fs_extra::copy_items(
				&[&ctx.configs_path.join(c)],
				&ctx.configs_local_path,
				&CopyOptions::new().overwrite(true)
			)?;
		}

		if let Some(ref data) = self.data {
			let cwd = ctx.data_local_path.join(name);
			std::fs::create_dir_all(&cwd)?;
			crate::tool::bash_exec(cwd, &data.dumper)?;
		}

		Ok(())
	}
}
