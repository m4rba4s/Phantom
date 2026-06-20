#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

use aya_ebpf::{
    bindings::xdp_action,
    macros::{map, xdp},
    maps::{HashMap, RingBuf},
    programs::XdpContext,
};
use core::mem;
use network_types::{
    eth::{EthHdr, EtherType},
    ip::{Ipv4Hdr, IpProto},
    tcp::TcpHdr,
};

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct KnockPattern {
    pub expected_seq: u32,
    pub expected_port: u16,
    pub action: u8,
    pub _padding: [u8; 1],
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct AuthorizedHost {
    pub src_ip: u32,
    pub expiry: u64,
    pub _padding: [u8; 4],
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct KnockEvent {
    pub src_ip: u32,
    pub port: u16,
    pub action: u8,
    pub _padding: [u8; 1],
}

#[map]
static KNOCK_PATTERNS: HashMap<u32, KnockPattern> = HashMap::<u32, KnockPattern>::with_max_entries(1024, 0);

#[map]
static AUTHORIZED_HOSTS: HashMap<u32, u64> = HashMap::<u32, u64>::with_max_entries(1024, 0);

#[map]
static KNOCK_EVENTS: RingBuf = RingBuf::with_byte_size(1024 * 1024, 0);

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}

#[xdp]
pub fn pattern_filter(ctx: XdpContext) -> u32 {
    match try_pattern_filter(ctx) {
        Ok(ret) => ret,
        Err(_) => xdp_action::XDP_PASS,
    }
}

#[inline(always)]
unsafe fn ptr_at<T>(ctx: &XdpContext, offset: usize) -> Result<*const T, ()> {
    let start = ctx.data();
    let end = ctx.data_end();
    let len = mem::size_of::<T>();

    if start + offset + len > end {
        return Err(());
    }

    Ok((start + offset) as *const T)
}

fn try_pattern_filter(ctx: XdpContext) -> Result<u32, ()> {
    let ethhdr: *const EthHdr = unsafe { ptr_at(&ctx, 0)? };
    match unsafe { (*ethhdr).ether_type } {
        EtherType::Ipv4 => {}
        _ => return Ok(xdp_action::XDP_PASS),
    }

    let ipv4hdr: *const Ipv4Hdr = unsafe { ptr_at(&ctx, EthHdr::LEN)? };
    let ipv4_len = unsafe { (*ipv4hdr).ihl() as usize * 4 };
    let src_ip = unsafe { (*ipv4hdr).src_addr };

    // Check authorized hosts
    if let Some(expiry_ptr) = unsafe { AUTHORIZED_HOSTS.get(&src_ip) } {
        let expiry = unsafe { *expiry_ptr };
        let current_time = unsafe { aya_ebpf::helpers::bpf_ktime_get_ns() };
        if current_time < expiry {
            return Ok(xdp_action::XDP_PASS);
        } else {
            // Expired, delete it
            unsafe { AUTHORIZED_HOSTS.remove(&src_ip) };
        }
    }

    match unsafe { (*ipv4hdr).proto } {
        IpProto::Tcp => {}
        _ => return Ok(xdp_action::XDP_PASS),
    }

    let tcphdr: *const TcpHdr = unsafe { ptr_at(&ctx, EthHdr::LEN + ipv4_len)? };
    let seq = u32::from_be(unsafe { (*tcphdr).seq });
    let dst_port = u16::from_be(unsafe { (*tcphdr).dest });

    // Check against patterns (in a real scenario we'd iterate or use a combined key)
    // For simplicity, we just use seq as the key to knock_patterns map to do O(1) lookup
    if let Some(pattern_ptr) = unsafe { KNOCK_PATTERNS.get(&seq) } {
        let pattern = unsafe { *pattern_ptr };
        if pattern.expected_port == dst_port {
            // Match found! Authorize host for 1 hour
            let current_time = unsafe { aya_ebpf::helpers::bpf_ktime_get_ns() };
            let expiry = current_time + (60 * 60 * 1_000_000_000);
            
            unsafe { AUTHORIZED_HOSTS.insert(&src_ip, &expiry, 0) }.map_err(|_| ())?;

            // Send event
            if let Some(mut buf) = KNOCK_EVENTS.reserve::<KnockEvent>(0) {
                unsafe {
                    core::ptr::write(buf.as_mut_ptr(), KnockEvent {
                        src_ip,
                        port: dst_port,
                        action: pattern.action,
                        _padding: [0; 1],
                    });
                }
                buf.submit(0);
            }

            return Ok(xdp_action::XDP_DROP);
        }
    }

    Ok(xdp_action::XDP_PASS)
}
