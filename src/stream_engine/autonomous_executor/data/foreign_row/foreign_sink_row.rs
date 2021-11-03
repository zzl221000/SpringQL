use crate::stream_engine::autonomous_executor::data::row::Row;

use super::format::json::JsonObject;

/// Output row into foreign systems (retrieved by SinkServer).
///
/// Immediately converted from Row on stream-engine boundary.
#[derive(Eq, PartialEq, Debug)]
pub(in crate::stream_engine) struct ForeignSinkRow(JsonObject);

impl From<ForeignSinkRow> for JsonObject {
    fn from(foreign_output_row: ForeignSinkRow) -> Self {
        foreign_output_row.0
    }
}

/// # Failure
///
/// - [SpringError::InvalidFormat](crate::error::SpringError::InvalidFormat) when:
///   - This row cannot be converted into foreign output row.
impl From<Row> for ForeignSinkRow {
    fn from(row: Row) -> Self {
        let map = row
            .into_iter()
            .map(|(col, val)| (col.to_string(), serde_json::Value::from(val)))
            .collect::<serde_json::Map<String, serde_json::Value>>();
        let v = serde_json::Value::from(map);
        Self(JsonObject::new(v))
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::{
        model::name::ColumnName,
        stream_engine::autonomous_executor::{data::value::sql_value::SqlValue, Timestamp},
    };

    use super::*;

    #[test]
    fn test_from_row() {
        let row = Row::fx_city_temperature_tokyo();

        let f_row = ForeignSinkRow(JsonObject::new(json!({
            "timestamp": Timestamp::fx_ts1().to_string(),
            "city": "Tokyo",
            "temperature": 21
        })));

        assert_eq!(ForeignSinkRow::from(row), f_row);
    }

    #[test]
    fn test_from_row_arrival_rowtime() {
        let row = Row::fx_no_promoted_rowtime();
        let f_row = ForeignSinkRow::from(row);
        let f_json = JsonObject::from(f_row);
        let mut f_colvals = f_json.into_column_values().unwrap();
        let f_rowtime_sql_value = f_colvals.remove(&ColumnName::arrival_rowtime()).unwrap();

        if let SqlValue::NotNull(f_rowtime_nn_sql_value) = f_rowtime_sql_value {
            let f_rowtime: Timestamp = f_rowtime_nn_sql_value.unpack().unwrap();
            assert_eq!(f_rowtime, Timestamp::fx_now());
        } else {
            unreachable!()
        };
    }
}