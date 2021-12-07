// Copyright (c) 2021 TOYOTA MOTOR CORPORATION. Licensed under MIT OR Apache-2.0.

mod boolean;
mod int;
mod text;
mod timestamp;

use crate::{
    error::{Result, SpringError},
    stream_engine::autonomous_executor::Timestamp,
};
use anyhow::anyhow;
use std::any::type_name;

use super::sql_value::nn_sql_value::NnSqlValue;

/// Rust values which can have bidirectional mapping to/from SQL [NnSqlValue](crate::NnSqlValue).
pub(crate) trait SqlConvertible: Sized {
    /// Convert Rust type into strictly-matching SQL type.
    fn into_sql_value(self) -> NnSqlValue;

    /// # Failures
    ///
    /// - [SpringError::Sql](crate::error::SpringError::Sql) when:
    ///   - the type implementing SqlConvertible is not convertible from i16
    fn try_from_i16(_: &i16) -> Result<Self> {
        Self::default_err("i16")
    }

    /// # Failures
    ///
    /// - [SpringError::Sql](crate::error::SpringError::Sql) when:
    ///   - the type implementing SqlConvertible is not convertible from i32
    fn try_from_i32(_: &i32) -> Result<Self> {
        Self::default_err("i32")
    }

    /// # Failures
    ///
    /// - [SpringError::Sql](crate::error::SpringError::Sql) when:
    ///   - the type implementing SqlConvertible is not convertible from i64
    fn try_from_i64(_: &i64) -> Result<Self> {
        Self::default_err("i64")
    }

    /// # Failures
    ///
    /// - [SpringError::Sql](crate::error::SpringError::Sql) when:
    ///   - the type implementing SqlConvertible is not convertible from String
    fn try_from_string(_: &str) -> Result<Self> {
        Self::default_err("String")
    }

    /// # Failures
    ///
    /// - [SpringError::Sql](crate::error::SpringError::Sql) when:
    ///   - the type implementing SqlConvertible is not convertible from bool
    fn try_from_bool(_: &bool) -> Result<Self> {
        Self::default_err("bool")
    }

    /// # Failures
    ///
    /// - [SpringError::Sql](crate::error::SpringError::Sql) when:
    ///   - the type implementing SqlConvertible is not convertible from Timestamp
    fn try_from_timestamp(_: &Timestamp) -> Result<Self> {
        Self::default_err("Timestamp")
    }

    #[doc(hidden)]
    fn default_err(from_type: &str) -> Result<Self> {
        Err(SpringError::Sql(anyhow!(
            "cannot convert {} -> {}",
            from_type,
            type_name::<Self>()
        )))
    }
}