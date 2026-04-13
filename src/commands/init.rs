use anyhow::Result;
use std::io::{self, Write};
use std::path::Path;

use crate::config::{save, Config};
use crate::discovery::find_log_files;
use crate::i18n::{Messages, EN, ES};


enum Os {
    Linux,
    Windows,
}

enum InstallType {
    Standard(Os),
    Laragon,
    Xampp(Os),
    Custom,
}

fn prompt(label: &str, msg: &str) -> Result<String> {
    print!("{label} {msg}");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn ask_os(t: &Messages) -> Result<Os> {
    println!("\n  {}", t.select_os);
    println!("  1. {}", t.os_linux);
    println!("  2. {}", t.os_windows);

    loop {
        match prompt(t.prompt, "").as_deref() {
            Ok("1") => return Ok(Os::Linux),
            Ok("2") => return Ok(Os::Windows),
            _       => println!("  {}", t.invalid_option),
        }
    }
}

fn known_path(install: InstallType) -> String {
    match install {
        InstallType::Standard(Os::Linux)   => "/var/log/apache2/access.log".to_string(),
        InstallType::Standard(Os::Windows) => "C:\\Apache24\\logs\\access.log".to_string(),
        InstallType::Laragon               => "C:\\laragon\\logs\\apache-access.log".to_string(),
        InstallType::Xampp(Os::Windows)    => "C:\\xampp\\apache\\logs\\access.log".to_string(),
        InstallType::Xampp(Os::Linux)      => "/opt/lampp/logs/access_log".to_string(),
        InstallType::Custom               => String::new(),
    }
}

fn ask_manually(t: &Messages) -> Result<String> {
    loop {
        let example = if cfg!(windows) {
            "C:\\Apache24\\logs\\access.log"
        } else {
            "/var/log/apache2/access.log"
        };
        let path = prompt(t.prompt, &format!("{} [e.g. {example}]: ", t.enter_manually))?;
        if Path::new(&path).exists() {
            return Ok(path);
        }
        println!("\n  {}", t.path_not_found);
    }
}

fn ask_install(t: &Messages) -> Result<String> {
    loop {
        println!("\n  {}", t.select_install);
        println!("  1. {}", t.install_standard);
        println!("  2. {}", t.install_laragon);
        println!("  3. {}", t.install_xampp);
        println!("  4. {}", t.install_other);

        let install = loop {
            match prompt(t.prompt, "").as_deref() {
                Ok("1") => break InstallType::Standard(ask_os(t)?),
                Ok("2") => break InstallType::Laragon,
                Ok("3") => break InstallType::Xampp(ask_os(t)?),
                Ok("4") => break InstallType::Custom,
                _       => println!("  {}", t.invalid_option),
            }
        };

        let path = match install {
            InstallType::Custom => return ask_manually(t),
            other => known_path(other),
        };

        if Path::new(&path).exists() {
            return Ok(path);
        }

        println!("\n  {} {path}", t.path_not_found);
    }
}

fn auto_discover(t: &Messages) -> Result<Option<String>> {
    println!("\n  {}", t.searching);

    let found = find_log_files();

    match found.len() {
        0 => Ok(None),
        1 => {
            let path = found[0].display().to_string();
            println!("  {}  {path}", t.found);
            let answer = prompt(t.prompt, t.use_this_path)?;
            let accepted = matches!(answer.to_lowercase().as_str(), "" | "s" | "y");
            Ok(if accepted { Some(path) } else { None })
        }
        _ => {
            println!("\n  {}", t.multiple_found);
            for (i, p) in found.iter().enumerate() {
                println!("  {}. {}", i + 1, p.display());
            }
            println!("\n  {}", t.select_number);

            loop {
                let input = prompt(t.prompt, "")?;
                match input.parse::<usize>() {
                    Ok(0) => return Ok(None),
                    Ok(n) if n <= found.len() => {
                        return Ok(Some(found[n - 1].display().to_string()))
                    }
                    _ => println!("  {}", t.invalid_option),
                }
            }
        }
    }
}

pub fn run() -> Result<()> {
    println!("\n  1. Español");
    println!("  2. English\n");

    let t: &Messages = loop {
        match prompt(" >", "").as_deref() {
            Ok("1") => break &ES,
            Ok("2") => break &EN,
            _       => println!("  1 / 2"),
        }
    };

    println!("\n{}\n", t.welcome);

    let log_path = match auto_discover(t)? {
        Some(path) => path,
        None => {
            println!("\n  {}", t.not_found_auto);
            ask_install(t)?
        }
    };

    let lang = if std::ptr::eq(t, &ES) { "es" } else { "en" }.to_string();

    println!("\n  {} {log_path}", t.path_set);
    save(&Config { log_path, lang })?;
    println!("  {}", t.config_saved);

    Ok(())
}
