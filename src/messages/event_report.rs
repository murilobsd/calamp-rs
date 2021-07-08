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
use nom::number::streaming::{be_i32, be_u16, be_u32, be_u8};
use nom::IResult;

pub(crate) fn parse_service_type(input: &[u8]) -> IResult<&[u8], u8> {
    let (i, a): (&[u8], u8) = be_u8::<_, (_, ErrorKind)>(input).unwrap();
    Ok((i, a))
}

pub(crate) fn parse_message_type(input: &[u8]) -> IResult<&[u8], u8> {
    let (i, a): (&[u8], u8) = be_u8::<_, (_, ErrorKind)>(input).unwrap();
    Ok((i, a))
}

pub(crate) fn parse_sequence_number(input: &[u8]) -> IResult<&[u8], u16> {
    let (i, a): (&[u8], u16) = be_u16::<_, (_, ErrorKind)>(input).unwrap();
    Ok((i, a))
}

pub(crate) fn parse_update_time(input: &[u8]) -> IResult<&[u8], u32> {
    let (i, a): (&[u8], u32) = be_u32::<_, (_, ErrorKind)>(input).unwrap();
    Ok((i, a))
}

pub(crate) fn parse_latitude(input: &[u8]) -> IResult<&[u8], f32> {
    let (i, a): (&[u8], i32) = be_i32::<_, (_, ErrorKind)>(input).unwrap();
    let b = (a as f32) * 10.000;
    Ok((i, b))
}

/// Event Report messages
///
/// Initiated by the LMU and are generated by the LMU’s Programmable Event
/// Generator (PEG). They can be either Acknowledged or Unacknowledged Requests.
/// The Server should respond to an Acknowledged Event Report Request with an
/// Acknowledge Message. Event reports have the following structure:
#[derive(Debug, PartialEq)]
pub struct EventReport {
    /// The time tag of the message in seconds.
    pub update_time: u32,

    /// The last known time of fix from the GPS satellites.
    pub time_of_fix: u32,

    /// The latitude reading of the GPS receiver, measured in degrees with a
    /// 1x10^-7 degree lsb, signed 2’s complement.
    pub latitude: f32,

    /// The longitude reading of the GPS receiver, measured in degrees with a
    /// 1x10^-7 degree lsb, signed 2’s complement.
    pub longitude: f32,

    /// The altitude reading of the GPS receiver measured in centimeters above
    /// the WGS-84 Datum, signed 2’s complement.
    pub altitude: u32,

    /// The speed as reported by the GPS receiver, measured in centimeters per
    /// second.
    pub speed: u32,

    /// The heading value reported in degrees from true North.
    pub heading: u16,

    /// The number of satellites used in the GPS solution.
    pub satellites: u8,

    /// The current fix status of the GPS receiver bitmapped as follows
    pub fix_status: u8,

    /// The identifier of the Carrier/Operator the wireless modem is currently
    /// using. For GSM, HSPA, and LTE networks, this is the MNC (mobile network
    /// code). For CDMA networks this is the SID (system identification number).
    pub carrier: u16,

    // pub carrier_ms: u16,
    /// The received signal strength of the wireless modem in dBm. This value is
    /// signed in a 2’s complement format.
    pub rssi: u16,

    /// The current state of the wireless modem bit mapped as follows
    ///
    /// Bit 0 – Available
    ///
    /// Bit 1 – Network Service
    ///
    /// Bit 2 – Data Service
    ///
    /// Bit 3 – Connected (PPP Session Up)
    ///
    /// Bit 4 – Voice Call is Active
    ///
    /// Bit 5 – Roaming
    ///
    /// Bits 6-7 – Network Technology
    ///
    /// 00 = 2G Network (CDMA or GSM)
    ///
    /// 01 = 3G Network (UMTS)
    ///
    /// 10 = 4G Network (LTE)
    ///
    /// 11 = Reserved
    pub comm_state: u8,

    /// The GPS Horizontal Dilution of Precision - it is a unit-less value
    /// reported with a 0.1 lsb.
    pub hdop: u8,

