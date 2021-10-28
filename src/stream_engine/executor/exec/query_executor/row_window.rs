use std::collections::VecDeque;

use super::interm_row::PreservedRow;

/// Note: RowWindow is a temporal structure during query execution (cannot be a pump output).
#[derive(Debug, Default, new)]
pub(in crate::stream_engine::executor::exec::query_executor) struct RowWindow(
    VecDeque<PreservedRow>,
);

impl RowWindow {
    pub(in crate::stream_engine::executor::exec::query_executor) fn inner(
        &self,
    ) -> &VecDeque<PreservedRow> {
        &self.0
    }
}