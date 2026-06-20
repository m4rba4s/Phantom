use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use std::net::IpAddr;

use crate::config::PhantomConfig;
use crate::{proxy, scanner, tunnel};

pub async fn run_interactive_wizard(config: &PhantomConfig) -> Result<()> {
    println!("\nStarting PHANTOM Interactive Wizard...\n");

    let modes = &["Scan", "Proxy", "Tunnel", "Wrap", "eBPF Filter", "TUI Dashboard"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select operation mode")
        .default(0)
        .items(&modes[..])
        .interact()?;

    match selection {
        0 => run_scan_wizard(config).await?,
        1 => run_proxy_wizard(config).await?,
        2 => run_tunnel_wizard(config).await?,
        3 => run_wrap_wizard(config).await?,
        4 => run_ebpf_wizard(config).await?,
        5 => run_tui_dashboard().await?,
        _ => unreachable!(),
    }

    Ok(())
}

async fn run_scan_wizard(config: &PhantomConfig) -> Result<()> {
    let target: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Target IP address")
        .interact_text()?;

    let target_ip: IpAddr = target.parse().map_err(|_| {
        anyhow::anyhow!("Invalid IP address provided.")
    })?;

    let ports: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Ports to scan")
        .default("80,443".to_string())
        .interact_text()?;

    let port_list = scanner::parse_ports(&ports)?;

    let do_fragment = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Enable IP Fragmentation (Stealth/Evasion)?")
        .default(true)
        .interact()?;

    let mtu: u32 = if do_fragment {
        Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Fragment MTU size (bytes)")
            .default(24)
            .interact_text()?
    } else {
        1500
    };

    let decoys: u8 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Number of decoy hosts (Spoofed IPs)")
        .default(0)
        .interact_text()?;

    let delay_ms: u64 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Delay between probes (ms) - Increase for stealth")
        .default(100)
        .interact_text()?;

    let authorized = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("I confirm I am authorized to scan this target")
        .default(false)
        .interact()?;

    if !authorized {
        anyhow::bail!("Authorization required to proceed.");
    }

    let mut scan_config = scanner::ScanConfig {
        target: target_ip,
        ports: port_list,
        fragment: do_fragment,
        fragment_mtu: mtu as u16,
        delay_ms,
        jitter_percent: config.timing.jitter_percent,
        decoy_count: decoys,
        ..Default::default()
    };

    if decoys > 0 {
        scan_config.generate_decoys(decoys);
    }

    let results = scanner::run_scan(&scan_config).await?;

    println!("\n{:<8} {:<12} {:<10}", "PORT", "STATE", "LATENCY");
    println!("{}", "-".repeat(32));

    for result in &results {
        if result.status == scanner::PortStatus::Open {
            let latency = result.latency_ms
                .map(|l| format!("{:.2}ms", l))
                .unwrap_or_else(|| "-".to_string());
            println!("{:<8} \x1b[32mopen\x1b[0m       {:<10}", result.port, latency);
        }
    }

    Ok(())
}

async fn run_proxy_wizard(config: &PhantomConfig) -> Result<()> {
    let listen_addr: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Listen address")
        .default(config.proxy.listen.clone())
        .interact_text()?;

    let authorized = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("I confirm I am authorized to run proxy mode")
        .default(false)
        .interact()?;

    if !authorized {
        anyhow::bail!("Authorization required to proceed.");
    }

    proxy::start_proxy(config, &listen_addr).await
}

async fn run_tunnel_wizard(config: &PhantomConfig) -> Result<()> {
    let tunnel_modes = &["dns", "icmp", "doh"];
    let mode_idx = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select tunnel mode")
        .default(0)
        .items(&tunnel_modes[..])
        .interact()?;
    let mode = tunnel_modes[mode_idx].to_string();

    let domain: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Target domain for tunneling")
        .default(config.tunnel.domain.clone())
        .interact_text()?;

    let authorized = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("I confirm I am authorized to establish this tunnel")
        .default(false)
        .interact()?;

    if !authorized {
        anyhow::bail!("Authorization required to proceed.");
    }

    tunnel::start_tunnel(config, &mode, &domain).await
}

async fn run_wrap_wizard(config: &PhantomConfig) -> Result<()> {
    let command_str: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Command to wrap (e.g., 'curl http://127.0.0.1')")
        .interact_text()?;

    let command: Vec<String> = command_str.split_whitespace().map(|s| s.to_string()).collect();

    if command.is_empty() {
        anyhow::bail!("Command cannot be empty.");
    }

    let authorized = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("I confirm I am authorized to wrap this traffic")
        .default(false)
        .interact()?;

    if !authorized {
        anyhow::bail!("Authorization required to proceed.");
    }

    proxy::wrap_command(config, &command).await
}

async fn run_ebpf_wizard(_config: &PhantomConfig) -> Result<()> {
    let iface: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Network interface to attach XDP filter (e.g. eth0, wlan0, xdp_test)")
        .default("xdp_test".to_string())
        .interact_text()?;

    let authorized = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("I confirm I am authorized to attach eBPF programs to this interface")
        .default(false)
        .interact()?;

    if !authorized {
        anyhow::bail!("Authorization required to proceed.");
    }

    tracing::info!("Loading eBPF XDP program to interface {}", iface);
            
    // Handle Ctrl-C
    let (tx, mut rx) = tokio::sync::mpsc::channel(1);
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        tx.send(()).await.unwrap();
    });

    match crate::transport::ebpf::EbpfLoader::load_and_attach(&iface) {
        Ok(_loader) => {
            tracing::info!("eBPF XDP program loaded successfully!");
            tracing::info!("Press Ctrl-C to detach and exit.");
            rx.recv().await;
            tracing::info!("Detaching eBPF program...");
            Ok(())
        }
        Err(e) => {
            anyhow::bail!("Failed to load eBPF program: {}", e);
        }
    }
}

async fn run_tui_dashboard() -> Result<()> {
    #[cfg(feature = "tui")]
    {
        let mut dashboard = crate::tui::Dashboard::new();
        dashboard.log("PHANTOM Tactical Dashboard Initialized".to_string());
        dashboard.log("Loading modules...".to_string());
        dashboard.log("Stealth protocols engaged.".to_string());
        
        // Run blocking TUI loop
        tokio::task::spawn_blocking(move || {
            if let Err(e) = dashboard.run() {
                eprintln!("TUI Error: {}", e);
            }
        }).await?;
    }
    
    #[cfg(not(feature = "tui"))]
    {
        println!("\n[!] TUI feature is not enabled in this build.");
        println!("Please rebuild with: cargo build --release --features tui\n");
    }
    
    Ok(())
}
