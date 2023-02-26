use std::env;
use std::env::Args;
use std::process::Command;

use rc::prelude::*;
use regex::Regex;

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
            "-h" | "--help" => show_help(),
            "-l" | "--list" => list_programs(&config),
            "-p" | "--path" => get_program_path(&config, &mut args),
            _ => edit_program(&config, &arg),
        }
    } else {
        show_info();
    }
    Ok(())
}

fn show_info() {
    info!("rc - A config file management utility");
    info!("");
    info!("Usage:  rc [options] <program>");
    info!("");
    info!("rc is a tool which allows you to manage your config files without");
    info!("the hassle of remembering the location of all your files.");
    info!("");
    info!("Use \"rc rc\" to manage your configuration.");
    info!("");
    info!("For configuration instructions visit:");
    info!("https://github.com/typable/rc#configuration");
    info!("");
    info!("For a listing of commands, use \"rc --help\".");
}

fn show_help() {
    info!("rc - A config file management utility");
    info!("");
    info!("Usage:  rc [options] <program>");
    info!("");
    info!("rc is a tool which allows you to manage your config files without");
    info!("the hassle of remembering the location of all your files.");
    info!("");
    info!("Use \"rc rc\" to manage your configuration.");
    info!("");
    info!("For configuration instructions visit:");
    info!("https://github.com/typable/rc#configuration");
    info!("");
    info!("All available commands:");
    info!("  rc <program>              Edit config file for a specific program.");
    info!("  rc [--path|-p] <program>  Get config path for a specific program.");
    info!("  rc [--list|-l]            List all config files.");
    info!("  rc [--help|-h]            Show help description.");
}

fn list_programs(config: &Config) {
    let mut width = 2;
    let mut lines = Vec::new();
    if let Some(programs) = &config.programs {
        for program in programs {
            let (id, path) = program;
            if width < id.len() {
                width = id.len();
            }
            lines.push((id, path));
        }
    }
    lines.sort();
    info!("{: <width$}  {}", "ID", "PATH", width = width);
    for line in lines {
        let (id, path) = line;
        info!("{: <width$}  {}", id, path, width = width);
    }
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

fn find_program(config: &Config, name: &str) -> Option<(String, String)> {
    let (id, path) = config
        .programs
        .as_ref()?
        .iter()
        .find(|(id, _)| name.eq(*id))?;
    Some((id.to_string(), evaluate_vars(path)))
}

fn evaluate_vars(path: &str) -> String {
    let re = Regex::new("\\$[\\w_]+").unwrap();
    let mut evaluated_path = path.to_string();
    for cap in re.captures_iter(&path) {
        if let Some(group) = cap.get(0) {
            let string = group.as_str();
            let variable = string[1..].to_string();
            if let Ok(variable_path) = env::var(variable) {
                evaluated_path = evaluated_path.replace(string, &variable_path);
            }
        }
    }
    evaluated_path
}
