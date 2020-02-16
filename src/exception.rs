use std::process::exit;

use crate::*;

#[derive(Debug)]
pub enum Exception {
    InstructionAddressMisaligned,
    IllegalInstruction,
}

impl Exception {
    pub fn take_trap(&self) {
        output(&format!("exception: {:#?}", self));
        exit(1);
    }
}
