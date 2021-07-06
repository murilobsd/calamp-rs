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

use nom::bits::{bits, streaming::take};
use nom::error::Error;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug)]
pub struct OptionsStatus {
    is_mobile_id: bool,
    is_mobile_id_type: bool,
    is_authentication_world: bool,
    is_routing: bool,
    is_forwarding: bool,
    is_response_redirection: bool,
    is_options_extension: bool,
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
            take(1u8),
            take(1u8),
            take(1u8),
            take(1u8),
            take(1u8),
            take(1u8),
            take(1u8),
            take(1u8),
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
    use super::parse_options_status;
    use crate::OptionsStatus;

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

        let (_, b): (&[u8], OptionsStatus) =
            parse_options_status(&data).unwrap();

        assert_eq!(b.is_mobile_id(), true);
        assert_eq!(b.is_mobile_id_type(), true);
        assert_eq!(b.is_authentication_world(), false);
        assert_eq!(b.is_routing(), false);
        assert_eq!(b.is_forwarding(), false);
        assert_eq!(b.is_response_redirection(), false);
        assert_eq!(b.is_options_extension(), false);
        assert_eq!(b.is_always_set(), true);
    }
}
