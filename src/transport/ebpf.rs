use aya::programs::{Xdp, XdpFlags};
use aya::Ebpf;
use aya_log::EbpfLogger;
use std::path::Path;

pub struct EbpfLoader {
    bpf: Ebpf,
    iface: String,
}

impl EbpfLoader {
    /// Loads the compiled eBPF XDP program and attaches it to the specified interface.
    pub fn load_and_attach(iface: &str) -> anyhow::Result<Self> {
        // In a real project, we would use a build script to compile the eBPF code
        pub const EBPF_PROGRAM: &[u8] = aya::include_bytes_aligned!("../../target-ebpf/bpfel-unknown-none/release/netprobe-ebpf");
        let mut bpf = Ebpf::load(EBPF_PROGRAM)?;

        // Attempt to initialize BPF Logger (may fail if aya-log was removed from the kernel side)
        if let Err(e) = EbpfLogger::init(&mut bpf) {
            tracing::debug!("Failed to initialize eBPF logger (expected if stripped): {}", e);
        }

        // Attach the XDP program named 'pattern_filter'
        let program: &mut Xdp = bpf.program_mut("pattern_filter").unwrap().try_into()?;
        program.load()?;
        
        // Use default flags. SKB_MODE can be used as fallback if native driver support is lacking.
        program.attach(iface, XdpFlags::default())?;

        tracing::info!("Successfully loaded and attached XDP program to {}", iface);

        Ok(Self {
            bpf,
            iface: iface.to_string(),
        })
    }
}

impl Drop for EbpfLoader {
    fn drop(&mut self) {
        tracing::info!("Detaching XDP program from {}", self.iface);
        // Bpf instance drops automatically and detaches programs if owned.
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ebpf_struct_instantiation() {
        // eBPF loader needs root to attach, so we can't fully integration-test without CAP_SYS_ADMIN.
        // This test merely ensures it compiles.
    }
}
