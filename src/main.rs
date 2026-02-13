mod conf;
mod exec;
mod tool;

use clap::Parser;

/// Infrastructure Setup As Config
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

	/// apply changes from modules
	Restore {
		/// restore a single module, or all if not specified
		module: Option<String>,

		// /// perform a dry run, without actually applying changes
		// #[arg(long, default_value_t = false)]
		// dry_run: bool,
	},

	/// generate snapshot for modules
	Snapshot {
		/// snapshot a single module, or all if not specified
		module: Option<String>,

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

	let res = match cli.action {
		Action::Config => unreachable!(),
		Action::Snapshot { module } => ctx.snapshot(module),
		Action::Restore { module } => ctx.restore(module),
	};

	if let Err(e) = res {
		eprintln!("[!] error executing restore/snapshot: {e}");
	}
}
