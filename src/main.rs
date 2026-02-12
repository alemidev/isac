mod conf;
mod exec;
mod tool;

use clap::Parser;

/// InfraStructureAsCode - configure fresh machines from a single config tree
#[derive(Debug, Parser)]
struct Cli {
	/// action to take
	#[clap(subcommand)]
	action: Action,

	/// specify a config path (and tree root)
	#[arg(short, long)]
	config: Option<std::path::PathBuf>,
}

#[derive(Debug, Clone, clap::Subcommand)]
enum Action {
	/// show loaded config and quit
	Config,

	/// apply all modules from loaded config tree
	Apply {
		// /// perform a dry run, without actually applying changes
		// #[arg(long, default_value_t = false)]
		// dry_run: bool,
	}
}

fn main() {
	let cli = Cli::parse();

	let (path, cwd) = match cli.config {
		Some(p) => {
			let mut root = p.clone();
			root.pop();
			(root, p)
		},
		None => {
			let cwd = std::env::current_dir()
				.expect("cannot get current path and no custom path was provided");
			(cwd.join("isac.toml"), cwd)
		},
	};

	let config = match conf::Isac::load(path) {
		Ok(c) => c,
		Err(msg) => {
			eprintln!("{msg}");
			return;
		},
	};

	// just print config and quit
	if matches!(cli.action, Action::Config) {
		println!("{}", toml::to_string_pretty(&config).expect("failed serializing back config"));
	}

	let ctx = exec::Context::new(config, cwd);

	if let Err(e) = ctx.exec() {
		eprintln!("[!] error applying changes to system: {e}");
	}
}
