# PHANTOM 👻

**Pentest Traffic Masquerading & Security Validation Framework**

PHANTOM is an advanced, modular network framework written in Rust. It is designed for authorized security assessments, Purple Team engagements, and IDS/IPS/NGFW evasion research. The framework provides extensive capabilities for traffic masquerading, protocol fragmentation, covert tunneling, and high-performance packet processing via eBPF XDP.

> [!WARNING]
> **Authorized Use Only.**
> This software is provided for educational and authorized testing purposes only. The authors assume no liability and are not responsible for any misuse or damage caused by this program. By using this software, you agree to use it only on systems you own or have explicit permission to test.

---

## 🎯 Key Features

- **Traffic Mimicry:** Simulation of legitimate TLS fingerprints (JA3/JA4) and HTTP header profiles (Chrome, Firefox, Safari) to evade signature-based detection.
- **Timing Evasion:** Implementation of adaptive jitter, delays, and circadian timing patterns to bypass heuristic traffic analysis.
- **Protocol Obfuscation:** Advanced TCP/IP fragmentation, segment overlapping, and out-of-order delivery.
- **Stealth Scanning:** Asynchronous, raw-socket SYN scanning with decoy support (Nmap-like capabilities).
- **Covert Channels:** Pluggable tunneling architectures (DNS over HTTPS, ICMP) to establish C2 communications.
- **eBPF XDP Port Knocking (New!):** High-performance, kernel-level packet filtering and stealth port knocking using eBPF, operating entirely below the standard networking stack.
- **Interactive UI:** Step-by-step wizard interface built on `dialoguer` and `ratatui` for real-time engagement control.

## 🏗️ Architecture

PHANTOM is structured as a Cargo Workspace containing the primary user-space application and a kernel-space eBPF module:
- `phantom` (Root Crate): The main user-space daemon handling UI, packet crafting, tunneling, and eBPF orchestration.
- `netprobe-ebpf`: The kernel-side XDP module that filters traffic, tracks authorized hosts, and parses port knock sequences directly in the NIC driver ring buffer.

## ⚙️ Prerequisites

To build PHANTOM, especially the eBPF components, you need specific Rust toolchains:

1. **Rust Nightly** (Required for `build-std` in eBPF compilation):
   ```bash
   rustup toolchain install nightly
   rustup component add rust-src --toolchain nightly
   ```

2. **eBPF Linker**:
   ```bash
   cargo install bpf-linker
   ```

3. **eBPF Target**:
   ```bash
   rustup target add bpfel-unknown-none --toolchain nightly
   ```

## 🚀 Build Instructions

The project uses a custom `build.rs` to automatically compile the `netprobe-ebpf` kernel module and embed it into the main user-space binary.

```bash
cargo build --release
```

The compiled binary will be located at `target/release/phantom`.

## 💻 Usage

Phantom requires root privileges (`sudo` or `CAP_SYS_ADMIN` / `CAP_NET_RAW`) for raw socket operations and attaching eBPF programs to network interfaces.

> **Note:** Always pass the `--i-am-authorized` flag to confirm you have legal permission to execute these operations.

### Interactive Mode
The easiest way to explore features (Proxy, Scanner, Tunnels, eBPF):
```bash
sudo target/release/phantom --i-am-authorized interactive
```

### Stealth SYN Scanning
Phantom supports advanced L3/L4 evasion techniques. Replace `<TARGET_IP>` with your authorized target.

**Basic Stealth Scan (Default MTU 24, Delay 100ms)**
```bash
sudo target/release/phantom --i-am-authorized scan <TARGET_IP> -p 80,443,22
```

**Advanced Evasion (Decoys & High Delay)**
Mixes your real traffic with 5 spoofed IP addresses and introduces a 500ms delay to evade rate-limiting.
```bash
sudo target/release/phantom --i-am-authorized scan <TARGET_IP> -p 80,443,22 --decoys 5 --delay 500
```

### Covert DNS Tunneling
Establish a covert DNS channel (requires a controlled authoritative nameserver):
```bash
sudo target/release/phantom --i-am-authorized tunnel --mode dns --domain example.com
```

### eBPF Filter & Port Knocking
Load the XDP program to establish a stealth firewall that only opens specific ports upon receiving a covert cryptographic "knock":
```bash
sudo target/release/phantom --i-am-authorized interactive
# Select "eBPF Filter" and specify your network interface (e.g., eth0, wlan0).
```

## 📜 Configuration

Configuration profiles can be customized via TOML files (e.g. `config/phantom.toml`) to load static profiles for Mimicry, Timing, and Decoys without relying purely on command-line flags.

## ⚖️ License
MIT License.
