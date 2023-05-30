use super::state::State;
use email_address::EmailAddress;
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use serde_json::Result;

// TODO:
// read json config file, command line, env, etc...

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    email: Email,
    network: Network,
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
    let filepath = "./assets/config.json";
    let config_json = std::fs::read_to_string(filepath).expect("failed to read json config file");
    state.json.exists = true;
    state.json.filepath = Some(String::from(filepath));

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

    let c: Config =
        serde_json::from_str(config_json.as_str()).expect("failed to parse json config");

    state.json.parsed = true;

    c
}
