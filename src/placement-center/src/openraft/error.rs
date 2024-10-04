use std::fmt::Display;

use common_base::errors::RobustMQError;
use openraft::error::{NetworkError, RPCError, Unreachable};

use super::typeconfig::TypeConfig;

#[derive(Debug)]
struct ErrWrap(Box<dyn std::error::Error>);

impl Display for ErrWrap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::error::Error for ErrWrap {}

pub fn to_error<E: std::error::Error + 'static + Clone>(
    e: RobustMQError,
) -> RPCError<TypeConfig, E> {
    RPCError::Unreachable(Unreachable::new(&e))
}