    /// The current state of the inputs, bit mapped as follows:
    ///
    /// Bit 0 – Ignition
    ///
    /// Bit 1 – Input 1
    ///
    /// Bit 2 – Input 2
    ///
    /// Bit 3 – Input 3
    ///
    /// Bit 4 – Input 4
    ///
    /// Bit 5 – Input 5
    ///
    /// Bit 6 – Input 6
    ///
    /// Bit 7 – Input 7
    pub inputs: u8,

    /// Status of key modules within the unit:
    ///
    /// Bit 0 – LMU32: HTTP OTA Update Status (0=OK, 1=Error), LMU8: Unused
    ///
    /// Bit 1 – GPS Antenna Status (0=OK, 1=Error)
    ///
    /// Bit 2 – GPS Receiver Self-Test (0=OK, 1=Error) (LMU32 only)
    ///
    /// Bit 3 – GPS Receiver Tracking (0=Yes, 1=No)
    ///
    /// Bit 4 – Reserved, Currently Unused
    ///
    /// Bit 5 – Reserved, Currently Unused
    ///
    /// Bit 6 – Reserved, Currently Unused
    ///
    /// Bit 7 – Unused
    pub unit_status: u8,

    /// The index number of the event that generated the report; values should
    /// range from 0-249. 255 represents a Real Time PEG Action request.
    pub event_index: u8,

    /// The event code assigned to the report as specified by the event’s Action
    /// Parameter
    pub event_code: u8,

    /// The number of 4-byte values in the AccumList and the Accumulator
    /// Reporting Format Type (upper 2 bits).
    pub accums: u8,

    /// This bit-mapped byte is used to indicate the presence, when
    /// corresponding bit is set, of specific data types appended to the end of
    /// the Event Report following the list of accumulators. Each data type
    /// starts with a single length byte followed by the data. If multiple data
    /// types are present, they shall appear in order of the bits set in the
    /// ‘Append’ byte, starting with bit 0.
    ///
    /// Bit 0 – Cell Info (see 'Appended Data' section below).
    ///
    /// Bit 1 thru 7 – reserved, set to zero (0).
    pub append: u8,

    /// A list of ‘n’ 4-byte fields where ‘n’ is defined in the Accums field.
    /// The format for this list is dependent upon the Accumulator Reporting
    /// Format Type also defined in the Accums field. Refer to Appendix G,
    /// ‘Accumulator Reporting Formats’ for details.
    pub accum_list: u32,
}

/*
impl EventReport {
    /// Parse event report
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (i, update_time) = parse_update_time(input).unwrap();
        let (i, time_of_fix) = parse_update_time(i).unwrap();
        let (i, latitude) = parse_latitude(i).unwrap();
        let (i, longitude) = parse_latitude(i).unwrap();
        let (i, altitude) = parse_update_time(i).unwrap();
        let (i, speed) = parse_update_time(i).unwrap();
        let (i, heading) = parse_sequence_number(i).unwrap();
        let (i, satellites) = parse_message_type(i).unwrap();
        let (i, fix_status) = parse_message_type(i).unwrap();
        let (i, carrier) = parse_sequence_number(i).unwrap();
        // FIX: is int16? signed singal rssi
        let (i, rssi) = parse_sequence_number(i).unwrap();
        // FIX: get bit map enum
        let (i, comm_state) = parse_message_type(i).unwrap();
        // https://en.wikipedia.org/wiki/Dilution_of_precision_(navigation)
        let (i, hdop) = parse_message_type(i).unwrap();
        // FIX: get bit map enum
        let (i, inputs) = parse_message_type(i).unwrap();
        // println!("inputs : {:#010b}", inputs);
        let (i, unit_status) = parse_message_type(i).unwrap();
        let (i, event_index) = parse_message_type(i).unwrap();
        let (i, event_code) = parse_message_type(i).unwrap();
        let (i, accums) = parse_message_type(i).unwrap();
        let (i, append) = parse_message_type(i).unwrap();
        let (_, accum_list) = parse_update_time(i).unwrap();
        Ok((
            i,
            Self {
                update_time,
                time_of_fix,
                latitude,
                longitude,
                altitude,
                speed,
                heading,
                satellites,
                fix_status,
                carrier,
                rssi,
                comm_state,
                hdop,
                inputs,
                unit_status,
                event_index,
                event_code,
                accums,
                append,
                accum_list,
            },
        ))
    }
}
*/