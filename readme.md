# Rust System Monitoring Agent
A simple Rust-based system monitoring agent that collects CPU, memory, and disk usage information and periodically sends it to a configured HTTP server.

## Features
CPU usage percentage

Total and used RAM in KB

Disk names and available/total space in GB

Logging with adjustable log level

Configuration-driven server URL, log level, and send interval

Runs continuously and sends stats at configurable intervals

## Requirements
Rust toolchain (1.65+ recommended)

config.toml file (example below)

Network connectivity to send HTTP POST requests

## Installation and Usage
Clone the repository or create the source files.

Create a config.toml file in the project root with the following content:

`````

server_url = "http://localhost:8000/log"
log_level = "info"          # optional: off, error, warn, info, debug, trace
interval_seconds = 30       # optional: send interval in seconds

``````
Build and run the agent:

`````
cargo build
cargo run
``````

The agent will send system stats in JSON format to the configured server URL and log its activity both to the console and agent.log file.

## Configuration Options

| Parameter          | Description                               | Default |
| ------------------ | ----------------------------------------- | ------- |
| `server_url`       | HTTP endpoint URL to send logs (required) | N/A     |
| `log_level`        | Log verbosity level (info recommended)    | `info`  |
| `interval_seconds` | Interval in seconds between each send     | 30      |

## Possible Improvements
Support HTTPS and authentication

Additional system metrics (network, processes, etc.)

Windows service / Linux daemon integration

Remote configuration updates

Retry logic on failure

Automated tests and CI/CD pipelines

