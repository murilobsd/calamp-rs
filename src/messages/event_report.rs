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

use nom::bits::{bits, streaming};
use nom::error::Error;
use nom::IResult;

use crate::utils;

#[derive(Debug, PartialEq)]
pub struct FixStatus {
    /// Bit is set when the position update has a horizontal position accuracy estimate that is
    /// less that the Horizontal Position Accuracy Threshold defined in S-Register 142 (and the
    /// threshold is non-zero).
    predicted: bool,

    /// This bit is set when WAAS DGPS is enabled (S-Register 139) and the position has been
    /// differentially corrected
    diff_corrected: bool,

    /// This bit is set when the current GPS fix is invalid but a previous fix’s value is
    /// available.
    last_know: bool,

    /// This bit is set only after a power-up or reset before a valid fix is obtained.
    invalid_fix: bool,

    /// This bit is set when 3 or fewer satellites are seen/used in the GPS fix. (i.e. with 3
    /// satellites or less, an altitude value cannot be calculated)
    twod_fix: bool,

    /// This bit is set when the message has been logged by the LMU.
    historic: bool,

    /// This bit is set only after a power-up or reset before a valid time-sync has been obtained.
    invalid_time: bool,
}

impl FixStatus {
    pub fn parse(input: &[u8]) -> IResult<&[u8], FixStatus> {
        #[allow(clippy::type_complexity)]
        let (i, b): (&[u8], (u8, u8, u8, u8, u8, u8, u8)) =
            bits::<_, _, Error<(&[u8], usize)>, _, _>(nom::sequence::tuple((
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
            FixStatus {
                predicted: b.6 == 1,
                diff_corrected: b.5 == 1,
                last_know: b.4 == 1,
                invalid_fix: b.3 == 1,
                twod_fix: b.2 == 1,
                historic: b.1 == 1,
                invalid_time: b.0 == 1,
            },
        ))
    }
}

#[derive(Debug, PartialEq)]
pub enum NetworkTechnology {
    /// 2G
    CdmaGsm,

    /// 3G
    Umts,

    /// 4G
    Lte,

    /// Reserved network
    Reserved,
}

impl NetworkTechnology {
    pub fn parse(input: &[u8]) -> IResult<&[u8], NetworkTechnology> {
        let (i, b) = utils::pu8(input).unwrap();

        match b {
            0 => Ok((i, NetworkTechnology::CdmaGsm)),
            1 => Ok((i, NetworkTechnology::Umts)),
            10 => Ok((i, NetworkTechnology::Lte)),
            11 => Ok((i, NetworkTechnology::Reserved)),
            _ => panic!("not found"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct CommState {
    pub available: bool,
    pub network_service: bool,
    pub data_service: bool,
    pub connected: bool,
    pub voice_call_active: bool,
    pub roaming: bool,
    pub network_technology: NetworkTechnology,
}

impl CommState {
    pub fn parse(input: &[u8]) -> IResult<&[u8], CommState> {
        let (i, _) = utils::pu8(input).unwrap();
        Ok((
            i,
            CommState {
                available: false,
                network_service: false,
                data_service: false,
                connected: false,
                voice_call_active: false,
                roaming: false,
                network_technology: NetworkTechnology::CdmaGsm,
            },
        ))
    }
}

#[derive(Debug, PartialEq)]
pub struct Inputs {
    ignition: bool,
    input_1: bool,
    input_2: bool,
    input_3: bool,
    input_4: bool,
    input_5: bool,
    input_6: bool,
    input_7: bool,
}

impl Inputs {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Inputs> {
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
            Inputs {
                ignition: b.7 == 1,
                input_1: b.6 == 1,
                input_2: b.5 == 1,
                input_3: b.4 == 1,
                input_4: b.3 == 1,
                input_5: b.2 == 1,
                input_6: b.1 == 1,
                input_7: b.0 == 1,
            },
        ))
    }
}

#[derive(Debug, PartialEq)]
pub struct UnitStatus {
    ignition: bool,
    input_1: bool,
    input_2: bool,
    input_3: bool,
    input_4: bool,
    input_5: bool,
    input_6: bool,
    input_7: bool,
}

impl UnitStatus {
    pub fn parse(input: &[u8]) -> IResult<&[u8], UnitStatus> {
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
            UnitStatus {
                ignition: b.7 == 1,
                input_1: b.6 == 1,
                input_2: b.5 == 1,
                input_3: b.4 == 1,
                input_4: b.3 == 1,
                input_5: b.2 == 1,
                input_6: b.1 == 1,
                input_7: b.0 == 1,
            },
        ))
    }
}

/// Event Report messages
///
/// Initiated by the LMU and are generated by the LMU’s Programmable Event
/// Generator (PEG). They can be either Acknowledged or Unacknowledged Requests.
/// The Server should respond to an Acknowledged Event Report Request with an
/// Acknowledge Message.
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
    pub altitude: f32,

