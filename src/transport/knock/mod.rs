use aya::maps::{HashMap, RingBuf};
use aya::programs::{xdp::XdpLinkId, Xdp, XdpFlags};
use aya::Ebpf;
use bytes::BytesMut;
use std::sync::Arc;
use tokio::sync::Mutex;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum KnockError {
    #[error("eBPF load error: {0}")]
    EbpfError(#[from] aya::EbpfError),
    #[error("Map error: {0}")]
    MapError(#[from] aya::maps::MapError),
    #[error("Program error: {0}")]
    ProgramError(#[from] aya::programs::ProgramError),
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct KnockPattern {
    pub expected_seq: u32,
    pub expected_port: u16,
    pub action: u8,
    pub _padding: [u8; 1],
}
unsafe impl aya::Pod for KnockPattern {}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct AuthorizedHost {
    pub src_ip: u32,
    pub expiry: u64,
    pub _padding: [u8; 4],
}
unsafe impl aya::Pod for AuthorizedHost {}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct KnockEvent {
    pub src_ip: u32,
    pub port: u16,
    pub action: u8,
    pub _padding: [u8; 1],
}
unsafe impl aya::Pod for KnockEvent {}

pub struct KnockHandler {
    bpf: Arc<Mutex<Ebpf>>,
    iface: String,
    link_id: Option<XdpLinkId>,
}

impl KnockHandler {
    pub fn load(iface: &str, _ttl: u64) -> Result<Self, KnockError> {
        let bpf_code = aya::include_bytes_aligned!("../../../target-ebpf/bpfel-unknown-none/release/netprobe-ebpf");
        let mut bpf = Ebpf::load(bpf_code)?;
        
        let program: &mut Xdp = bpf.program_mut("pattern_filter").unwrap().try_into()?;
        program.load()?;
        let link_id = program.attach(iface, XdpFlags::default())?;

        Ok(Self {
            bpf: Arc::new(Mutex::new(bpf)),
            iface: iface.to_string(),
            link_id: Some(link_id),
        })
    }

    pub async fn add_pattern(&self, pattern_id: u32, pattern: KnockPattern) -> Result<(), KnockError> {
        let mut bpf = self.bpf.lock().await;
        let mut patterns_map: HashMap<_, u32, KnockPattern> = HashMap::try_from(bpf.map_mut("KNOCK_PATTERNS").unwrap())?;
        patterns_map.insert(pattern_id, pattern, 0)?;
        Ok(())
    }

    pub async fn remove_pattern(&self, pattern_id: u32) -> Result<(), KnockError> {
        let mut bpf = self.bpf.lock().await;
        let mut patterns_map: HashMap<_, u32, KnockPattern> = HashMap::try_from(bpf.map_mut("KNOCK_PATTERNS").unwrap())?;
        patterns_map.remove(&pattern_id)?;
        Ok(())
    }

    pub async fn is_authorized(&self, src_ip: u32) -> bool {
        let mut bpf = self.bpf.lock().await;
        let hosts_map: HashMap<_, u32, u64> = HashMap::try_from(bpf.map_mut("AUTHORIZED_HOSTS").unwrap()).unwrap();
        hosts_map.get(&src_ip, 0).is_ok()
    }

    pub async fn revoke_host(&self, src_ip: u32) -> Result<(), KnockError> {
        let mut bpf = self.bpf.lock().await;
        let mut hosts_map: HashMap<_, u32, u64> = HashMap::try_from(bpf.map_mut("AUTHORIZED_HOSTS").unwrap())?;
        let _ = hosts_map.remove(&src_ip);
        Ok(())
    }

    pub async fn poll_events(&self) -> Result<Vec<KnockEvent>, KnockError> {
        let mut bpf = self.bpf.lock().await;
        let mut events_map = RingBuf::try_from(bpf.map_mut("KNOCK_EVENTS").unwrap())?;
        
        let mut events = Vec::new();
        while let Some(item) = events_map.next() {
            let event = unsafe { std::ptr::read_unaligned(item.as_ptr() as *const KnockEvent) };
            events.push(event);
        }
        Ok(events)
    }

    pub async fn cleanup_expired(&self) {
        // Handled entirely by eBPF data path, but user-space could iterate and clean up
        // Currently skipping implementation for brevity as BPF handles it on hit.
    }

    pub async fn unload(&mut self) -> Result<(), KnockError> {
        // Automatically unloaded on Drop or can detach
        if let Some(link_id) = self.link_id.take() {
            let mut bpf = self.bpf.lock().await;
            let program: &mut Xdp = bpf.program_mut("pattern_filter").unwrap().try_into()?;
            program.detach(link_id)?;
        }
        Ok(())
    }
}
