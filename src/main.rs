#![allow(dead_code, unused_imports, clippy::enum_variant_names, clippy::wrong_self_convention, clippy::explicit_auto_deref)]
//! PHANTOM - Pentest Traffic Masquerading Framework
//!
//! A stealth-focused traffic masquerading tool for AUTHORIZED penetration testing.
//! Makes reconnaissance and scanning traffic blend with legitimate network activity.
//!
//! # WARNING
//! This tool is designed for AUTHORIZED security testing ONLY.
//! Unauthorized use against systems you do not own or have explicit permission
//! to test is illegal and unethical.

use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing::{info, warn, Level};
use tracing_subscriber::FmtSubscriber;

mod config;
mod mimicry;
mod noise;
mod proxy;
mod scanner;
mod timing;
mod tunnel;
mod transport;
mod menu;

#[cfg(feature = "tui")]
mod tui;

use config::PhantomConfig;

const BANNER: &str = r#"
    ██████╗ ██╗  ██╗ █████╗ ███╗   ██╗████████╗ ██████╗ ███╗   ███╗
    ██╔══██╗██║  ██║██╔══██╗████╗  ██║╚══██╔══╝██╔═══██╗████╗ ████║
    ██████╔╝███████║███████║██╔██╗ ██║   ██║   ██║   ██║██╔████╔██║
    ██╔═══╝ ██╔══██║██╔══██║██║╚██╗██║   ██║   ██║   ██║██║╚██╔╝██║
    ██║     ██║  ██║██║  ██║██║ ╚████║   ██║   ╚██████╔╝██║ ╚═╝ ██║
    ╚═╝     ╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═══╝   ╚═╝    ╚═════╝ ╚═╝     ╚═╝
                    Pentest Traffic Masquerading Framework
"#;

const WARNING: &str = r#"
╔══════════════════════════════════════════════════════════════════════════════╗
║                              ⚠️  WARNING ⚠️                                   ║
╠══════════════════════════════════════════════════════════════════════════════╣
║  This tool is designed for AUTHORIZED penetration testing ONLY.              ║
║                                                                              ║
║  By using this tool, you confirm that:                                       ║
║    • You have explicit written authorization to test the target systems      ║
║    • You understand and accept all legal responsibilities                    ║
║    • All actions will be logged for audit purposes                           ║
║                                                                              ║
║  Unauthorized access to computer systems is a criminal offense.              ║
╚══════════════════════════════════════════════════════════════════════════════╝
"#;

#[derive(Parser)]
#[command(name = "phantom")]
#[command(author, version, about = "Stealth traffic masquerading for authorized pentesting")]
#[command(long_about = None)]
#[command(after_help = WARNING)]
struct Cli {
    /// Path to configuration file
    #[arg(short, long, default_value = "config/phantom.toml")]
    config: String,

    /// Verbosity level (-v, -vv, -vvv)
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    /// Acknowledge authorization warning
    #[arg(long)]
    i_am_authorized: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start proxy mode - intercept and transform traffic
    Proxy {
        /// Override listen address
        #[arg(short, long)]
        listen: Option<String>,
    },

    /// Wrap a command's traffic through PHANTOM
    Wrap {
        /// Command to wrap
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        command: Vec<String>,
    },

    /// Start tunnel mode for covert channel
    Tunnel {
        /// Tunnel mode: dns, icmp, doh
        #[arg(short, long, default_value = "dns")]
        mode: String,

        /// Domain for DNS tunneling
        #[arg(short, long)]
        domain: Option<String>,
    },

    /// Test traffic against IDS signatures
    Test {
        /// Output PCAP file
        #[arg(short, long)]
        pcap: Option<String>,
    },

    /// SYN scan with evasion techniques (like nmap but stealthier)
    Scan {
        /// Target IP address
        target: String,

        /// Ports to scan (e.g., "22,80,443" or "1-1000")
        #[arg(short, long, default_value = "21,22,23,25,53,80,110,143,443,445,3306,3389,5432,8080")]
        ports: String,

        /// Disable IP fragmentation
        #[arg(long)]
        no_fragment: bool,

        /// Fragment MTU size
        #[arg(long, default_value = "24")]
        mtu: u16,

        /// Number of decoy hosts
        #[arg(short, long, default_value = "0")]
        decoys: u8,

        /// Delay between probes (ms)
        #[arg(long, default_value = "100")]
        delay: u64,

        /// Show all ports (including closed/filtered)
        #[arg(short, long)]
        all: bool,
    },

    /// Show configuration
    ShowConfig,

    /// Start interactive wizard
    Interactive,
}

