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

use std::fmt;

use nom::bits::{bits, streaming};
use nom::error::{Error, ErrorKind};
use nom::number::streaming::be_u8;
use nom::IResult;

const OPTIONS_HEADER: u8 = 0x83;

#[derive(Clone, PartialEq, Eq)]
pub enum MobileIDType {
    Off,

    /// Electronic Serial Number (ESN) of the LMU
    Esn,

    /// International Mobile Equipment Identifier (IMEI) or Electronic
    /// Identifier (EID) of the wireless modem
    Equipment,

    /// International Mobile Subscriber Identifier (IMSI) of the SIM card
    /// (GSM/GPRS devices only)
    Subscriber,

    /// User Defined Mobile ID
    Defined,

    /// Phone Number ofkk the mobile (if available)
    PhoneNumber,

    /// The current IP Address of the LMU
    IpAddress,

    /// CDMA Mobile Equipment ID (MEID) or International Mobile Equipment
    /// Identifier (IMEI) of the wireless modem
    Cdma,
}

impl MobileIDType {
    pub fn parse(input: &[u8]) -> IResult<&[u8], MobileIDType> {
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
}

impl fmt::Display for MobileIDType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            MobileIDType::Off => write!(f, "MobileIDType::Off"),
            MobileIDType::Esn => write!(f, "MobileIDType::Esn"),
            MobileIDType::Equipment => write!(f, "MobileIDType::Equipament"),
            MobileIDType::Subscriber => write!(f, "MobileIDType::Subscriber"),
            MobileIDType::Defined => write!(f, "MobileIDType::Defined"),
            MobileIDType::PhoneNumber => write!(f, "MobileIDType::PhoneNumber"),
            MobileIDType::IpAddress => write!(f, "MobileIDType::IpAddress"),
            MobileIDType::Cdma => write!(f, "MobileIDType::Cdma"),
        }
    }
}

impl fmt::Debug for MobileIDType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            MobileIDType::Off => write!(f, "MobileIDType::Off"),
            MobileIDType::Esn => write!(f, "MobileIDType::Esn"),
            MobileIDType::Equipment => write!(f, "MobileIDType::Equipament"),
            MobileIDType::Subscriber => write!(f, "MobileIDType::Subscriber"),
            MobileIDType::Defined => write!(f, "MobileIDType::Defined"),
            MobileIDType::PhoneNumber => write!(f, "MobileIDType::PhoneNumber"),
            MobileIDType::IpAddress => write!(f, "MobileIDType::IpAddress"),
            MobileIDType::Cdma => write!(f, "MobileIDType::Cdma"),
        }
    }
}

#[derive(Debug)]
pub struct MobileID<'a>(&'a [u8]);

