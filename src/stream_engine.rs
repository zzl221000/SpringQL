//! Stream Engine component.
//!
//! Responsible for pipeline management and execution.

mod executor;

pub(crate) use executor::{CurrentTimestamp, RowRepository, Timestamp};

#[cfg(test)]
pub(crate) use executor::TestRowRepository;