fn init_logging(verbosity: u8) {
    let level = match verbosity {
        0 => Level::INFO,
        1 => Level::DEBUG,
        _ => Level::TRACE,
    };

    let subscriber = FmtSubscriber::builder()
        .with_max_level(level)
        .with_target(true)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set tracing subscriber");
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    init_logging(cli.verbose);

    // Print banner
    println!("{}", BANNER);

    // Authorization check
    if !cli.i_am_authorized {
        eprintln!("{}", WARNING);
        eprintln!("\nTo proceed, you must acknowledge authorization with --i-am-authorized\n");
        std::process::exit(1);
    }

    warn!("PHANTOM session started - all actions are being logged for audit");

    // Load configuration
    let config = PhantomConfig::load(&cli.config).unwrap_or_else(|e| {
        warn!("Failed to load config from {}: {}. Using defaults.", cli.config, e);
        PhantomConfig::default()
    });

    info!("Loaded configuration: mode={}", config.general.mode);

    match cli.command {
        Commands::Proxy { listen } => {
            let listen_addr = listen.unwrap_or_else(|| config.proxy.listen.clone());
            info!("Starting proxy on {}", listen_addr);
            proxy::start_proxy(&config, &listen_addr).await?;
        }

        Commands::Wrap { command } => {
            if command.is_empty() {
                eprintln!("Error: No command specified to wrap");
                std::process::exit(1);
            }
            info!("Wrapping command: {:?}", command);
            proxy::wrap_command(&config, &command).await?;
        }

        Commands::Tunnel { mode, domain } => {
            let domain = domain.unwrap_or_else(|| config.tunnel.domain.clone());
            info!("Starting {} tunnel to {}", mode, domain);
            tunnel::start_tunnel(&config, &mode, &domain).await?;
        }

        Commands::Test { pcap } => {
            info!("Running IDS evasion test");
            if let Some(pcap_path) = pcap {
                info!("Will save PCAP to: {}", pcap_path);
            }
            // TODO: Implement test mode
            println!("Test mode not yet implemented");
        }

        Commands::Scan {
            target,
            ports,
            no_fragment,
            mtu,
            decoys,
            delay,
            all,
        } => {
            use scanner::{ScanConfig, PortStatus, run_scan, parse_ports};
            use std::net::IpAddr;

            info!("Starting stealth SYN scan against {}", target);

            // Parse IP address (Strict mode - No DNS leaks)
            let target_ip: IpAddr = target.parse().map_err(|_| {
                anyhow::anyhow!(
                    "OPSEC FAILURE: Hostname resolution is disabled to prevent DNS leaks.\n\
                     You provided '{}', which is not a valid IP address.\n\
                     \n\
                     SOLUTION:\n\
                     1. Resolve the domain manually using a secure channel (e.g. Tor/DoH).\n\
                     2. Pass the IP address directly to phantom.",
                    target
                )
            })?;

            let port_list = parse_ports(&ports)?;

            let mut scan_config = ScanConfig {
                target: target_ip,
                ports: port_list,
                fragment: !no_fragment,
                fragment_mtu: mtu,
                delay_ms: delay,
                jitter_percent: config.timing.jitter_percent,
                decoy_count: decoys,
                ..Default::default()
            };

            if decoys > 0 {
                scan_config.generate_decoys(decoys);
            }

            match run_scan(&scan_config).await {
                Ok(results) => {
                    println!("\n{:<8} {:<12} {:<10}", "PORT", "STATE", "LATENCY");
                    println!("{}", "-".repeat(32));

                    for result in &results {
                        if all || result.status == PortStatus::Open {
                            let latency = result.latency_ms
                                .map(|l| format!("{:.2}ms", l))
                                .unwrap_or_else(|| "-".to_string());

                            let status_str = match result.status {
                                PortStatus::Open => "\x1b[32mopen\x1b[0m",
                                PortStatus::Closed => "\x1b[31mclosed\x1b[0m",
                                PortStatus::Filtered => "\x1b[33mfiltered\x1b[0m",
                            };

                            println!("{:<8} {:<20} {:<10}", result.port, status_str, latency);
                        }
                    }

                    let open_count = results.iter().filter(|r| r.status == PortStatus::Open).count();
                    let filtered_count = results.iter().filter(|r| r.status == PortStatus::Filtered).count();

                    println!("\n{} ports scanned: {} open, {} filtered",
                        results.len(), open_count, filtered_count);
                }
                Err(e) => {
                    eprintln!("Scan failed: {}", e);
                    std::process::exit(1);
                }
            }
        }

        Commands::ShowConfig => {
            println!("{}", toml::to_string_pretty(&config)?);
        }

        Commands::Interactive => {
            menu::run_interactive_wizard(&config).await?;
        }
    }

    Ok(())
}
