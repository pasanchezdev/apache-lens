use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::thread;
use std::time::Duration;

use crate::core::models::LogEntry;
use crate::core::parser::parse_file;
use crate::core::parser::parse_line;

fn status_label(code: u16) -> &'static str {
    match code {
        200 => "OK",
        201 => "Created",
        204 => "No Content",
        301 => "Moved Permanently",
        302 => "Found",
        304 => "Not Modified",
        400 => "Bad Request",
        401 => "Unauthorized",
        403 => "Forbidden",
        404 => "Not Found",
        405 => "Method Not Allowed",
        408 => "Request Timeout",
        429 => "Too Many Requests",
        500 => "Internal Server Error",
        502 => "Bad Gateway",
        503 => "Service Unavailable",
        _   => "Unknown",
    }
}

fn print_entry(entry: &LogEntry) {
    println!("  IP         {}", entry.ip);
    println!("  Fecha      {}", entry.timestamp.format("%d/%m/%Y  %H:%M:%S"));
    println!("  Método     {}", entry.method);
    println!("  Ruta       {}", entry.path);
    println!("  Estado     {}  {}", entry.status, status_label(entry.status));
    println!("  Tamaño     {} bytes", entry.bytes);
    if let Some(ref ua) = entry.user_agent {
        println!("  Agente     {ua}");
    }
    println!();
}

fn print_section(title: &str, icon: &str, entries: &[&LogEntry]) {
    if entries.is_empty() {
        return;
    }
    println!("{icon}  {title}  ({})", entries.len());
    println!("{}", "─".repeat(50));
    for entry in entries {
        print_entry(entry);
    }
}

fn run_snapshot(file: &str, code: Option<u16>) -> Result<()> {
    let entries = parse_file(file)?;

    let filtered: Vec<_> = entries
        .iter()
        .filter(|e| code.map_or(true, |c| e.status == c))
        .collect();

    if filtered.is_empty() {
        println!("\n  Sin resultados.\n");
        return Ok(());
    }

    let ok:     Vec<_> = filtered.iter().copied().filter(|e| e.status < 400).collect();
    let client: Vec<_> = filtered.iter().copied().filter(|e| (400..500).contains(&e.status)).collect();
    let server: Vec<_> = filtered.iter().copied().filter(|e| e.status >= 500).collect();

    println!();
    print_section("Solicitudes exitosas", "✓", &ok);
    print_section("Errores del cliente", "⚠", &client);
    print_section("Errores del servidor", "✗", &server);

    println!("{}", "─".repeat(50));
    println!("  Total   {}   |   OK {}   |   Cliente {}   |   Servidor {}",
        filtered.len(), ok.len(), client.len(), server.len()
    );
    println!();

    Ok(())
}

fn run_live(file: &str, code: Option<u16>) -> Result<()> {
    let mut f = File::open(file)?;
    f.seek(SeekFrom::End(0))?;

    println!("\n  Modo en vivo activo. Esperando nuevas entradas... (Ctrl+C para salir)\n");
    println!("{}", "─".repeat(50));

    let mut reader = BufReader::new(f);
    loop {
        let mut line = String::new();

        while reader.read_line(&mut line)? > 0 {
            if let Some(entry) = parse_line(line.trim()).ok() {
                if code.map_or(true, |c| entry.status == c) {
                    let icon = match entry.status {
                        s if s < 400 => "✓",
                        s if s < 500 => "⚠",
                        _            => "✗",
                    };
                    println!("{icon}  {} {} → {}  {}  {} bytes",
                        entry.timestamp.format("%H:%M:%S"),
                        entry.ip,
                        entry.path,
                        entry.status,
                        entry.bytes,
                    );
                }
            }
            line.clear();
        }

        thread::sleep(Duration::from_millis(500));
    }
}

pub fn run(file: &str, code: Option<u16>, live: bool) -> Result<()> {
    if live {
        run_live(file, code)
    } else {
        run_snapshot(file, code)
    }
}
