use std::{env, error, io, path, str};

use clap::Parser;
use regex::RegexBuilder;

use domain::{Args, Config, Template};

mod domain;

async fn notify(
    notify_url: &String,
    notify_token: &String,
    template: String,
) -> Result<(), Box<dyn error::Error>> {
    let req = reqwest::Client::new()
        .post(notify_url)
        .header("Content-Type", "application/json")
        .header("Authorization", "Bearer ".to_owned() + &notify_token)
        .body(template);

    match req.send().await {
        Ok(r) => {
            if r.status().is_success() {
                return Ok(());
            }
            Err(Box::new(std::io::Error::new(
                io::ErrorKind::Other,
                format!(
                    "notify: response err: {}: {}",
                    notify_url,
                    r.status().to_string()
                ),
            )))
        }
        Err(_e) => Err(Box::new(std::io::Error::new(
            io::ErrorKind::Other,
            format!("notify: request err: {}: {}", notify_url, _e.to_string()),
        ))),
    }
}

fn create_message_from(tmpl: Template) -> String {
    let mut message = tmpl
        .tmpl
        .replace("{{actor}}", &tmpl.actor)
        .replace("{{title}}", &tmpl.title)
        .replace("{{link}}", &tmpl.link)
        .replace("{{changes}}", &tmpl.changes.join("\\n"));
    if tmpl.alerts.len() > 0 {
        message = tmpl.alerts.join("\\n") + "\\n\\n" + &message
    }
    return tmpl
        .body
        .replace("{{message}}", &message.replace("\n", "\\n"));
}

fn is_alert(changes: &[String], pattern: &str) -> Result<bool, std::io::Error> {
    let regex = RegexBuilder::new(pattern)
        .case_insensitive(true)
        .build()
        .map_err(|_err| std::io::Error::new(io::ErrorKind::Other, "unable to parse regex"))?;

    for i in changes {
        if regex.is_match(&std::fs::read_to_string(i)?) {
            return Ok(true);
        }
    }
    Ok(false)
}

fn get_alerts(changes: &[String], config: Config) -> Result<Vec<String>, Box<dyn error::Error>> {
    let mut alerts: Vec<String> = Vec::new();

    if let Some(list) = &config.watches.prepend_with {
        for item in list {
            if is_alert(&changes, &item.if_content_match)? {
                alerts.push(String::from(&item.message));
            }
        }
    }
    Ok(alerts)
}

fn get_changes(diff: &str, pattern: String) -> Result<Vec<String>, Box<dyn error::Error>> {
    let regex = regex::Regex::new(&pattern)?;

    let files: Result<Vec<_>, _> = diff
        .lines()
        .filter(|line| path::Path::new(line).exists() && regex.is_match(line))
        .map(|line| Ok(String::from(line)))
        .collect();

    files
}

fn git_diff(from: &String, to: &String) -> Result<String, std::io::Error> {
    let output = std::process::Command::new("git")
        .args(["diff", "--name-only", &from, &to])
        .output()
        .map_err(|_err| {
            std::io::Error::new(
                io::ErrorKind::Other,
                format!("unable to run git diff: {}", _err.to_string()),
            )
        })?;

    let stdout = String::from_utf8(output.stdout).map_err(|_err| {
        std::io::Error::new(
            io::ErrorKind::Other,
            format!("unable to parse output: {}", _err.to_string()),
        )
    })?;

    Ok(stdout)
}

fn read_config(config: String) -> Result<Config, Box<dyn error::Error>> {
    let contents = std::fs::read_to_string(config)
        .map_err(|_err| {
            Box::new(io::Error::new(
                io::ErrorKind::Other,
                format!("unable to read yaml: {}", _err.to_string()),
            ))
        })?
        .replace("${WD_NOTIFY_URL}", &env::var("WD_NOTIFY_URL")?)
        .replace("${WD_NOTIFY_TOKEN}", &env::var("WD_NOTIFY_TOKEN")?)
        .replace("${WD_NOTIFY_TOPIC}", &env::var("WD_NOTIFY_TOPIC")?);

    let config: Config = serde_yaml::from_str(&contents).map_err(|_err| {
        Box::new(io::Error::new(
            io::ErrorKind::Other,
            format!("unable to parse yaml: {}", _err.to_string()),
        ))
    })?;

    Ok(config)
}

async fn sirene(args: Args) -> Result<(), Box<dyn error::Error>> {
    let config = read_config(args.config).map_err(|_err| {
        Box::new(io::Error::new(
            io::ErrorKind::Other,
            format!("unable to get config: {}", _err.to_string()),
        ))
    })?;

    let changes = get_changes(
        &git_diff(&args.from, &args.to)?,
        String::from(&config.watches.file_pattern),
    )?;

    if changes.len() == 0 {
        println!(
            "{}",
            "no files matching the pattern: ".to_owned() + &config.watches.file_pattern
        );
        return Ok(());
    }

    return notify(
        &config.notify.url,
        &config.notify.token,
        create_message_from(Template {
            actor: args.actor.clone(),
            title: args.title.replace("\"", ""),
            changes: changes.clone(),
            tmpl: config.message.template.clone(),
            link: args.link,
            body: config.notify.body.clone(),
            alerts: get_alerts(&changes, config.clone())?,
        }),
    )
    .await;
}

#[tokio::main]
async fn main() {
    match sirene(Args::parse()).await {
        Ok(_r) => std::process::exit(0),
        Err(e) => {
            println!("error: {}", e);
            std::process::exit(1)
        }
    }
}
