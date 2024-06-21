// This file is part of https://github.com/SpringQL/SpringQL which is licensed under MIT OR Apache-2.0. See file LICENSE-MIT or LICENSE-APACHE for full license details.


mod json_source_row;


pub use json_source_row::JsonSourceRow;

use crate::{
    api::{error::Result, SpringError},
    stream_engine::autonomous_executor::row::schemaless_row::SchemalessRow,
};

/// Input row from foreign sources (retrieved from SourceReader).
///
/// Immediately converted into `Row` on stream-engine boundary.
#[derive(Clone, PartialEq, Debug)]
pub enum SourceRow {
    Json(JsonSourceRow),
    Raw(SchemalessRow),
}

impl SourceRow {
    /// # Failure
    ///
    /// - `SpringError::InvalidFormat` when:
    ///   - `json` cannot be parsed as a JSON
    pub fn from_json(json: &str) -> Result<Self> {
        let json_source_row = JsonSourceRow::parse(json)?;
        Ok(Self::Json(json_source_row))
    }
}

impl TryFrom<SourceRow> for SchemalessRow {
    type Error = SpringError;

    fn try_from(row: SourceRow) -> Result<Self> {
        match row {
            SourceRow::Json(json_source_row) => json_source_row.into_schemaless_row(),
            SourceRow::Raw(schemaless_row) => Ok(schemaless_row),
        }
    }
}
