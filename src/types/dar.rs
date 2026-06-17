// SPDX-License-Identifier: MPL-2.0
// SPDX-FileCopyrightText: sevonj
/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::path::Path;

use crate::error::MgsError;
use crate::util::*;

/// Absolutely minimal archive format
#[derive(Debug, Clone)]
pub struct Darchive {
    files: Vec<DarFileEntry>,
}

impl Darchive {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, MgsError> {
        Self::read(&std::fs::read(path)?, &mut 0)
    }

    pub fn read(buf: &[u8], data_offset: &mut usize) -> Result<Self, MgsError> {
        let num_files = read_u32_le(buf, *data_offset)? as usize;
        *data_offset += 4;

        let mut files = Vec::with_capacity(num_files);
        for _ in 0..num_files {
            files.push(DarFileEntry::read(buf, data_offset)?);
        }

        Ok(Self { files })
    }

    pub fn files(&self) -> &[DarFileEntry] {
        &self.files
    }
}

#[derive(Debug, Clone)]
pub struct DarFileEntry {
    name: String,
    contents: Box<[u8]>,
}

impl DarFileEntry {
    pub fn read(buf: &[u8], data_offset: &mut usize) -> Result<Self, MgsError> {
        let name = read_cstr(buf, *data_offset)?.to_string();
        *data_offset += name.len() + 1;

        align(data_offset, 4);

        let len = read_u32_le(buf, *data_offset)? as usize;
        *data_offset += 4;

        let contents: Box<[u8]> = read_slice(buf, *data_offset, len)?.into();
        *data_offset += contents.len() + 1;

        Ok(Self { name, contents })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn contents(&self) -> &[u8] {
        &self.contents
    }
}
