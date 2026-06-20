#![no_std]
#![no_main]

use aya_ebpf::{
    bindings::xdp_action,
    macros::xdp,
    programs::XdpContext,
};
use core::mem;
use network_types::{
    eth::{EthHdr, EtherType},
    ip::{Ipv4Hdr, IpProto},
    udp::UdpHdr,
};

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

    match unsafe { (*ipv4hdr).proto } {
        IpProto::Udp => {}
        _ => return Ok(xdp_action::XDP_PASS),
    }

    let _udphdr: *const UdpHdr = unsafe { ptr_at(&ctx, EthHdr::LEN + ipv4_len)? };
    let payload_offset = EthHdr::LEN + ipv4_len + UdpHdr::LEN;

    // We only need to check the first 4 bytes of the payload
    let payload: *const [u8; 4] = unsafe { ptr_at(&ctx, payload_offset)? };
    
    let magic_pattern = [0xDE, 0xAD, 0xBE, 0xEF];
    
    if unsafe { *payload } == magic_pattern {
        return Ok(xdp_action::XDP_DROP);
    }

    Ok(xdp_action::XDP_PASS)
}
