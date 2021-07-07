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

//! Calamp LMDirect message parser.

use nom::bits::{bits, streaming};
use nom::bytes;
use nom::error::{Error, ErrorKind};
use nom::number::complete::{be_u16, be_u8};
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug)]
pub struct MobileID<'a>(&'a [u8]);

impl<'a> MobileID<'a> {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn parse(input: &'a [u8]) -> IResult<&[u8], Self> {
        let (i, a): (&[u8], u8) = be_u8::<_, (_, ErrorKind)>(input).unwrap();
        let (i, b): (&[u8], &[u8]) = bytes::streaming::take(a)(i)?;
        Ok((i, Self(b)))
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<'a> ToString for MobileID<'a> {
    fn to_string(&self) -> String {
        let mut id = String::from("");
        for d in self.0.iter() {
            id.push_str(&format!("{0:2x}", d))
        }
        id
    }
}

pub fn parse_service_type(input: &[u8]) -> IResult<&[u8], u8> {
    let (i, a): (&[u8], u8) = be_u8::<_, (_, ErrorKind)>(input).unwrap();
    Ok((i, a))
}

pub fn parse_message_type(input: &[u8]) -> IResult<&[u8], u8> {
    let (i, a): (&[u8], u8) = be_u8::<_, (_, ErrorKind)>(input).unwrap();
    Ok((i, a))
}

pub fn parse_sequence_number(input: &[u8]) -> IResult<&[u8], u16> {
    let (i, a): (&[u8], u16) = be_u16::<_, (_, ErrorKind)>(input).unwrap();
    Ok((i, a))
}

#[derive(Debug, PartialEq, Eq)]
pub enum MobileIDType {
    Off,
    /// Electronic Serial Number (ESN) of the LMU
    Esn,
    /// International Mobile Equipment Identifier (IMEI) or Electronic Identifier (EID) of the wireless modem
    Equipment,
    /// International Mobile Subscriber Identifier (IMSI) of the SIM card (GSM/GPRS devices only)
    Subscriber,
    /// User Defined Mobile ID
    Defined,
    /// Phone Number of the mobile (if available)
    PhoneNumber,
    /// The current IP Address of the LMU
    IpAddress,
    /// CDMA Mobile Equipment ID (MEID) or International Mobile Equipment Identifier (IMEI) of the wireless modem
    Cdma,
}

// FIX: is necessary call multiple be_u8
pub fn parse_mobile_id_type(input: &[u8]) -> IResult<&[u8], MobileIDType> {
    let (i, _): (&[u8], u8) = be_u8::<_, (_, ErrorKind)>(input).unwrap();
    let (i, b): (&[u8], u8) = be_u8::<_, (_, ErrorKind)>(i).unwrap();
    match b {
        0 => Ok((i, MobileIDType::Off)),
        1 => Ok((i, MobileIDType::Esn)),
        2 => Ok((i, MobileIDType::Equipment)),
        3 => Ok((i, MobileIDType::Subscriber)),
        4 => Ok((i, MobileIDType::Defined)),
        5 => Ok((i, MobileIDType::PhoneNumber)),
        6 => Ok((i, MobileIDType::IpAddress)),
        7 => Ok((i, MobileIDType::Cdma)),
        _ => panic!("not found"),
    }
}

#[derive(Debug)]
pub struct OptionsStatus {
    /// MobileID is set
    is_mobile_id: bool,
    /// MobileIdType is set
    is_mobile_id_type: bool,
    /// Authentication World is set
    is_authentication_world: bool,
    /// Routing is set
    is_routing: bool,
    /// Forwarding is set
    is_forwarding: bool,
    /// Response redirections is set
    is_response_redirection: bool,
    /// Options extension is set
    is_options_extension: bool,
    /// Options headers exist
    is_always_set: bool,
}

impl OptionsStatus {
    pub fn is_mobile_id(&self) -> bool {
        self.is_mobile_id
    }

    pub fn is_mobile_id_type(&self) -> bool {
        self.is_mobile_id_type
    }

    pub fn is_authentication_world(&self) -> bool {
        self.is_authentication_world
    }

    pub fn is_routing(&self) -> bool {
        self.is_routing
    }

    pub fn is_forwarding(&self) -> bool {
        self.is_forwarding
    }

    pub fn is_response_redirection(&self) -> bool {
        self.is_response_redirection
    }

    pub fn is_options_extension(&self) -> bool {
        self.is_options_extension
    }

    pub fn is_always_set(&self) -> bool {
        self.is_always_set
    }
}

pub fn parse_options_status(input: &[u8]) -> IResult<&[u8], OptionsStatus> {
    #[allow(clippy::type_complexity)]
    let (i, b): (&[u8], (u8, u8, u8, u8, u8, u8, u8, u8)) =
        bits::<_, _, Error<(&[u8], usize)>, _, _>(tuple((
            streaming::take(1u8),
            streaming::take(1u8),
            streaming::take(1u8),
            streaming::take(1u8),
            streaming::take(1u8),
            streaming::take(1u8),
            streaming::take(1u8),
            streaming::take(1u8),
        )))(input)?;
    Ok((
        i,
        OptionsStatus {
            is_mobile_id: b.7 == 1,
            is_mobile_id_type: b.6 == 1,
            is_authentication_world: b.5 == 1,
            is_routing: b.4 == 1,
            is_forwarding: b.3 == 1,
            is_response_redirection: b.2 == 1,
            is_options_extension: b.1 == 1,
            is_always_set: b.0 == 1,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::{parse_mobile_id_type, parse_options_status};
    use crate::{
        parse_message_type, parse_sequence_number, parse_service_type,
        MobileID, MobileIDType, OptionsStatus,
    };

    #[test]
    fn test_parse_options_status() {
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

        let (i, b): (&[u8], OptionsStatus) =
            parse_options_status(&data).unwrap();

        assert_eq!(b.is_mobile_id(), true);
        assert_eq!(b.is_mobile_id_type(), true);
        assert_eq!(b.is_authentication_world(), false);
        assert_eq!(b.is_routing(), false);
        assert_eq!(b.is_forwarding(), false);
        assert_eq!(b.is_response_redirection(), false);
        assert_eq!(b.is_options_extension(), false);
        assert_eq!(b.is_always_set(), true);

        if b.is_mobile_id() {
            let (i, mobileid): (&[u8], MobileID) = MobileID::parse(i).unwrap();
            assert_eq!(mobileid.len(), 5);
            assert_eq!(mobileid.to_string(), String::from("4634663235"));

            if b.is_mobile_id_type() {
                let (i, mobileidtp): (&[u8], MobileIDType) =
                    parse_mobile_id_type(i).unwrap();
                assert_eq!(mobileidtp, MobileIDType::Esn);
                let (i, service_type) = parse_service_type(i).unwrap();
                assert_eq!(service_type, 1);
                let (i, message_type) = parse_message_type(i).unwrap();
                assert_eq!(message_type, 2);
                let (_, sequence_number) = parse_sequence_number(i).unwrap();
                assert_eq!(sequence_number, 14982);
            }
        }
    }
}
