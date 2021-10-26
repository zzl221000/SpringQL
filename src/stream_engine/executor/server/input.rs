use crate::{
    error::Result, model::option::Options,
    stream_engine::executor::data::foreign_input_row::ForeignInputRow,
};

pub(in crate::stream_engine::executor) mod net;

pub(in crate::stream_engine::executor) trait InputServerStandby<A: InputServerActive> {
    fn new(options: Options) -> Result<Self>
    where
        Self: Sized;

    /// Blocks until the server is ready to provide ForeignInputRow.
    fn start(self) -> Result<A>;
}

/// Active: ready to provide ForeignInputRow.
pub(in crate::stream_engine::executor) trait InputServerActive {
    /// Returns currently available foreign row.
    fn next_row(&mut self) -> Result<ForeignInputRow>;
}