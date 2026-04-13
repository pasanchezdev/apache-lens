# applogs — Apache Log Analyzer

CLI tool for analyzing Apache logs. Displays errors, statistics and real-time activity from the terminal.

---

## Installation

**Linux / macOS**

```bash
curl -sSf https://raw.githubusercontent.com/pasanchezdev/apache-lens/main/install.sh | sh
```

**Windows — PowerShell**

```powershell
irm https://raw.githubusercontent.com/pasanchezdev/apache-lens/main/install.ps1 | iex
```

The script handles everything: installs Rust if not present, compiles applogs and makes it available as a global command. Once done, it launches the initial setup automatically.

Requirement: `git` must be installed.

---

## Usage

```bash
applogs status          # detailed view grouped by response type
applogs stats           # general summary: hits, unique IPs, HTTP codes
applogs parse           # list all log entries
applogs top -n 10       # top 10 IPs, paths and user-agents by traffic
applogs filter --ip 192.168.1.1 --code 404
applogs export -o output.json
```

Real-time mode:

```bash
applogs status --live
```

Streams each new log entry as Apache writes it. Stop with Ctrl+C.

---

## Requirements

- Linux or Windows
- Apache installed (Standard, XAMPP, Laragon or other)
- Rust 1.70 or higher to compile
