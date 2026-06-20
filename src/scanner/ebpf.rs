#![no_std]
#![no_main]

use aya_ebpf::{
    bindings::xdp_action,
    macros::{map, xdp},
    maps::{Array, RingBuf},
    programs::XdpContext,
};
use core::mem;

// Simplified network headers since network_types is not available in std
#[repr(C)]
struct EthHdr {
    _dst: [u8; 6],
    _src: [u8; 6],
    ether_type: u16,
}
impl EthHdr { const LEN: usize = 14; }

#[repr(C)]
struct Ipv4Hdr {
    _ihl_version: u8,
    _tos: u8,
    _tot_len: u16,
    _id: u16,
    _frag_off: u16,
    _ttl: u8,
    proto: u8,
    _check: u16,
    _saddr: u32,
    _daddr: u32,
}
impl Ipv4Hdr { const LEN: usize = 20; }

#[repr(C)]
struct TcpHdr {
    source: u16,
    _dest: u16,
    _seq: u32,
    _ack_seq: u32,
    _res1_doff: u8,
    flags: u8,
    _window: u16,
    _check: u16,
    _urg_ptr: u16,
}
impl TcpHdr { const LEN: usize = 20; }

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ScanConfig {
    pub target_ip: u32,
    pub ports: [u16; 16],
    pub flags_mask: u8,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ScanResult {
    pub port: u16,
    pub state: u8, // 1 = SYN-ACK, 2 = RST
}

#[map]
static CONFIG: Array<ScanConfig> = Array::with_max_entries(1, 0);

#[map]
static RESULTS: RingBuf = RingBuf::with_byte_size(4096 * 256, 0);

#[xdp]
pub fn xdp_scan_monitor(ctx: XdpContext) -> u32 {
    match try_xdp_scan_monitor(ctx) {
        Ok(ret) => ret,
        Err(_) => xdp_action::XDP_PASS,
    }
}

#[inline(always)]
fn ptr_at<T>(ctx: &XdpContext, offset: usize) -> Result<*const T, ()> {
    let start = ctx.data();
    let end = ctx.data_end();
    let len = mem::size_of::<T>();

    if start + offset + len > end {
        return Err(());
    }

    Ok((start + offset) as *const T)
}

fn try_xdp_scan_monitor(ctx: XdpContext) -> Result<u32, ()> {
    let ethhdr: *const EthHdr = ptr_at(&ctx, 0)?;
    
    // 0x0800 in network byte order
    if unsafe { (*ethhdr).ether_type } != u16::from_be(0x0800) {
        return Ok(xdp_action::XDP_PASS);
    }

    let ipv4hdr: *const Ipv4Hdr = ptr_at(&ctx, EthHdr::LEN)?;
    let ip_proto = unsafe { (*ipv4hdr).proto };
    if ip_proto != 6 { // TCP
        return Ok(xdp_action::XDP_PASS);
    }

    // Note: IHL is dynamic in IPv4, but assuming 20 bytes for simplicity here
    let tcphdr: *const TcpHdr = ptr_at(&ctx, EthHdr::LEN + Ipv4Hdr::LEN)?;
    
    let flags = unsafe { (*tcphdr).flags };
    let syn = (flags & 0x02) != 0;
    let ack = (flags & 0x10) != 0;
    let rst = (flags & 0x04) != 0;

    let mut state = 0;
    if syn && ack {
        state = 1;
    } else if rst {
        state = 2;
    }

    if state != 0 {
        if let Some(mut reserve) = RESULTS.reserve::<ScanResult>(0) {
            unsafe {
                let result = reserve.as_mut();
                (*result).port = u16::from_be((*tcphdr).source);
                (*result).state = state;
            }
            reserve.submit(0);
        }
    }

    Ok(xdp_action::XDP_PASS)
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

// Ensure the license is preserved
#[no_mangle]
#[link_section = "license"]
pub static _LICENSE: [u8; 13] = *b"Dual BSD/GPL\0";
