//! A context is a wrapper arround all data and handlers that might be useful to the micro-service writer.

use crate::journal::journal::Journal;

pub struct Context<'a> {
    pub journal: &'a mut dyn Journal,
}
