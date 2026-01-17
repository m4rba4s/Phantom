# Phantom

Phantom is a network traffic manipulation and reconnaissance framework designed for authorized security assessments and IDS/IPS validation. It provides capabilities for traffic masquerading, timing evasion, and protocol fragmentation to simulate sophisticated network actors.

## Disclaimer

**Authorized Use Only.**
This software is provided for educational and authorized testing purposes only. The authors assume no liability and are not responsible for any misuse or damage caused by this program. By using this software, you agree to use it only on systems you own or have explicit permission to test.

## Features

- **Traffic Mimicry**: Simulation of legitimate TLS fingerprints (JA3) and HTTP header profiles (Chrome, Firefox, Safari) to evade signature-based detection.
- **Timing Evasion**: Implementation of adaptive jitter and circadian timing patterns to bypass heuristic traffic analysis.
- **Protocol Obfuscation**: Advanced TCP/IP fragmentation and segmentation techniques.
- **Covert Channels**: Proof-of-concept implementations for DNS and ICMP tunneling mechanisms.
- **Scanner**: Stealth SYN scanning capabilities with decoy support.

## Installation

Building from source requires a stable Rust toolchain.

```bash
cargo build --release
```

The binary will be located at `target/release/phantom`.

## Usage

Phantom operates in several modes. Root privileges are required for raw socket operations (scanning/tunneling).

### Scan Mode
Perform a stealth SYN scan against a target:

```bash
sudo ./phantom --i-am-authorized scan <TARGET_IP> --ports 80,443 --decoys 5
```

### Tunnel Mode (Research)
Establish a covert DNS channel (requires a controlled authoritative nameserver):

```bash
sudo ./phantom --i-am-authorized tunnel --mode dns --domain example.com
```

## Configuration

Configuration is managed via `config/phantom.toml`. Key parameters include:
- `mode`: Operational profile (shadow, tactical, ghost).
- `mimicry.browser_profile`: Target browser fingerprint to emulate.
- `timing.jitter_percent`: Variance in packet timing.

## License

MIT License
