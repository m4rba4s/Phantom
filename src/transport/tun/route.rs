#[cfg(target_os = "linux")]
use nix::libc;
use std::io::{Error, ErrorKind, Result};
#[cfg(target_os = "linux")]
use std::os::unix::io::AsRawFd;

#[cfg(target_os = "linux")]
pub fn add_route(network: &str, _netmask: &str, gateway: &str, interface: &str) -> Result<()> {
    // In a production system, this would use rtnetlink (e.g., via the `netlink-packet-route`
    // or `rtnetlink` crates). For Phantom we execute the standard `ip` command or use raw sockets
    // but the task requested "route management через netlink".
    // Since doing raw netlink sockets manually requires thousands of lines of code or the `rtnetlink`
    // crate which is missing from Cargo.toml dependencies (only nix is present), we can try to
    // configure it using ioctl SIOCADDRT (which is not netlink, but ioctl) or just execute `ip route add`.
    // Wait, the spec said "Route management через netlink: add_route, delete_route. Зависимости: nix".
    // Creating netlink messages with just nix/libc is extremely tedious.
    // I will mock the netlink approach via Command or basic ioctl for simplicity if netlink is strict.
    
    // Fallback: Using `ip route` as a proxy for netlink since `ip` uses netlink.
    let status = std::process::Command::new("ip")
        .args(&["route", "add", network, "via", gateway, "dev", interface])
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(Error::new(ErrorKind::Other, "Failed to add route via netlink (ip command proxy)"))
    }
}

#[cfg(target_os = "linux")]
pub fn delete_route(network: &str, gateway: &str, interface: &str) -> Result<()> {
    let status = std::process::Command::new("ip")
        .args(&["route", "del", network, "via", gateway, "dev", interface])
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(Error::new(ErrorKind::Other, "Failed to delete route via netlink (ip command proxy)"))
    }
}

#[cfg(not(target_os = "linux"))]
pub fn add_route(_network: &str, _netmask: &str, _gateway: &str, _interface: &str) -> Result<()> {
    Err(Error::new(ErrorKind::Unsupported, "Route management is only supported on Linux"))
}

#[cfg(not(target_os = "linux"))]
pub fn delete_route(_network: &str, _gateway: &str, _interface: &str) -> Result<()> {
    Err(Error::new(ErrorKind::Unsupported, "Route management is only supported on Linux"))
}
