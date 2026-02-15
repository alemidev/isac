use std::str::FromStr;

impl crate::conf::Module {
	pub fn restore(&self, _name: &str, ctx: &crate::exec::Context) -> Result<(), super::ExecutorError> {
		if !self.deps.is_empty() {
			ctx.installer.install(&self.deps)?;
		}

		if let Some(ref user) = self.user
			&& !ctx.users.esixts(&user.name)?
		{
			ctx.users.create(&user.name, user.basedir.as_deref(), &user.groups, user.system)?;
		}

		for f in self.files.iter() {
			let file_path = std::path::PathBuf::from_str(f).expect("infallible");
			// TODO this doesnt work on windows!
			let Ok(local_path) = file_path.strip_prefix("/").map(|x| x.to_path_buf())
			else {
				return Err(super::ExecutorError::Path(f.clone()));
			};
			if let Some(ancestor) = file_path.parent() {
				std::fs::create_dir_all(ancestor)?;
			}
			let store_path = ctx.root.join(local_path);

			if file_path.is_dir() {
				dircpy::copy_dir(&store_path, &file_path)?;
			} else {
				std::fs::copy(store_path, &file_path)?;
			}

			if let Some(ref user) = self.user {
				ctx.users.change_owner(&file_path, &user.name)?;
			}
		}

		if let Some(ref loader) = self.exec.loader {
			crate::tool::bash_exec(&ctx.root, loader)?;
		}

		for s in self.units.iter() {
			ctx.services.reload()?;
			ctx.services.enable(s)?;
			ctx.services.start(s)?;
		}

		Ok(())
	}
}
