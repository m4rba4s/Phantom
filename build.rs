use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=netprobe-ebpf/src");
    println!("cargo:rerun-if-changed=netprobe-ebpf/Cargo.toml");

    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let status = Command::new(cargo)
        .current_dir("netprobe-ebpf")
        .args(["build", "-Z", "build-std=core", "--target", "bpfel-unknown-none", "--release", "--target-dir", "../target-ebpf"])
        .status()
        .expect("Failed to build eBPF program");

    if !status.success() {
        panic!("Failed to compile eBPF program");
    }
}
