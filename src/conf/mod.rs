use indexmap::IndexMap;


#[derive(Debug, serde_default::DefaultFromSerde, serde::Deserialize, serde::Serialize)]
pub struct Isac {
	#[serde(default)]
	pub system: System,

	#[serde(default)]
	pub r#mod: IndexMap<String, Module>,
}

#[serde_inline_default::serde_inline_default]
#[derive(Debug, serde_default::DefaultFromSerde, serde::Deserialize, serde::Serialize)]
pub struct System {
	#[serde_inline_default("/etc".to_string())]
	pub configs: String,

	#[serde_inline_default("/etc/systemd/system".to_string())]
	pub services: String,
}

#[serde_inline_default::serde_inline_default]
#[derive(Debug, serde_default::DefaultFromSerde, serde::Deserialize, serde::Serialize)]
pub struct Module {
	#[serde(default)]
	pub dependencies: Vec<String>,

	#[serde(default)]
	pub configs: Vec<String>,

	#[serde(default)]
	pub services: Vec<String>,

	#[serde(default)]
	pub user: Option<UserConfig>,

	#[serde(default)]
	pub data: Option<DataConfig>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct UserConfig {
	pub name: String,
	#[serde(default)]
	pub groups: Vec<String>,
	#[serde(default)]
	pub system: bool,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct DataConfig {
	pub loader: String,
	pub dumper: String,
}

impl Isac {
	// TODO returning a string as err is cheap but a bit ugly
	pub fn load(path: std::path::PathBuf) -> Result<Self, String> {
		let config_raw = match std::fs::read_to_string(path) {
			Ok(c) => c,
			Err(e) => {
				return Err(format!("[!] error loading config: {e}"));
			},
		};

		let config: Isac = match toml::from_str(&config_raw) {
			Ok(c) => c,
			Err(e) => {
				return Err(format!("[!] invalid config file:\n{e}"));
			},
		};

		Ok(config)
	}
}
