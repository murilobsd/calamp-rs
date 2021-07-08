//
// Copyright (c) 2021 Murilo Ijanc' <mbsd@m0x.ru>
//
// Permission to use, copy, modify, and distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//

use nom::error::ErrorKind;
use nom::number::streaming::be_u8;
use nom::IResult;
use std::fmt;

#[derive(PartialEq, Eq)]
pub enum MessageType {
    Null,
    AckNak,
    EventReport,
    IDReport,
    UserData,
    ApplicationData,
    ConfigurationParameter,
    UnitRequest,
    LocateReport,
    UserDataWithAccumulators,
    MiniEventReport,
    MiniUserData,
    MiniApplication,
    DeviceVersion,
    ApplicationMessageWithAccumulators,
}

impl MessageType {
    pub fn parse(input: &[u8]) -> IResult<&[u8], MessageType> {
        let (i, b): (&[u8], u8) = be_u8::<_, (_, ErrorKind)>(input).unwrap();

        match b {
            0 => Ok((i, MessageType::Null)),
            1 => Ok((i, MessageType::AckNak)),
            2 => Ok((i, MessageType::EventReport)),
            3 => Ok((i, MessageType::IDReport)),
            4 => Ok((i, MessageType::UserData)),
            5 => Ok((i, MessageType::ApplicationData)),
            6 => Ok((i, MessageType::ConfigurationParameter)),
            7 => Ok((i, MessageType::UnitRequest)),
            8 => Ok((i, MessageType::LocateReport)),
            9 => Ok((i, MessageType::UserDataWithAccumulators)),
            10 => Ok((i, MessageType::MiniEventReport)),
            11 => Ok((i, MessageType::MiniUserData)),
            12 => Ok((i, MessageType::MiniApplication)),
            13 => Ok((i, MessageType::DeviceVersion)),
            14 => Ok((i, MessageType::ApplicationMessageWithAccumulators)),
            _ => panic!("not found"),
        }
    }
}

impl fmt::Display for MessageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            MessageType::Null => write!(f, "MessageType::Null"),
            MessageType::AckNak => write!(f, "MessageType::AckNak"),
            MessageType::EventReport => write!(f, "MessageType::EventReport"),
            MessageType::IDReport => write!(f, "MessageType::IDReport"),
            MessageType::UserData => write!(f, "MessageType::UserData"),
            MessageType::ApplicationData => {
                write!(f, "MessageType::ApplicationData")
            }
            MessageType::ConfigurationParameter => {
                write!(f, "MessageType::ConfigurationParameter")
            }
            MessageType::UnitRequest => write!(f, "MessageType::UnitRequest"),
            MessageType::LocateReport => write!(f, "MessageType::LocateReport"),
            MessageType::UserDataWithAccumulators => {
                write!(f, "MessageType::UserDataWithAccumulators")
            }
            MessageType::MiniEventReport => {
                write!(f, "MessageType::MiniEventReport")
            }
            MessageType::MiniUserData => write!(f, "MessageType::MiniUserData"),
            MessageType::MiniApplication => {
                write!(f, "MessageType::MiniApplication")
            }
            MessageType::DeviceVersion => {
                write!(f, "MessageType::DeviceVersion")
            }
            MessageType::ApplicationMessageWithAccumulators => {
                write!(f, "MessageType::ApplicationMessageWithAccumulators")
            }
        }
    }
}

