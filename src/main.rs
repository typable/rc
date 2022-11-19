use std::env;
use std::process::Command;

use rc::prelude::*;

fn main() {
    if let Err(err) = run() {
        error!("{}", err);
    }
}

fn run() -> Result<()> {
    let config = Config::load()?;
    let mut args = env::args();
    args.next().unwrap();
    if let Some(name) = args.next() {
        if let Some((_, path)) = find_program(&config, &name) {
            if let Err(err) = Command::new(config.editor).args([&path]).status() {
                error!("{}", err);
            }
        } else {
            warn!("No config found for {}!", name);
        }
    } else {
        let mut config_count = 0;
        if let Some(programs) = config.programs {
            config_count = programs.len();
        }
        info!("");
        info!("\"rc\" - A config file management utility");
        info!("");
        info!("Usage: rc <program>");
        info!("Use \"rc rc\" to add a new config file.");
        info!("");
        info!("Editor: {}", config.editor);
        info!("Config files: {}", config_count);
        info!("");
    }
    Ok(())
}

fn find_program(config: &Config, name: &str) -> Option<(String, String)> {
    let (id, path) = config
        .programs
        .as_ref()?
        .iter()
        .find(|(id, _)| name.eq(*id))?;
    Some((id.to_string(), path.to_string()))
}
