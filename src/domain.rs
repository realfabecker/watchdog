use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notify {
    pub url: String,
    pub token: String,
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub template: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prepend {
    pub message: String,
    pub if_content_match: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Watches {
    pub file_pattern: String,
    #[serde(default)]
    pub prepend_with: Option<Vec<Prepend>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub notify: Notify,
    pub message: Message,
    pub watches: Watches,
}

pub struct Template {
    pub actor: String,
    pub title: String,
    pub changes: Vec<String>,
    pub link: String,
    pub tmpl: String,
    pub alerts: Vec<String>,
    pub body: String,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(long)]
    pub actor: String,

    #[arg(long)]
    pub from: String,

    #[arg(long)]
    pub to: String,

    #[arg(long)]
    pub link: String,

    #[arg(long)]
    pub title: String,

    #[arg(long)]
    pub config: String,
}
