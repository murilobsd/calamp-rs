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

#[cfg(feature = "chrono")]
use chrono::{DateTime, TimeZone, Utc};
use nom::error::ErrorKind;
use nom::number::streaming::{be_i16, be_i32, be_u16, be_u32, be_u8};
use nom::IResult;

#[allow(dead_code)]
pub(crate) fn pu32(input: &[u8]) -> IResult<&[u8], u32> {
    let (i, a): (&[u8], u32) = be_u32::<_, (_, ErrorKind)>(input).unwrap();
    Ok((i, a))
}

#[allow(dead_code)]
pub(crate) fn pu16(input: &[u8]) -> IResult<&[u8], u16> {
    let (i, a): (&[u8], u16) = be_u16::<_, (_, ErrorKind)>(input).unwrap();
    Ok((i, a))
}

#[allow(dead_code)]
pub(crate) fn pu8(input: &[u8]) -> IResult<&[u8], u8> {
    let (i, a): (&[u8], u8) = be_u8::<_, (_, ErrorKind)>(input).unwrap();
    Ok((i, a))
}

#[allow(dead_code)]
pub(crate) fn p16(input: &[u8]) -> IResult<&[u8], i16> {
    let (i, a): (&[u8], i16) = be_i16::<_, (_, ErrorKind)>(input).unwrap();
    Ok((i, a))
}

#[allow(dead_code)]
pub(crate) fn pf32(input: &[u8]) -> IResult<&[u8], f32> {
    let (i, a): (&[u8], i32) = be_i32::<_, (_, ErrorKind)>(input).unwrap();
    let b = (a as f32) * 1e-7;
    Ok((i, b))
}

#[cfg(feature = "chrono")]
#[allow(dead_code)]
pub(crate) fn pdt(input: &[u8]) -> IResult<&[u8], DateTime<Utc>> {
    let (i, a): (&[u8], u32) = be_u32::<_, (_, ErrorKind)>(input).unwrap();
    let dt = Utc.timestamp(a, 0);
    Ok((i, dt))
}
