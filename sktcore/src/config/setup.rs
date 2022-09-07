use serde::Deserialize;
use serde::Serialize;
use serde_json;

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    binds: Vec<ShortKut>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ShortKut {
    keys: Vec<String>,
    action_type: String,
    action: ActionDescription,
}

#[derive(Deserialize, Serialize, Debug)]
enum AType {
    AppByName,
    AppByLocationStr,
    CommandByStr,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ActionDescription {
    program_name: Option<String>,
    program_path: Option<String>,
    opts: Option<String>,
}

pub fn load_config_json() -> Config {
    let path = "~/config/sortkut/config.json";
    let config_json = {
        let config_str = std::fs::read_to_string(path).expect("Error opening file");
        serde_json::from_str::<Config>(&config_str).unwrap()
    };
    config_json
}
