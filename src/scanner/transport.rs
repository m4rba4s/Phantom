use async_trait::async_trait;
use std::io;
use std::net::SocketAddr;

#[async_trait]
pub trait PacketTransport: Send + Sync {
    async fn send_packet(&self, packet: &[u8], dst: SocketAddr) -> io::Result<usize>;
    async fn recv_packet(&self, buf: &mut [u8]) -> io::Result<usize>;
    fn transport_type(&self) -> &'static str;
    fn supports_spoofing(&self) -> bool;
}

#[cfg(unix)]
pub struct UnixTransport {
    socket: crate::scanner::raw_socket::AsyncRawSocket,
}

#[cfg(unix)]
impl UnixTransport {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            socket: crate::scanner::raw_socket::AsyncRawSocket::new()?,
        })
    }
}

#[cfg(unix)]
#[async_trait]
impl PacketTransport for UnixTransport {
    async fn send_packet(&self, packet: &[u8], dst: SocketAddr) -> io::Result<usize> {
        self.socket.send(packet, dst).await
    }

    async fn recv_packet(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.socket.recv(buf).await
    }

    fn transport_type(&self) -> &'static str {
        "AF_PACKET / RAW_SOCKET"
    }

    fn supports_spoofing(&self) -> bool {
        true // Supported via IP_HDRINCL
    }
}

#[cfg(windows)]
#[repr(C)]
pub struct WinDivertAddress {
    timestamp: i64,
    layer: u8,
    event: u8,
    sniffed: u8,
    outbound: u8,
    loopback: u8,
    impostor: u8,
    ipv6: u8,
    ip_checksum: u8,
    tcp_checksum: u8,
    udp_checksum: u8,
    reserved1: u8,
    reserved2: u8,
    reserved3: u32,
}

#[cfg(windows)]
type HANDLE = *mut std::ffi::c_void;

#[cfg(windows)]
#[link(name = "WinDivert")]
extern "system" {
    fn WinDivertOpen(filter: *const i8, layer: i32, priority: i16, flags: u64) -> HANDLE;
    fn WinDivertRecv(handle: HANDLE, packet: *mut u8, packet_len: u32, read_len: *mut u32, addr: *mut WinDivertAddress) -> i32;
    fn WinDivertSend(handle: HANDLE, packet: *const u8, packet_len: u32, write_len: *mut u32, addr: *const WinDivertAddress) -> i32;
    fn WinDivertClose(handle: HANDLE) -> i32;
}

#[cfg(windows)]
pub struct WindowsTransport {
    handle: HANDLE,
}

#[cfg(windows)]
unsafe impl Send for WindowsTransport {}
#[cfg(windows)]
unsafe impl Sync for WindowsTransport {}

#[cfg(windows)]
impl WindowsTransport {
    pub fn new() -> io::Result<Self> {
        // Simplified open call for illustrative purposes
        let filter = std::ffi::CString::new("true").unwrap();
        let handle = unsafe { WinDivertOpen(filter.as_ptr(), 0, 0, 0) };
        if handle.is_null() || handle as isize == -1 {
            return Err(io::Error::last_os_error());
        }
        Ok(Self { handle })
    }
}

#[cfg(windows)]
impl Drop for WindowsTransport {
    fn drop(&mut self) {
        unsafe { WinDivertClose(self.handle) };
    }
}

#[cfg(windows)]
#[async_trait]
impl PacketTransport for WindowsTransport {
    async fn send_packet(&self, packet: &[u8], _dst: SocketAddr) -> io::Result<usize> {
        let mut write_len = 0;
        let addr = std::mem::MaybeUninit::<WinDivertAddress>::zeroed();
        let success = unsafe {
            WinDivertSend(self.handle, packet.as_ptr(), packet.len() as u32, &mut write_len, addr.as_ptr())
        };
        if success == 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(write_len as usize)
        }
    }

    async fn recv_packet(&self, buf: &mut [u8]) -> io::Result<usize> {
        let mut read_len = 0;
        let mut addr = std::mem::MaybeUninit::<WinDivertAddress>::zeroed();
        let success = unsafe {
            WinDivertRecv(self.handle, buf.as_mut_ptr(), buf.len() as u32, &mut read_len, addr.as_mut_ptr())
        };
        if success == 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(read_len as usize)
        }
    }

    fn transport_type(&self) -> &'static str {
        "WinDivert"
    }

    fn supports_spoofing(&self) -> bool {
        true
    }
}

pub fn create_transport() -> io::Result<Box<dyn PacketTransport>> {
    #[cfg(unix)]
    {
        Ok(Box::new(UnixTransport::new()?))
    }
    #[cfg(windows)]
    {
        Ok(Box::new(WindowsTransport::new()?))
    }
    #[cfg(not(any(unix, windows)))]
    {
        Err(io::Error::new(io::ErrorKind::Unsupported, "Unsupported platform"))
    }
}
