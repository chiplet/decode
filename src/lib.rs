use std::{collections::HashMap, fmt};

use bits::{Bit, Bits};

// command-line interface
pub mod frontend;
pub mod commands;

// library
pub mod bits;
pub mod decoder;

/// Inclusive range of bit indices from `start` to `end`.
#[derive(Eq, PartialEq, Hash, Debug)]
struct IdxRange {
    start: usize,
    end: usize,
}
impl IdxRange {
    fn new(end: usize, start: usize) -> Self {
        assert!(end >= start);
        Self {
            start,
            end,
        }
    }
}
impl fmt::Display for IdxRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.start, self.end)
    }
}

pub trait Fields<'a> {
    fn fields(&self, bits: &'a Bits) -> HashMap<&str, Vec<(&'a [Bit], IdxRange)>>;
}