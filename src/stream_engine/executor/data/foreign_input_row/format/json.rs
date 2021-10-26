use anyhow::Context;

use crate::{
    error::{Result, SpringError},
    model::name::ColumnName,
    stream_engine::executor::data::{column_values::ColumnValues, value::sql_value::SqlValue},
};

#[derive(Clone, Eq, PartialEq, Debug, new)]
pub(in crate::stream_engine::executor) struct JsonObject(serde_json::Value);

impl ToString for JsonObject {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl JsonObject {
    /// # Failure
    ///
    /// - [SpringError::InvalidFormat](crate::error::SpringError::InvalidFormat) when:
    ///   - Internal JSON cannot be mapped to SQL type (nested, for example).
    ///
    /// # TODO
    ///
    /// See stream.options to more intelligently parse JSON. <https://docs.sqlstream.com/sql-reference-guide/create-statements/createforeignstream/#parsing-json>
    pub(in crate::stream_engine::executor) fn into_column_values(self) -> Result<ColumnValues> {
        let json_object = self.0;

        let top_object = json_object
            .as_object()
            .context("top-level must be JSON object")
            .map_err(|e| SpringError::InvalidFormat {
                source: e,
                s: format!("{:?}", json_object),
            })?;

        top_object
            .into_iter()
            .fold(Ok(ColumnValues::default()), |acc, (k, v)| {
                let mut column_values = acc?;
                let (column_name, sql_value) = Self::to_column_value(k, v)?;
                column_values.insert(column_name, sql_value)?;
                Ok(column_values)
            })
    }

    fn to_column_value(
        json_key: &str,
        json_value: &serde_json::Value,
    ) -> Result<(ColumnName, SqlValue)> {
        let sql_value = SqlValue::try_from(json_value)?;
        let column_name = ColumnName::new(json_key.to_string());
        Ok((column_name, sql_value))
    }
}