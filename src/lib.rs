#![doc = include_str!("../README.md")]

use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6},
    num::ParseIntError,
    str::FromStr,
};
use thiserror::Error;

pub use crate::host_port_pair::{Host, HostPortPair};

mod host_port_pair {
    use std::net::IpAddr;

    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize),
        rkyv(derive(Debug, Hash), compare(PartialEq))
    )]
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct HostPortPair {
        pub(crate) host: Host,
        pub(crate) port: u16,
    }

    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize),
        rkyv(derive(Debug, Hash), compare(PartialEq))
    )]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum Host {
        IpAddr(IpAddr),
        DnsName(String),
    }
}

#[derive(Debug, Error)]
pub enum HostPortPairError {
    #[error("no port")]
    NoPort,
    #[error("invalid port: {0}")]
    ParsePort(#[from] ParseIntError),
}

impl HostPortPair {
    pub fn host(&self) -> &Host {
        &self.host
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn host_mut(&mut self) -> &mut Host {
        &mut self.host
    }

    pub fn port_mut(&mut self) -> &mut u16 {
        &mut self.port
    }
}

impl Host {
    pub fn is_ip_address(&self) -> bool {
        matches!(self, Host::IpAddr(_))
    }

    pub fn is_dns_name(&self) -> bool {
        matches!(self, Host::DnsName(_))
    }
}

impl From<IpAddr> for Host {
    fn from(ip: IpAddr) -> Self {
        Host::IpAddr(ip)
    }
}

impl From<Ipv4Addr> for Host {
    fn from(ip: Ipv4Addr) -> Self {
        Host::IpAddr(IpAddr::V4(ip))
    }
}

impl From<Ipv6Addr> for Host {
    fn from(ip: Ipv6Addr) -> Self {
        Host::IpAddr(IpAddr::V6(ip))
    }
}

impl From<String> for Host {
    fn from(host: String) -> Self {
        match host.parse() {
            Ok(ip) => Host::IpAddr(ip),
            Err(_) => Host::DnsName(host),
        }
    }
}

impl From<&String> for Host {
    fn from(host: &String) -> Self {
        match host.parse() {
            Ok(ip) => Host::IpAddr(ip),
            Err(_) => Host::DnsName(host.clone()),
        }
    }
}

impl From<&str> for Host {
    fn from(host: &str) -> Self {
        match host.parse() {
            Ok(ip) => Host::IpAddr(ip),
            Err(_) => Host::DnsName(host.to_owned()),
        }
    }
}

impl<T: Into<Host>> From<(T, u16)> for HostPortPair {
    fn from((host, port): (T, u16)) -> Self {
        HostPortPair {
            host: host.into(),
            port,
        }
    }
}

impl From<SocketAddr> for HostPortPair {
    fn from(addr: SocketAddr) -> Self {
        HostPortPair {
            host: Host::IpAddr(addr.ip()),
            port: addr.port(),
        }
    }
}

impl From<SocketAddrV4> for HostPortPair {
    fn from(addr: SocketAddrV4) -> Self {
        HostPortPair {
            host: Host::from(*addr.ip()),
            port: addr.port(),
        }
    }
}

impl From<SocketAddrV6> for HostPortPair {
    fn from(addr: SocketAddrV6) -> Self {
        HostPortPair {
            host: Host::from(*addr.ip()),
            port: addr.port(),
        }
    }
}

impl TryFrom<String> for HostPortPair {
    type Error = HostPortPairError;

    fn try_from(mut s: String) -> Result<Self, Self::Error> {
        let Some((host, port)) = s.rsplit_once(':') else {
            return Err(HostPortPairError::NoPort);
        };

        let port = port.parse()?;
        s.truncate(host.len());

        Ok(HostPortPair {
            host: Host::from(s),
            port,
        })
    }
}

impl TryFrom<&String> for HostPortPair {
    type Error = HostPortPairError;

    fn try_from(s: &String) -> Result<Self, Self::Error> {
        let Some((host, port)) = s.rsplit_once(':') else {
            return Err(HostPortPairError::NoPort);
        };

        let port = port.parse()?;

        Ok(HostPortPair {
            host: host.into(),
            port,
        })
    }
}

impl TryFrom<&str> for HostPortPair {
    type Error = HostPortPairError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let Some((host, port)) = s.rsplit_once(':') else {
            return Err(HostPortPairError::NoPort);
        };

        let port = port.parse()?;

        Ok(HostPortPair {
            host: host.into(),
            port,
        })
    }
}

impl FromStr for HostPortPair {
    type Err = HostPortPairError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

impl Display for Host {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Host::IpAddr(ip) => write!(f, "{ip}"),
            Host::DnsName(name) => write!(f, "{name}"),
        }
    }
}

impl Display for HostPortPair {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}:{}", self.host, self.port)
    }
}

#[cfg(feature = "serde")]
mod serde {
    use super::*;
    use ::serde::{de::Error as DeError, Deserialize, Deserializer, Serialize, Serializer};

    impl Serialize for HostPortPair {
        fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            ser.collect_str(self)
        }
    }

    impl<'de> Deserialize<'de> for HostPortPair {
        fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
            let s = String::deserialize(de)?;
            Self::try_from(s).map_err(DeError::custom)
        }
    }
}

#[cfg(feature = "rkyv")]
pub mod rkyv {
    pub use crate::host_port_pair::{
        ArchivedHost, ArchivedHostPortPair, HostPortPairResolver, HostResolver,
    };
}
