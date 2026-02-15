use std::str::FromStr;


impl crate::conf::Module {
	pub fn snapshot(&self, name: &str, ctx: &crate::exec::Context) -> Result<(), super::ExecutorError> {
		for d in self.deps.iter() {
			if !ctx.installer.is_installed(d.as_str())? {
				eprintln!("<?> package {d} is dep for {name} but it's not installed");
			}
		}

		for f in self.files.iter() {
			let file_path = std::path::PathBuf::from_str(f).expect("infallible");
			// TODO this doesnt work on windows!
			let Ok(local_path) = file_path.strip_prefix("/").map(|x| x.to_path_buf())
			else {
				return Err(super::ExecutorError::Path(f.clone()));
			};
			if let Some(ancestor) = local_path.parent() {
				std::fs::create_dir_all(ancestor)?;
			}
			let store_path = ctx.root.join(local_path);

			if file_path.is_dir() {
				dircpy::copy_dir(&file_path, &store_path)?;
			} else {
				std::fs::copy(file_path, store_path)?;
			}
		}

		if let Some(ref dumper) = self.exec.dumper {
			crate::tool::bash_exec(&ctx.root, dumper)?;
		}

		Ok(())
	}
}