impl fmt::Debug for MessageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            MessageType::Null => write!(f, "MessageType::Null"),
            MessageType::AckNak => write!(f, "MessageType::AckNak"),
            MessageType::EventReport => write!(f, "MessageType::EventReport"),
            MessageType::IDReport => write!(f, "MessageType::IDReport"),
            MessageType::UserData => write!(f, "MessageType::UserData"),
            MessageType::ApplicationData => {
                write!(f, "MessageType::ApplicationData")
            }
            MessageType::ConfigurationParameter => {
                write!(f, "MessageType::ConfigurationParameter")
            }
            MessageType::UnitRequest => write!(f, "MessageType::UnitRequest"),
            MessageType::LocateReport => write!(f, "MessageType::LocateReport"),
            MessageType::UserDataWithAccumulators => {
                write!(f, "MessageType::UserDataWithAccumulators")
            }
            MessageType::MiniEventReport => {
                write!(f, "MessageType::MiniEventReport")
            }
            MessageType::MiniUserData => write!(f, "MessageType::MiniUserData"),
            MessageType::MiniApplication => {
                write!(f, "MessageType::MiniApplication")
            }
            MessageType::DeviceVersion => {
                write!(f, "MessageType::DeviceVersion")
            }
            MessageType::ApplicationMessageWithAccumulators => {
                write!(f, "MessageType::ApplicationMessageWithAccumulators")
            }
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum ServiceType {
    /// Unacknowledged Request
    Unacknowledged,
    /// Acknowledged Request
    Acknowledged,
    /// Response to an Acknowledged Request
    ResponseToAnAcknowledged,
}

impl ServiceType {
    pub fn parse(input: &[u8]) -> IResult<&[u8], ServiceType> {
        let (i, b): (&[u8], u8) = be_u8::<_, (_, ErrorKind)>(input).unwrap();

        match b {
            0 => Ok((i, ServiceType::Unacknowledged)),
            1 => Ok((i, ServiceType::Acknowledged)),
            2 => Ok((i, ServiceType::ResponseToAnAcknowledged)),
            _ => panic!("not found"),
        }
    }
}

impl fmt::Display for ServiceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ServiceType::Unacknowledged => {
                write!(f, "ServiceType::Unacknowledged")
            }
            ServiceType::Acknowledged => write!(f, "ServiceType::Acknowledged"),
            ServiceType::ResponseToAnAcknowledged => {
                write!(f, "ServiceType::ResponseToAnAcknowledged")
            }
        }
    }
}

impl fmt::Debug for ServiceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ServiceType::Unacknowledged => {
                write!(f, "ServiceType::Unacknowledged")
            }
            ServiceType::Acknowledged => write!(f, "ServiceType::Acknowledged"),
            ServiceType::ResponseToAnAcknowledged => {
                write!(f, "ServiceType::ResponseToAnAcknowledged")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{MessageType, ServiceType};
    use crate::options_header::{MobileIDType, OptionsHeader};

    #[test]
    fn test_parse_message_headers() {
        let data: [u8; 117] = [
            0x83, 0x05, 0x46, 0x34, 0x66, 0x32, 0x35, 0x01, 0x01, 0x01, 0x02,
            0x3a, 0x86, 0x5f, 0xf1, 0x3a, 0x54, 0x5f, 0xf1, 0x3a, 0x57, 0xf1,
            0xe2, 0x85, 0x78, 0xe4, 0x22, 0xd6, 0x40, 0x00, 0x01, 0x36, 0xf8,
            0x00, 0x00, 0x00, 0x0b, 0x00, 0x00, 0x06, 0x20, 0x00, 0x00, 0xff,
            0x8d, 0x02, 0x1e, 0x1e, 0x00, 0x7b, 0x21, 0x10, 0x00, 0x00, 0x00,
            0x31, 0xe0, 0x00, 0x00, 0x10, 0x1a, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x22, 0x2a, 0x32, 0x00, 0x00, 0x03, 0xf1, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x01, 0xc8, 0x2d, 0x3f, 0x01, 0xc8, 0x2d,
            0x3f, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x01, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        let (i, _) = OptionsHeader::parse(&data).unwrap();
        let (i, service_type) = ServiceType::parse(i).unwrap();
        assert_eq!(service_type, ServiceType::Acknowledged);
        assert_eq!(
            format!("{}", service_type),
            String::from("ServiceType::Acknowledged")
        );

        let (i, message_type) = MessageType::parse(i).unwrap();
        assert_eq!(message_type, MessageType::EventReport);
        assert_eq!(
            format!("{}", message_type),
            String::from("MessageType::EventReport")
        );
    }
}
