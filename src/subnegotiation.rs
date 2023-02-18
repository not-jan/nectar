use bytes::Bytes;

use crate::option::TelnetOption;

/// Represents all Telnet subnegotiation events supported by Nectar.
/// See `<https://tools.ietf.org/html/rfc854>` for more information.
#[derive(Debug)]
pub enum SubnegotiationType {
    WindowSize(u16, u16),
    Unknown(TelnetOption, Bytes),
}
