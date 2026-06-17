// SPDX-License-Identifier: MPL-2.0
// SPDX-FileCopyrightText: sevonj
/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::any::type_name;

use crate::error::MgsError;

pub fn check_fits_buf<T>(buf: &[u8]) -> Result<(), MgsError> {
    let expected = size_of::<T>();
    if buf.len() < expected {
        Err(MgsError::BufferTooSmall {
            for_what: type_name::<T>(),
            need: expected,
            avail: buf.len(),
        })
    } else {
        Ok(())
    }
}

/// Errors if value is infinite, NaN, or subnormal
pub const fn validate_f32(value: f32, field: &'static str) -> Result<(), MgsError> {
    if value.is_infinite() || value.is_nan() || value.is_subnormal() {
        return Err(MgsError::NonsensicalFloat { field, got: value });
    }
    Ok(())
}

pub fn read_i32_le(buf: &[u8], offset: usize) -> Result<i32, MgsError> {
    check_fits_buf::<i32>(buf)?;
    Ok(read_i32_le_unchecked(buf, offset))
}

pub fn read_i32_le_unchecked(buf: &[u8], offset: usize) -> i32 {
    i32::from_le_bytes(buf[offset..offset + 4].try_into().unwrap())
}

pub fn read_u32_le(buf: &[u8], offset: usize) -> Result<u32, MgsError> {
    check_fits_buf::<u32>(buf)?;
    Ok(read_u32_le_unchecked(buf, offset))
}

pub fn read_u32_le_unchecked(buf: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(buf[offset..offset + 4].try_into().unwrap())
}

pub fn read_i16_le(buf: &[u8], offset: usize) -> Result<i16, MgsError> {
    check_fits_buf::<i16>(buf)?;
    Ok(read_i16_le_unchecked(buf, offset))
}

pub fn read_i16_le_unchecked(buf: &[u8], offset: usize) -> i16 {
    i16::from_le_bytes(buf[offset..offset + 2].try_into().unwrap())
}

pub fn read_u16_le(buf: &[u8], offset: usize) -> Result<u16, MgsError> {
    check_fits_buf::<u16>(buf)?;
    Ok(read_u16_le_unchecked(buf, offset))
}

pub fn read_u16_le_unchecked(buf: &[u8], offset: usize) -> u16 {
    u16::from_le_bytes(buf[offset..offset + 2].try_into().unwrap())
}

pub fn read_f32_le(buf: &[u8], offset: usize) -> Result<f32, MgsError> {
    check_fits_buf::<f32>(buf)?;
    Ok(f32::from_le_bytes(
        buf[offset..offset + 4].try_into().unwrap(),
    ))
}

pub fn read_array<const N: usize>(buf: &[u8], offset: usize) -> [u8; N] {
    buf.get(offset..offset + N)
        .and_then(|b| b.try_into().ok())
        .unwrap()
}

pub fn read_slice<'a>(buf: &'a [u8], offset: usize, len: usize) -> Result<&'a [u8], MgsError> {
    if buf.len() < offset + len {
        return Err(MgsError::BufferTooSmall {
            for_what: "read_slice",
            need: len,
            avail: buf.len(),
        });
    }
    Ok(&buf[offset..offset + len])
}

pub fn read_cstr(buf: &[u8], offset: usize) -> Result<&str, MgsError> {
    let buf = buf
        .get(offset..)
        .ok_or(MgsError::CStringRanOutOfBytes(buf.len()))?;

    let len = buf
        .iter()
        .position(|&b| b == 0)
        .ok_or(MgsError::InvalidString { offset })?;

    std::str::from_utf8(&buf[..len]).map_err(|_| MgsError::InvalidString { offset })
}

pub fn align(offset: &mut usize, align: usize) {
    *offset = offset.div_ceil(align) * align;
}

pub fn aligned(offset: usize, align: usize) -> usize {
    offset.div_ceil(align) * align
}

pub fn align_pad<W: std::io::Write>(
    w: &mut W,
    data_offset: &mut usize,
    align: usize,
) -> Result<(), std::io::Error> {
    while !data_offset.is_multiple_of(align) {
        w.write_all(&[0])?;
        *data_offset += 1;
    }
    Ok(())
}
