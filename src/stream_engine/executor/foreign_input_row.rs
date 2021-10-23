pub(in crate::stream_engine) mod format;

use self::format::json::JsonObject;

use crate::{
    error::Result, stream_engine::model::option::foreign_stream::parsing::ParseJsonOption,
};

/// Input row from foreign systems (retrieved from InputServer).
///
/// Immediately converted into Row on stream-engine boundary.
#[derive(Eq, PartialEq, Debug)]
pub(super) struct ForeignInputRow(JsonObject);

impl ForeignInputRow {
    pub(super) fn from_json(json: JsonObject) -> Self {
        Self(json)
    }

    pub(super) fn into_row(self, option: ParseJsonOption) -> Result<ForeignInputRow> {
        todo!()
    }
}
