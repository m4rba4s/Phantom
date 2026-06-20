#[cfg(target_os = "linux")]
use libc::{
    c_int, c_void, setsockopt, socket, AF_PACKET, ETH_P_ALL, PACKET_RX_RING, SOCK_RAW,
    SOL_PACKET, tpacket_req3,
};
#[cfg(target_os = "linux")]
use std::io;
#[cfg(target_os = "linux")]
use std::os::unix::io::{AsRawFd, FromRawFd, OwnedFd};
#[cfg(target_os = "linux")]
use tokio::io::unix::AsyncFd;

#[cfg(target_os = "linux")]
pub struct AfPacketReceiver {
    io: AsyncFd<OwnedFd>,
    // we would also map memory here for ring buffer, but keeping it minimal for architecture
}

#[cfg(target_os = "linux")]
impl AfPacketReceiver {
    pub fn new() -> io::Result<Self> {
        unsafe {
            // Create raw packet socket
            let fd = socket(AF_PACKET, SOCK_RAW, (ETH_P_ALL as u16).to_be() as i32);
            if fd < 0 {
                return Err(io::Error::last_os_error());
            }

            // Set up TPACKET_V3
            let version: c_int = libc::tpacket_versions::TPACKET_V3 as libc::c_int;
            if setsockopt(
                fd,
                SOL_PACKET,
                libc::PACKET_VERSION,
                &version as *const _ as *const c_void,
                std::mem::size_of_val(&version) as libc::socklen_t,
            ) < 0
            {
                let err = io::Error::last_os_error();
                libc::close(fd);
                return Err(err);
            }

            // Configure RX ring
            let req = tpacket_req3 {
                tp_block_size: 4096 * 8, // 32KB
                tp_block_nr: 256,
                tp_frame_size: 2048,
                tp_frame_nr: 4096,
                tp_retire_blk_tov: 10,
                tp_sizeof_priv: 0,
                tp_feature_req_word: 0,
            };

            if setsockopt(
                fd,
                SOL_PACKET,
                PACKET_RX_RING,
                &req as *const _ as *const c_void,
                std::mem::size_of_val(&req) as libc::socklen_t,
            ) < 0
            {
                let err = io::Error::last_os_error();
                libc::close(fd);
                return Err(err);
            }

            // Convert to OwnedFd and wrap in AsyncFd
            let owned_fd = OwnedFd::from_raw_fd(fd);

            // Set non-blocking
            let flags = libc::fcntl(owned_fd.as_raw_fd(), libc::F_GETFL);
            if flags < 0 {
                return Err(io::Error::last_os_error());
            }
            if libc::fcntl(owned_fd.as_raw_fd(), libc::F_SETFL, flags | libc::O_NONBLOCK) < 0 {
                return Err(io::Error::last_os_error());
            }

            Ok(Self {
                io: AsyncFd::new(owned_fd)?,
            })
        }
    }

    pub async fn wait_readable(&self) -> io::Result<()> {
        let mut guard = self.io.readable().await?;
        guard.clear_ready();
        Ok(())
    }
}
