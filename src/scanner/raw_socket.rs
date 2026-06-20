#[cfg(unix)]
use std::io;
#[cfg(unix)]
use std::net::SocketAddr;
#[cfg(unix)]
use std::os::unix::io::AsRawFd;
#[cfg(unix)]
use socket2::{Domain, Protocol, Socket as Socket2, Type};
#[cfg(unix)]
use tokio::io::unix::AsyncFd;

#[cfg(unix)]
pub struct AsyncRawSocket {
    io: AsyncFd<Socket2>,
}

#[cfg(unix)]
impl AsyncRawSocket {
    pub fn new() -> io::Result<Self> {
        let socket = Socket2::new(
            Domain::IPV4,
            Type::RAW,
            Some(Protocol::from(libc::IPPROTO_TCP)),
        )?;
        
        socket.set_nonblocking(true)?;
        
        // Ensure IP_HDRINCL is set
        #[cfg(target_os = "linux")]
        {
            let fd = socket.as_raw_fd();
            let optval: libc::c_int = 1;
            let ret = unsafe {
                libc::setsockopt(
                    fd,
                    libc::IPPROTO_IP,
                    libc::IP_HDRINCL,
                    &optval as *const _ as *const libc::c_void,
                    std::mem::size_of_val(&optval) as libc::socklen_t,
                )
            };
            if ret < 0 {
                return Err(io::Error::last_os_error());
            }
        }
        
        Ok(Self {
            io: AsyncFd::new(socket)?,
        })
    }

    pub async fn send(&self, packet: &[u8], dst: SocketAddr) -> io::Result<usize> {
        loop {
            let mut guard = self.io.writable().await?;
            
            match guard.try_io(|inner| inner.get_ref().send_to(packet, &dst.into())) {
                Ok(result) => return result,
                Err(_would_block) => continue,
            }
        }
    }

    pub async fn recv(&self, buf: &mut [u8]) -> io::Result<usize> {
        loop {
            let mut guard = self.io.readable().await?;
            
            match guard.try_io(|inner| {
                inner.get_ref().recv(unsafe {
                    &mut *(buf as *mut [u8] as *mut [std::mem::MaybeUninit<u8>])
                })
            }) {
                Ok(result) => return result,
                Err(_would_block) => continue,
            }
        }
    }

    #[cfg(target_os = "linux")]
    pub fn set_pacing_rate(&self, bytes_per_sec: u64) -> io::Result<()> {
        let fd = self.io.get_ref().as_raw_fd();
        let rate: u32 = if bytes_per_sec > u32::MAX as u64 {
            u32::MAX
        } else {
            bytes_per_sec as u32
        };
        
        let ret = unsafe {
            libc::setsockopt(
                fd,
                libc::SOL_SOCKET,
                libc::SO_MAX_PACING_RATE,
                &rate as *const _ as *const libc::c_void,
                std::mem::size_of_val(&rate) as libc::socklen_t,
            )
        };
        
        if ret < 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(())
        }
    }
}
