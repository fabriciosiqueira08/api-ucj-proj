use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PipeFile {
    pub filename: String,
    pub sheet_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PipeId {
    pub id: String,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub pipefy_api_token: String,
    pub pipefy_graphql_endpoint: String,
    pub pipe_to_file: HashMap<String, PipeFile>,
    pub pipe_ids: HashMap<String, PipeId>,
}

impl Config {
    pub fn new() -> Self {
        dotenv().ok();

        let pipefy_api_token = env::var("PIPEFY_API_TOKEN").expect("PIPEFY_API_TOKEN não está definida no .env");
        let pipefy_graphql_endpoint = env::var("PIPEFY_GRAPHQL_ENDPOINT").expect("PIPEFY_GRAPHQL_ENDPOINT não está definida no .env");

        let pipe_to_file: HashMap<String, PipeFile> = serde_json::from_str(
            &env::var("PIPE_TO_FILE").expect("PIPE_TO_FILE não está definida no .env"),
        ).expect("Erro ao ler PIPE_TO_FILE");

        let pipe_ids: HashMap<String, PipeId> = serde_json::from_str(
            &env::var("PIPE_IDS").expect("PIPE_IDS não está definida no .env"),
        ).expect("Erro ao ler PIPE_IDS");

        Config {
            pipefy_api_token,
            pipefy_graphql_endpoint,
            pipe_to_file,
            pipe_ids,
        }
    }
}
