use std::process::Command;

use args::Args;
use clap::Parser;
use colored::Colorize;
use config::{App, Config, Session};
use error::Error;
use jiff::{Span, SpanRound, Unit, Zoned};

mod args;
mod config;
mod error;

fn main() -> anyhow::Result<()> {
    let config = Config::read()?;
    let args = Args::parse();

    match args.command {
        args::Command::Add { name, exe } => add(config, name, exe),
        args::Command::Remove { name } => remove(config, name),
        args::Command::List => list(config),
        args::Command::Start { name } => start(config, name),
        args::Command::Sessions { name } => sessions(config, name),
    }?;

    Ok(())
}

fn add(mut config: Config, name: String, exe: String) -> Result<(), Error> {
    if config.apps.iter().any(|a| a.name == name) {
        return Err(Error::AppExists(name));
    }

    config.apps.push(App {
        name: name.clone(),
        exe,
        sessions: vec![],
    });
    config.save()?;

    println!("added {} to the config file", name.blue());

    Ok(())
}

fn remove(mut config: Config, name: String) -> Result<(), Error> {
    let position = config
        .apps
        .iter()
        .position(|a| a.name == name)
        .ok_or(Error::AppNotFound(name.clone()))?;

    config.apps.remove(position);
    config.save()?;

    println!("removed {} from the config file", name.blue());

    Ok(())
}

fn list(config: Config) -> Result<(), Error> {
    if config.apps.is_empty() {
        println!("{}", "no apps added to the config file".yellow());
    }

    for app in config.apps.iter() {
        let total = app.time()?;
        let recent = app.time_since(Zoned::now().checked_sub(Span::new().days(7))?)?;

        println!("{}", app.name.blue());
        println!(" executable: {}", app.exe.cyan());
        println!(" recorded total: {}", format_span(&total)?.cyan());
        println!(" recorded recently: {}", format_span(&recent)?.cyan());
    }

    Ok(())
}

fn start(mut config: Config, name: String) -> Result<(), Error> {
    let app = config
        .apps
        .iter_mut()
        .find(|a| a.name == name)
        .ok_or(Error::AppNotFound(name))?;

    let start = Zoned::now();
    let status = Command::new(app.exe.clone()).status()?;
    let end = Zoned::now();

    let duration = start.until(&end)?;

    println!("process exited with status: {}", status.to_string().blue());

    if duration.total(Unit::Millisecond)? < 1000_f64 {
        println!(
            "{}",
            "warning: process terminated after less than 1 second".yellow()
        );

        Ok(())
    } else {
        println!("session duration: {}", format_span(&duration)?.blue());

        app.sessions.push(Session {
            timestamp: start,
            duration,
        });
        config.save()
    }
}

fn sessions(config: Config, name: String) -> Result<(), Error> {
    let app = config
        .apps
        .iter()
        .find(|a| a.name == name)
        .ok_or(Error::AppNotFound(name.clone()))?;

    println!("sessions for {}", name.blue());

    if app.sessions.is_empty() {
        println!(" no sessions recorded");
    }

    for session in app.sessions.iter() {
        let timestamp = format_zoned(&session.timestamp);
        let duration = format_span(&session.duration)?;
        println!(" played on {} for {}", timestamp.blue(), duration.blue());
    }

    Ok(())
}

fn format_zoned(zoned: &Zoned) -> String {
    zoned.strftime("%Y-%m-%d at %H:%M %Z").to_string()
}

fn format_span(span: &Span) -> Result<String, Error> {
    let rounded = span.round(SpanRound::new().largest(Unit::Hour))?;
    let h = rounded.get_hours();
    let m = rounded.get_minutes();
    let s = rounded.get_seconds();

    Ok(format!("{:02}h {:02}m {:02}s", h, m, s))
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_format_zoned() {
        let zoned = Zoned::from_str("2024-08-10T23:14:00-04:00[America/New_York]").unwrap();
        assert_eq!(format_zoned(&zoned), "2024-08-10 at 23:14 EDT".to_string());
    }

    #[test]
    fn test_format_span() {
        let span = Span::new().minutes(124);
        assert_eq!(format_span(&span).unwrap(), "02h 04m 00s".to_string());
    }
}
