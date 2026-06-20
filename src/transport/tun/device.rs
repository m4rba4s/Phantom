use crate::transport::tun::AsyncTunDevice;
use async_trait::async_trait;
use std::io::{Error, ErrorKind, Result};

#[cfg(target_os = "linux")]
mod os {
    use super::*;
    use std::fs::OpenOptions;
    use std::os::unix::fs::OpenOptionsExt;
    use std::os::unix::io::{AsRawFd, RawFd};
    use tokio::io::unix::AsyncFd;
    use libc;

    const TUNSETIFF: libc::c_ulong = 0x400454ca;
    const IFF_TUN: libc::c_short = 0x0001;
    const IFF_NO_PI: libc::c_short = 0x1000;

    pub struct NativeTunDevice {
        fd: AsyncFd<std::fs::File>,
        pub name: String,
    }

    impl NativeTunDevice {
        pub fn new(name: &str) -> Result<Self> {
            let file = OpenOptions::new()
                .read(true)
                .write(true)
                .custom_flags(libc::O_NONBLOCK)
                .open("/dev/net/tun")?;

            let mut ifr: libc::ifreq = unsafe { std::mem::zeroed() };
            ifr.ifr_ifru.ifru_flags = IFF_TUN | IFF_NO_PI;

            let bytes = name.as_bytes();
            if bytes.len() >= libc::IFNAMSIZ {
                return Err(Error::new(ErrorKind::InvalidInput, "TUN name too long"));
            }
            for (i, &b) in bytes.iter().enumerate() {
                ifr.ifr_name[i] = b as libc::c_char;
            }

            let res = unsafe { libc::ioctl(file.as_raw_fd(), TUNSETIFF, &ifr as *const _ as *mut libc::c_void) };
            if res < 0 {
                return Err(Error::last_os_error());
            }

            // Extract the actual assigned name
            let mut name_buf = Vec::new();
            for &c in &ifr.ifr_name {
                if c == 0 { break; }
                name_buf.push(c as u8);
            }
            let actual_name = String::from_utf8_lossy(&name_buf).into_owned();

            let async_fd = AsyncFd::new(file)?;

            Ok(Self {
                fd: async_fd,
                name: actual_name,
            })
        }
    }

    #[async_trait]
    impl AsyncTunDevice for NativeTunDevice {
        async fn read_packet(&self, buf: &mut [u8]) -> Result<usize> {
            loop {
                let mut guard = self.fd.readable().await?;
                let fd = self.fd.get_ref().as_raw_fd();
                let res = unsafe { libc::read(fd, buf.as_mut_ptr() as *mut libc::c_void, buf.len()) };
                if res < 0 {
                    let err = Error::last_os_error();
                    if err.kind() == ErrorKind::WouldBlock {
                        guard.clear_ready();
                        continue;
                    }
                    return Err(err);
                }
                return Ok(res as usize);
            }
        }

        async fn write_packet(&self, buf: &[u8]) -> Result<usize> {
            loop {
                let mut guard = self.fd.writable().await?;
                let fd = self.fd.get_ref().as_raw_fd();
                let res = unsafe { libc::write(fd, buf.as_ptr() as *const libc::c_void, buf.len()) };
                if res < 0 {
                    let err = Error::last_os_error();
                    if err.kind() == ErrorKind::WouldBlock {
                        guard.clear_ready();
                        continue;
                    }
                    return Err(err);
                }
                return Ok(res as usize);
            }
        }
    }
}

#[cfg(not(target_os = "linux"))]
mod os {
    use super::*;

    pub struct NativeTunDevice {
        pub name: String,
    }

    impl NativeTunDevice {
        pub fn new(name: &str) -> Result<Self> {
            Err(Error::new(ErrorKind::Unsupported, "TUN is only supported on Linux in this version"))
        }
    }

    #[async_trait]
    impl AsyncTunDevice for NativeTunDevice {
        async fn read_packet(&self, _buf: &mut [u8]) -> Result<usize> {
            Err(Error::new(ErrorKind::Unsupported, "TUN not supported"))
        }

        async fn write_packet(&self, _buf: &[u8]) -> Result<usize> {
            Err(Error::new(ErrorKind::Unsupported, "TUN not supported"))
        }
    }
}

pub use os::*;

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};
    
    // Checks if the process is running as root / has CAP_NET_ADMIN.
    // We use a simple heuristics: can we open /dev/net/tun?
    fn can_create_tun() -> bool {
        #[cfg(target_os = "linux")]
        {
            let file = std::fs::OpenOptions::new()
                .read(true)
                .write(true)
                .open("/dev/net/tun");
            file.is_ok()
        }
        #[cfg(not(target_os = "linux"))]
        {
            false
        }
    }

    #[tokio::test]
    async fn test_tun_creation_if_permitted() {
        if !can_create_tun() {
            println!("Skipping TUN creation test due to lack of permissions.");
            return;
        }

        let tun_res = NativeTunDevice::new("phantom_test0");
        match tun_res {
            Ok(tun) => {
                assert!(tun.name.starts_with("phantom_test"), "TUN device name mismatch");
            }
            Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied || e.raw_os_error() == Some(libc::EPERM) => {
                println!("Skipping TUN creation test due to lack of CAP_NET_ADMIN permissions. Error: {}", e);
            }
            Err(e) => {
                panic!("Failed to create TUN device with unexpected error: {}", e);
            }
        }
    }
}
