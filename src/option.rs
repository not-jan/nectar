use crate::constants::{
    BINARY, CHARSET, ECHO, ENVIRON, GA, GMCP, LINEMODE, MCCP2, MSP, MSSP, MXP, NAWS,
    REMOTE_FLOW_CONTROL, SGA, STATUS, TELOPT_EOR, TIMING_MARK,
};

/// Represents all Telnet options supported by Nectar.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TelnetOption {
    /// Echo a message back to the other side
    Echo,
    /// Indicates that the receiver may now send data to the sender.
    ///
    /// This is used in half-duplex connections, where the sender and receiver
    /// cannot send data at the same time. If you want bi-directional data
    /// transfer, you must set `SuppressGoAhead` on the sender and receiver
    /// sides.
    GoAhead,
    /// Indicates that the communication stream will be bi-directional.
    ///
    /// This must be set on both sides of the data stream independently, even
    /// though it is expected that if one side sets it, the other side will also
    /// set it.
    ///
    /// See <https://datatracker.ietf.org/doc/html/rfc858> for more information.
    SuppressGoAhead,
    /// Indicates how EOR (End Of Record) is handled between a sender and
    /// receiver. Typically this is marked by Carrige Return and Line Feed. In
    /// systems that have a different EOR marker, this option can be used to
    /// change the EOR marker.
    ///
    /// Like SuppressGoAhead, this must be set on both sides of the data stream
    /// independently - even though it is expected that if one side sets it, the
    /// other side will also set it.
    ///
    /// See <https://datatracker.ietf.org/doc/html/rfc885> for more information.
    EndOfRecord,
    Charset,
    MCCP2,
    GMCP,
    MSSP,
    MSP,
    MXP,
    NAWS,
    Status,
    TimingMark,
    LineMode,
    RemoteFlowControl,
    Binary,
    Environ,
    /// A generic marker indicating an unknown option.
    Unknown(u8),
}

impl From<u8> for TelnetOption {
    fn from(byte: u8) -> Self {
        match byte {
            ECHO => TelnetOption::Echo,
            GA => TelnetOption::GoAhead,
            SGA => TelnetOption::SuppressGoAhead,
            TELOPT_EOR => TelnetOption::EndOfRecord,
            CHARSET => TelnetOption::Charset,
            MCCP2 => TelnetOption::MCCP2,
            GMCP => TelnetOption::GMCP,
            MSSP => TelnetOption::MSSP,
            MSP => TelnetOption::MSP,
            MXP => TelnetOption::MXP,
            NAWS => TelnetOption::NAWS,
            STATUS => TelnetOption::Status,
            TIMING_MARK => TelnetOption::TimingMark,
            LINEMODE => TelnetOption::LineMode,
            REMOTE_FLOW_CONTROL => TelnetOption::RemoteFlowControl,
            BINARY => TelnetOption::Binary,
            ENVIRON => TelnetOption::Environ,
            _ => TelnetOption::Unknown(byte),
        }
    }
}

impl From<TelnetOption> for u8 {
    fn from(option: TelnetOption) -> Self {
        match option {
            TelnetOption::Echo => ECHO,
            TelnetOption::GoAhead => GA,
            TelnetOption::SuppressGoAhead => SGA,
            TelnetOption::EndOfRecord => TELOPT_EOR,
            TelnetOption::Charset => CHARSET,
            TelnetOption::MCCP2 => MCCP2,
            TelnetOption::GMCP => GMCP,
            TelnetOption::MSSP => MSSP,
            TelnetOption::MSP => MSP,
            TelnetOption::MXP => MXP,
            TelnetOption::NAWS => NAWS,
            TelnetOption::Status => STATUS,
            TelnetOption::TimingMark => TIMING_MARK,
            TelnetOption::LineMode => LINEMODE,
            TelnetOption::RemoteFlowControl => REMOTE_FLOW_CONTROL,
            TelnetOption::Binary => BINARY,
            TelnetOption::Environ => ENVIRON,
            TelnetOption::Unknown(byte) => byte,
        }
    }
}
