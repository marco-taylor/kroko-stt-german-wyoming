use crate::models::Model;
use std::{env, path::PathBuf};

pub const MODEL_NAME: &str = "kroko";
pub const PROGRAM_NAME: &str = "kroko";

#[derive(Clone)]
pub struct Config {
    pub model_dir: PathBuf,
    pub model: Model,
    pub host: String,
    pub port: u16,
    pub num_threads: i32,
    pub language: String,
    pub log_level: String,
}
impl Config {
    pub fn from_env() -> Result<Self, String> {
        let value = |name: &str, default: &str| env::var(name).unwrap_or_else(|_| default.into());
        let model = Model::parse(&value("KROKO_MODEL", "classic"))?;
        let config = Self {
            model_dir: PathBuf::from(value("MODEL_DIR", "models")).join(model.key()),
            model,
            host: value("HOST", "127.0.0.1"),
            port: value("PORT", "10300").parse().map_err(|_| "PORT must be 1..65535")?,
            num_threads: value("NUM_THREADS", "1").parse().map_err(|_| "NUM_THREADS must be 1..4")?,
            language: value("LANGUAGE", "de").to_string(),
            log_level: value("LOG_LEVEL", "info").to_lowercase(),
        };
        if config.port == 0 { return Err("PORT must be 1..65535".into()); }
        if !(1..=4).contains(&config.num_threads) { return Err("NUM_THREADS must be 1..4".into()); }
        if !matches!(config.language.as_str(), "de" | "de-DE") { return Err("This pinned monolingual model supports LANGUAGE=de or de-DE only".into()); }
        if !matches!(config.log_level.as_str(), "error" | "warn" | "info" | "debug") { return Err("LOG_LEVEL must be error, warn, info or debug".into()); }
        Ok(config)
    }
    pub fn enabled(&self, level: &str) -> bool {
        let rank = |v| match v { "error" => 0, "warn" => 1, "info" => 2, _ => 3 };
        rank(level) <= rank(&self.log_level)
    }
}
