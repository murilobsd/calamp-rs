//
// Copyright (c) 2021 Murilo Ijanc' <mbsd@m0x.ru>
//
// Permission to use, copy, modify, and distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//

const OPTIONS_BYTE: u8 = 0x83;

#[derive(Debug)]
pub struct OptionsHeader {
    pub mobile_id: String,
    pub mobile_type: u8,
    next_part: usize,
}

impl OptionsHeader {
    pub fn new(mobile_id: &str, mobile_type: u8, next_part: usize) -> Self {
        Self {
            mobile_id: mobile_id.to_string(),
            mobile_type,
            next_part,
        }
    }

    fn parse_options(data: &[u8]) -> Option<OptionsHeader> {
        let mut i: usize = 0;
        if data[i] == OPTIONS_BYTE {
            i += 1;
            let mobile_id_field_length: u8 = data[i];
            i += 1;
            let start = i;
            let mut mobile_id = String::from("");
            while i < start + mobile_id_field_length as usize {
                mobile_id.push_str(&format!("{0:2x}", data[i]));
                i += 1;
            }
            i += 1;
            let _mobile_type_length = data[i];
            i += 1;
            let mobile_type = data[i];
            i += 1;
            Some(OptionsHeader::new(&mobile_id, mobile_type, i))
        } else {
            None
        }
    }

    pub fn next_part(&self) -> usize {
        self.next_part
    }
}

pub fn parse(data: &[u8]) -> OptionsHeader {
    match OptionsHeader::parse_options(data) {
        Some(option) => option,
        None => panic!("nao possui option"),
    }
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn test_parse_options() {
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

        let option = parse(&data);
        assert_eq!(option.mobile_type, 1);
        assert_eq!(option.mobile_id, String::from("4634663235"));
        assert_eq!(option.next_part(), 10);
    }
}
