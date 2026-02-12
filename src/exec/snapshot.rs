use fs_extra::dir::CopyOptions;


impl crate::conf::Module {
	pub fn snapshot(&self, ctx: &crate::exec::Context) -> Result<(), super::ExecutorError> {
		for c in self.configs.iter() {
			fs_extra::copy_items(
				&[&ctx.config_to.join(c)],
				ctx.config_from.join(c),
				&CopyOptions::new()
			)?;
		}

		if let Some(ref dumper) = self.dumper {
			crate::tool::run_command("bash", &["-c"], &[dumper])?;
		}

		Ok(())
	}
}
