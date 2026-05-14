# host-port-pair

Small Rust types for representing a host and port together.

[![Version](https://img.shields.io/crates/v/host-port-pair.svg?style=flat)](https://crates.io/crates/host-port-pair)
[![Documentation](https://img.shields.io/badge/docs-release-brightgreen.svg?style=flat)](https://docs.rs/host-port-pair)
[![License](https://img.shields.io/crates/l/host-port-pair.svg?style=flat)](https://github.com/EAimTY/host-port-pair/blob/master/LICENSE)

`host-port-pair` keeps the host as either an IP address or a DNS name and keeps
the port as a `u16`. It is useful when code needs a lightweight owned
`host:port` value without committing to `std::net::SocketAddr`, which only
represents IP socket addresses.

## Install

```sh
cargo add host-port-pair
```

## Example

```rust
use host_port_pair::{Host, HostPortPair};

let pair: HostPortPair = "example.com:443".parse()?;

assert_eq!(pair.host(), &Host::DnsName("example.com".to_owned()));
assert_eq!(pair.port(), 443);
assert_eq!(pair.to_string(), "example.com:443");
# Ok::<(), host_port_pair::HostPortPairError>(())
```

IP hosts are detected automatically:

```rust
use std::net::Ipv4Addr;

use host_port_pair::{Host, HostPortPair};

let pair = HostPortPair::from((Ipv4Addr::LOCALHOST, 8080));

assert_eq!(pair.host(), &Host::from(Ipv4Addr::LOCALHOST));
assert!(pair.host().is_ip_address());
assert_eq!(pair.port(), 8080);
```

## Types

- `HostPortPair` stores a `Host` and a `u16` port.
- `Host` is either `Host::IpAddr(std::net::IpAddr)` or
  `Host::DnsName(String)`.
- `HostPortPairError` reports parse failures when a port is missing or invalid.

`HostPortPair` and `Host` both implement `Debug`, `Clone`, `PartialEq`, `Eq`,
and `Hash`.

## Parsing And Formatting

`HostPortPair` implements `FromStr`, `TryFrom<&str>`, `TryFrom<&String>`, and
`TryFrom<String>`.

Parsing splits the input at the final `:`:

```rust
use host_port_pair::{Host, HostPortPair};

let pair = HostPortPair::try_from("127.0.0.1:3000")?;

assert_eq!(pair.host(), &Host::from("127.0.0.1"));
assert_eq!(pair.port(), 3000);
# Ok::<(), host_port_pair::HostPortPairError>(())
```

The text before the final `:` is parsed as an IP address when possible. If it
does not parse as an IP address, it is stored as `Host::DnsName` without DNS
syntax validation.

Formatting writes the value back as `<host>:<port>`:

```rust
use host_port_pair::HostPortPair;

let pair = HostPortPair::from(("localhost", 8080));

assert_eq!(pair.to_string(), "localhost:8080");
```

IPv6 addresses are formatted without square brackets:

```rust
use std::net::Ipv6Addr;

use host_port_pair::HostPortPair;

let pair = HostPortPair::from((Ipv6Addr::LOCALHOST, 8080));

assert_eq!(pair.to_string(), "::1:8080");
```

## Conversions

Create a pair from any host value that converts into `Host`:

```rust
use std::net::{IpAddr, Ipv4Addr};

use host_port_pair::{Host, HostPortPair};

let from_dns = HostPortPair::from(("example.com", 443));
let from_string = HostPortPair::from(("example.com".to_owned(), 443));
let from_ip = HostPortPair::from((IpAddr::V4(Ipv4Addr::LOCALHOST), 8080));

assert_eq!(from_dns.host(), &Host::DnsName("example.com".to_owned()));
assert_eq!(from_string.port(), 443);
assert!(from_ip.host().is_ip_address());
```

`HostPortPair` can also be created from standard socket address types:

```rust
use std::net::SocketAddr;

use host_port_pair::{Host, HostPortPair};

let socket = SocketAddr::from(([127, 0, 0, 1], 3000));
let pair = HostPortPair::from(socket);

assert_eq!(pair.host(), &Host::from("127.0.0.1"));
assert_eq!(pair.port(), 3000);
```

Convert a pair back into its owned parts with `Into<(Host, u16)>`:

```rust
use host_port_pair::{Host, HostPortPair};

let pair = HostPortPair::from(("example.com", 443));
let (host, port): (Host, u16) = pair.into();

assert_eq!(host, Host::DnsName("example.com".to_owned()));
assert_eq!(port, 443);
```

## Optional Features

This crate has no default optional features.

- `serde`: serializes `Host` and `HostPortPair` as strings and deserializes
  them from strings.
- `rkyv`: derives `Archive`, `Serialize`, and `Deserialize` for `Host` and
  `HostPortPair`. Archived types and resolvers are re-exported from
  `host_port_pair::rkyv`.

## License

MIT License
