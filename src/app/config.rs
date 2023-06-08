use super::state::State;
use email_address::EmailAddress;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use windows_sys::Win32::Foundation::WININET_E_FAILED_DUETOSECURITYCHECK;

use serde::{Deserialize, Serialize};
use serde_json::Result;

const WATCHDOG_STORE_FILENAME: &str = ".watchdog_store";
const DEFAULT_CONFIG_FILENAME: &str = "watchdog_config.json";

//
// Store

// This element allows for management of a file named .watchdog_store, that exists alongside the running .exe,
// and stores a plaintext path reference to the desired Config JSON file
// This store file must is expected to exist in the same place, and if it does not exist, it will be
// automatically created, and populated with a default value
//

pub struct Store {
    path: PathBuf,
}

impl Store {
    pub fn build_empty() -> Self {
        Self {
            path: PathBuf::new(),
        }
    }

    pub fn build() -> Self {
        // Create new store instance
        let s = Self {
            path: Store::create_path(),
        };

        // Ensure the file exists on disk
        if !s.path.exists() {
            s.write(&Store::default_config_filepath());
        }

        s
    }

    // Read the contents of the .watchdog_store file, and parse the string to a PathBuf
    pub fn read(&self) -> PathBuf {
        if !self.path.exists() {
            self.write(&Store::default_config_filepath());
        }

        let data = fs::read_to_string(&self.path)
            .expect("Config ERROR could not read .watchdog_store contents");

        Path::new(&data).to_path_buf()
    }

    // Write the provided PathBuf to the contents of .watchdog_store
    pub fn write(&self, filepath: &PathBuf) {
        fs::write(&self.path, filepath.to_str().unwrap().to_string())
            .expect("Config ERROR could not write .watchdog_store to disk");
    }

    // Helper to generate the expected filepath of .watchdog_store
    pub fn create_path() -> PathBuf {
        // Get location of current executable
        let cwd = env::current_exe().expect("Unable to get current exe working directory");
        // Create filepath for generic text file alongside .exe
        cwd.parent().unwrap().join(WATCHDOG_STORE_FILENAME)
    }

    // Helper to generate the default filepath of the default config.json file
    pub fn default_config_filepath() -> PathBuf {
        // Get location of current executable
        let cwd = env::current_exe().expect("Unable to get current exe working directory");
        // Create a filepath for a JSON file alongside .exe
        cwd.parent().unwrap().join(DEFAULT_CONFIG_FILENAME)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub hello: String,
    // email: Email,
    // network: Network,
}

impl Config {
    //
    // Create default Config instance
    //
    fn default() -> Self {
        Self {
            hello: "world".to_string(),
        }
    }

    // Create customized Config instance by parsing JSON file
    fn parse(filepath: &std::path::PathBuf) -> Result<Self> {
        // Parse & Validate existing JSON Config file
        let data = std::fs::read_to_string(filepath)
            .expect(format!("Failed to read json config file {:?}", filepath).as_str());

        println!("Read existing json config");

        let c: Result<Self> = serde_json::from_str(data.as_str());

        c
    }

    pub fn validate(&self) -> bool {
        // TO DO make sure all fields are okay
        return true;
    }

    // Helper to convert to JSON string
    fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    // Store any changes to the Config instance to the JSON file
    pub fn write(&self, filepath: &std::path::PathBuf) {
        if self.validate() {
            let data = &self.to_json();
            fs::write(filepath, data).expect("Unable to write Watchdog Config JSON file");
        } else {
            println!("TO DO handle invalid config, did not write to json");
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Email {
    startup_success: Vec<String>,
    startup_failure: Vec<String>,
    non_responsive: Vec<String>,
    email_limit_per_day: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Network {
    client_listen_port: u32,
    client_response_port: u32,
}

pub fn init(state: &mut State) -> Config {
    //
    // Initialize .watchdog_store file
    state.json.store = Store::build();
    state.json.filepath = state.json.store.read();

    let c = if !state.json.filepath.exists() {
        // Initialize Config instance brand new
        let c = Config::default();

        // Write to disk
        c.write(&state.json.filepath);

        c
    } else {
        // Initialize from existing file
        let c = Config::parse(&state.json.filepath);

        // Invalid json file, reset the store to defaults
        if c.is_err() {
            println!("Error parsing the specified config Json file");

            // Reset store, write
            state.json.filepath = Store::default_config_filepath();
            state.json.store.write(&state.json.filepath);

            // Load default config
            let c = Config::default();
            c.write(&state.json.filepath);

            c
        } else {
            c.unwrap()
        }
    };

    // Write it back to disk for good measure

    println!("Stored default JSON config to disk");
    state.json.parsed = true;

    c

    /*
    let email = Email {
        startup_success: vec![EmailAddress::from_str("blake@blakerutledge.com").unwrap()],
        startup_failure: vec![EmailAddress::from_str("blake@blakerutledge.com").unwrap()],
        non_responsive: vec![EmailAddress::from_str("blake@blakerutledge.com").unwrap()],
        email_limit_per_day: 3,
    };

    let network = Network {
        client_listen_port: 1235,
        client_response_port: 1234,
    };
    */
}

// Helper to write the Config instance to a new json file, and also update the .watchdog_store reference
pub fn move_config(file: PathBuf, state: &mut State, config: &mut Config) {
    state.json.filepath = file;
    state.json.store.write(&state.json.filepath);
    config.write(&state.json.filepath);
}

pub fn replace_from_file(file: PathBuf, state: &mut State, config: &mut Config) {
    // Read file from disk
    let c = Config::parse(&file);
    if c.is_err() {
        println!("Error parsing this config Json file");
    } else {
        // Update path, update store, write
        state.json.filepath = file;
        state.json.store.write(&state.json.filepath);

        // Replace config instance
        *config = c.unwrap();
    }
}

pub fn reset_config(state: &mut State, config: &mut Config) {
    let c = Config::default();
    *config = c;
    config.write(&state.json.filepath);
}

pub fn reinit_config(state: &mut State, config: &mut Config) {
    // Update path, update store, write
    state.json.filepath = Store::default_config_filepath();
    state.json.store.write(&state.json.filepath);

    // Reset config
    let c = Config::default();
    *config = c;
    config.write(&state.json.filepath);
}
