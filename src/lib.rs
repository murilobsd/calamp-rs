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
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub mod message_header;
pub mod messages;
pub mod options_header;
mod utils;

use message_header::MessageHeader;
use messages::event_report::EventReport;
use options_header::OptionsHeader;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Message {
    pub options_header: Option<OptionsHeader>,
    pub message_header: MessageHeader,
    pub msg: EventReport,
}

impl Message {
    pub fn parse(input: &[u8]) -> Self {
        let (i, options_header) = OptionsHeader::parse(input).unwrap();
        let (i, message_header) = MessageHeader::parse(i).unwrap();
        let msg = EventReport::parse(i).unwrap();

        Message {
            options_header,
            message_header,
            msg,
        }
    }
}
