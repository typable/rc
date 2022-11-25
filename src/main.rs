use std::env;
use std::env::Args;
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
    if let Some(arg) = args.next() {
        match arg.as_str() {
            "-l" => list_programs(&config),
            "-p" => get_program_path(&config, &mut args),
            _ => edit_program(&config, &arg),
        }
    } else {
        show_info();
    }
    Ok(())
}

fn list_programs(config: &Config) {
    let mut total = 0;
    let mut width = 2;
    let mut lines = Vec::new();
    if let Some(programs) = &config.programs {
        total = programs.len();
        for program in programs {
            let (id, path) = program;
            if width < id.len() {
                width = id.len();
            }
            lines.push((id, path));
        }
    }
    lines.sort();
    info!("");
    info!("total config files: {}", total);
    info!("");
    warn!("{: <width$}  {}", "ID", "PATH", width = width);
    for line in lines {
        let (id, path) = line;
        info!("{: <width$}  {}", id, path, width = width);
    }
    info!("");
}

fn get_program_path(config: &Config, args: &mut Args) {
    if let Some(id) = args.next() {
        if let Some((_, path)) = find_program(config, &id) {
            info!("{}", path);
        } else {
            warn!("No config found for {}!", id);
        }
    } else {
        warn!("Argument required! Usage: rc -p <program>");
    }
}

fn edit_program(config: &Config, id: &str) {
    if let Some((_, path)) = find_program(config, id) {
        if let Err(err) = Command::new(&config.editor).args([&path]).status() {
            error!("{}", err);
        }
    } else {
        warn!("No config found for {}!", id);
    }
}

fn show_info() {
    info!("");
    warn!("\"rc\" - A config file management utility");
    info!("");
    info!("Usage: rc <program>");
    info!("Use \"rc rc\" to add a new config file.");
    info!("");
}

fn find_program(config: &Config, name: &str) -> Option<(String, String)> {
    let (id, path) = config
        .programs
        .as_ref()?
        .iter()
        .find(|(id, _)| name.eq(*id))?;
    Some((id.to_string(), path.to_string()))
}
