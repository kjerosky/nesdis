use std::{env, process::ExitCode};

use crate::cartridge::Cartridge;

mod cartridge;
mod instruction;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} cartridge_file", args[0]);
        return ExitCode::FAILURE;
    }

    let cartridge_filename = &args[1];
    let cartridge = Cartridge::load_from_file(cartridge_filename);
    cartridge.disassemble();

    return ExitCode::SUCCESS;
}
