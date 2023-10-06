use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    net::SocketAddr,
    num::ParseIntError,
    str::FromStr,
};
use thiserror::Error;

pub enum HostPortPair {
    SocketAddress(SocketAddr),
    DomainAddress(String, u16),
}

impl HostPortPair {
    pub fn port(&self) -> u16 {
        match self {
            HostPortPair::SocketAddress(addr) => addr.port(),
            HostPortPair::DomainAddress(_, port) => *port,
        }
    }

    pub fn is_ip_address(&self) -> bool {
        matches!(self, HostPortPair::SocketAddress(_))
    }

    pub fn is_domain_address(&self) -> bool {
        matches!(self, HostPortPair::DomainAddress(_, _))
    }
}

impl From<SocketAddr> for HostPortPair {
    fn from(addr: SocketAddr) -> Self {
        HostPortPair::SocketAddress(addr)
    }
}

impl TryFrom<(String, u16)> for HostPortPair {
    type Error = HostPortPairError;

    fn try_from((domain, port): (String, u16)) -> Result<Self, Self::Error> {
        check_domain(&domain)?;
        Ok(HostPortPair::DomainAddress(domain, port))
    }
}

impl TryFrom<String> for HostPortPair {
    type Error = HostPortPairError;

    fn try_from(mut s: String) -> Result<Self, Self::Error> {
        if let Ok(addr) = s.parse() {
            return Ok(HostPortPair::SocketAddress(addr));
        }

        let Some((domain, port)) = s.rsplit_once(':') else {
            return Err(HostPortPairError::NoPort);
        };

        check_domain(domain)?;
        let port = port.parse()?;
        s.truncate(domain.len());

        Ok(HostPortPair::DomainAddress(s, port))
    }
}

impl TryFrom<(&str, u16)> for HostPortPair {
    type Error = HostPortPairError;

    fn try_from((domain, port): (&str, u16)) -> Result<Self, Self::Error> {
        check_domain(domain)?;
        Ok(HostPortPair::DomainAddress(domain.to_owned(), port))
    }
}

impl TryFrom<&str> for HostPortPair {
    type Error = HostPortPairError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if let Ok(addr) = s.parse() {
            return Ok(HostPortPair::SocketAddress(addr));
        }

        let Some((domain, port)) = s.rsplit_once(':') else {
            return Err(HostPortPairError::NoPort);
        };

        check_domain(domain)?;
        let port = port.parse()?;

        Ok(HostPortPair::DomainAddress(domain.to_owned(), port))
    }
}

impl Display for HostPortPair {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            HostPortPair::SocketAddress(addr) => write!(f, "{addr}"),
            HostPortPair::DomainAddress(domain, port) => write!(f, "{domain}:{port}"),
        }
    }
}

impl FromStr for HostPortPair {
    type Err = HostPortPairError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(addr) = s.parse() {
            return Ok(HostPortPair::SocketAddress(addr));
        }

        let Some((domain, port)) = s.rsplit_once(':') else {
            return Err(HostPortPairError::NoPort);
        };

        check_domain(domain)?;
        let port = port.parse()?;

        Ok(HostPortPair::DomainAddress(domain.to_owned(), port))
    }
}

#[derive(Debug, Error)]
pub enum HostPortPairError {
    #[error("no port provided")]
    NoPort,
    #[error("invalid character in domain at position {0}: {1}")]
    InvalidCharacterInDomain(usize, char),
    #[error("invalid port: {0}")]
    ParsePort(#[from] ParseIntError),
}

fn check_domain(domain: &str) -> Result<(), HostPortPairError> {
    for (idx, b) in domain.as_bytes().iter().enumerate() {
        if !matches!(b, b'0'..=b'9' | b'a'..=b'z' | b'A'..=b'Z' | b'-' | b'.' | b'_') {
            return Err(HostPortPairError::InvalidCharacterInDomain(idx, *b as char));
        }
    }

    Ok(())
}

#[cfg(feature = "serde")]
mod serde_impls {
    use super::*;
    use serde::{de::Error as DeError, Deserialize, Deserializer, Serialize, Serializer};

    impl Serialize for HostPortPair {
        fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            ser.collect_str(self)
        }
    }

    impl<'de> Deserialize<'de> for HostPortPair {
        fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
            let s = String::deserialize(de)?;
            s.parse().map_err(DeError::custom)
        }
    }
}