impl<'a> MobileID<'a> {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn parse(input: &'a [u8]) -> IResult<&[u8], Self> {
        let (i, a): (&[u8], u8) = be_u8::<_, (_, ErrorKind)>(input).unwrap();
        let (i, b): (&[u8], &[u8]) = nom::bytes::streaming::take(a)(i)?;
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

fn is_options_header(input: u8) -> bool {
    input == OPTIONS_HEADER
}

pub struct OptionsHeader<'a> {
    pub mobile_id: Option<MobileID<'a>>,
    pub mobile_id_type: Option<MobileIDType>,
}

impl<'a> OptionsHeader<'a> {
    pub fn parse(input: &'a [u8]) -> IResult<&[u8], Self> {
        // FIX: return error
        if !is_options_header(input[0]) {
            panic!("");
        }

        let (i, opt_status) = parse_options_status(input)?;
        let mut opt_header = OptionsHeader {
            mobile_id: None,
            mobile_id_type: None,
        };
        let mut inp: &[u8] = &[];

        // check mobile id
        if opt_status.is_mobile_id() {
            let (i, mob_id) = MobileID::parse(i)?;
            opt_header.mobile_id = Some(mob_id);
            inp = i;
        }

        // check mobile id type
        if opt_status.is_mobile_id_type() {
            let (i, mob_id_tp) = MobileIDType::parse(inp)?;
            opt_header.mobile_id_type = Some(mob_id_tp);
            inp = i;
        }

        Ok((inp, opt_header))
    }
}

fn parse_options_status(input: &[u8]) -> IResult<&[u8], OptionsStatus> {
    #[allow(clippy::type_complexity)]
    let (i, b): (&[u8], (u8, u8, u8, u8, u8, u8, u8, u8)) =
        bits::<_, _, Error<(&[u8], usize)>, _, _>(nom::sequence::tuple((
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
    use super::{MobileIDType, OptionsHeader};

    #[test]
    fn test_parse_options_headers() {
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

        let (i, opt_header) = OptionsHeader::parse(&data).unwrap();
        assert_eq!(i.len(), 108);

        if let Some(mob_id) = opt_header.mobile_id {
            assert_eq!(mob_id.len(), 5);
            assert_eq!(mob_id.to_string(), String::from("4634663235"));
        }

        if let Some(mob_id_tp) = opt_header.mobile_id_type {
            assert_eq!(mob_id_tp, MobileIDType::Esn);
            assert_eq!(
                format!("{}", mob_id_tp),
                String::from("MobileIDType::Esn")
            );
        }
    }
}

/*
#[cfg(test)]
mod tests {
    use super::{parse_mobile_id_type, parse_options_status};
    use crate::{
        mobileid, mobileidtype, optionsstatus, parse_latitude,
        parse_message_type, parse_sequence_number, parse_service_type,
        parse_update_time,
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

        let (i, b): (&[u8], optionsstatus) =
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
            let (i, mobileid): (&[u8], mobileid) = mobileid::parse(i).unwrap();
            assert_eq!(mobileid.len(), 5);
            assert_eq!(mobileid.to_string(), String::from("4634663235"));

            if b.is_mobile_id_type() {
                let (i, mobileidtp): (&[u8], mobileidtype) =
                    parse_mobile_id_type(i).unwrap();
                assert_eq!(mobileidtp, mobileidtype::esn);

                // message header
                let (i, service_type) = parse_service_type(i).unwrap();
                assert_eq!(service_type, 1);
                let (i, message_type) = parse_message_type(i).unwrap();
                assert_eq!(message_type, 2);
                let (i, sequence_number) = parse_sequence_number(i).unwrap();
                assert_eq!(sequence_number, 14982);

                // event report == message type == 2
                // let (i, update_time) = parse_update_time(i).unwrap();
                // assert_eq!(update_time, 1609644628);
                // let (i, time_of_fix) = parse_update_time(i).unwrap();
                // assert_eq!(time_of_fix, 1609644631);
                // let (i, latitude) = parse_latitude(i).unwrap();
                // assert_eq!(latitude, -2368129300.0);
                // let (i, longitude) = parse_latitude(i).unwrap();
                // assert_eq!(longitude, -4674790000.0);
                // let (i, altitude) = parse_update_time(i).unwrap();
                // assert_eq!(altitude, 79608);
                // let (i, speed) = parse_update_time(i).unwrap();
                // assert_eq!(speed, 11);
                // let (i, heading) = parse_sequence_number(i).unwrap();
                // assert_eq!(heading, 0);
                // let (i, satellites) = parse_message_type(i).unwrap();
                // assert_eq!(satellites, 6);
                // let (i, fix_status) = parse_message_type(i).unwrap();
                // assert_eq!(fix_status, 32);
                // let (i, carrier) = parse_sequence_number(i).unwrap();
                // assert_eq!(carrier, 0);
                // // fix: is int16? signed singal rssi
                // let (i, rssi) = parse_sequence_number(i).unwrap();
                // assert_eq!(rssi, 65421);
                // // fix: get bit map enum
                // let (i, comm_state) = parse_message_type(i).unwrap();
                // assert_eq!(comm_state, 2);
                // // println!("comm state: {:#010b}", comm_state);
                // // https://en.wikipedia.org/wiki/dilution_of_precision_(navigation)
                // let (i, hdop) = parse_message_type(i).unwrap();
                // assert_eq!(hdop, 30);
                // // fix: get bit map enum
                // let (i, inputs) = parse_message_type(i).unwrap();
                // assert_eq!(inputs, 30);
                // // println!("inputs : {:#010b}", inputs);
                // let (i, unit_status) = parse_message_type(i).unwrap();
                // assert_eq!(unit_status, 0);
                // let (i, event_index) = parse_message_type(i).unwrap();
                // assert_eq!(event_index, 123);
                // let (i, event_code) = parse_message_type(i).unwrap();
                // assert_eq!(event_code, 33);
                // let (i, accums) = parse_message_type(i).unwrap();
                // assert_eq!(accums, 16);
                // let (i, append) = parse_message_type(i).unwrap();
                // assert_eq!(append, 0);
                // let (_, accum_list) = parse_update_time(i).unwrap();
                // assert_eq!(accum_list, 12768);
            }
        }
    }
}
*/