    /// The speed as reported by the GPS receiver, measured in centimeters per
    /// second.
    pub speed: f32,

    /// The heading value reported in degrees from true North.
    pub heading: u16,

    /// The number of satellites used in the GPS solution.
    pub satellites: u8,

    /// The current fix status of the GPS receiver bitmapped as follows
    pub fix_status: FixStatus,

    /// The identifier of the Carrier/Operator the wireless modem is currently
    /// using. For GSM, HSPA, and LTE networks, this is the MNC (mobile network
    /// code). For CDMA networks this is the SID (system identification number).
    pub carrier: u16,

    // pub carrier_ms: u16,
    /// The received signal strength of the wireless modem in dBm. This value is
    /// signed in a 2’s complement format.
    pub rssi: i16,

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
    pub comm_state: CommState,

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
    pub inputs: Inputs,

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
    pub unit_status: UnitStatus,

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

impl EventReport {
    /// Parse event report
    pub fn parse(input: &[u8]) -> IResult<&[u8], EventReport> {
        let (i, update_time) = utils::pu32(input).unwrap();
        let (i, time_of_fix) = utils::pu32(i).unwrap();
        let (i, latitude) = utils::pf32(i).unwrap();
        let (i, longitude) = utils::pf32(i).unwrap();
        let (i, altitude) = utils::pf32(i).unwrap();
        let (i, speed) = utils::pf32(i).unwrap();
        let (i, heading) = utils::pu16(i).unwrap();
        let (i, satellites) = utils::pu8(i).unwrap();
        let (i, fix_status) = FixStatus::parse(i).unwrap();
        let (i, carrier) = utils::pu16(i).unwrap();
        let (i, rssi) = utils::p16(i).unwrap();
        let (i, comm_state) = CommState::parse(i).unwrap();
        let (i, hdop) = utils::pu8(i).unwrap();
        let (i, inputs) = Inputs::parse(i).unwrap();
        let (i, unit_status) = UnitStatus::parse(i).unwrap();
        let (i, event_index) = utils::pu8(i).unwrap();
        let (i, event_code) = utils::pu8(i).unwrap();
        let (i, accums) = utils::pu8(i).unwrap();
        let (i, append) = utils::pu8(i).unwrap();
        let (i, accum_list) = utils::pu32(i).unwrap();

        Ok((
            i,
            EventReport {
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

#[cfg(test)]
mod tests {

    use super::EventReport;
    use crate::message_header::MessageHeader;
    use crate::options_header::OptionsHeader;

    #[test]
    fn test_parse_event_report_message() {
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
        let (i, _) = MessageHeader::parse(i).unwrap();
        let (_, event_report) = EventReport::parse(i).unwrap();

        assert_eq!(event_report.update_time, 1609644628);
        assert_eq!(event_report.time_of_fix, 1609644631);
        approx::assert_relative_eq!(event_report.latitude, -23.6812936);
        approx::assert_relative_eq!(
            event_report.longitude,
            -46.747897599999995
        );
        approx::assert_relative_eq!(event_report.altitude, 0.0079608);
        approx::assert_relative_eq!(event_report.speed, 0.0000011);
        assert_eq!(event_report.heading, 0);
        assert_eq!(event_report.satellites, 6);
        // assert_eq!(event_report.fix_status, 0);
        assert_eq!(event_report.carrier, 0);
        assert_eq!(event_report.rssi, -115);
        // assert_eq!(event_report.comm_state, 0);
        assert_eq!(event_report.hdop, 30);
        // assert_eq!(event_report.inputs, 0);
        // assert_eq!(event_report.unit_status, 0);
        assert_eq!(event_report.event_index, 123);
        assert_eq!(event_report.event_code, 33);
        assert_eq!(event_report.accums, 16);
        assert_eq!(event_report.append, 0);
        // assert_eq!(event_report.accum_list, 0);
    }
}
