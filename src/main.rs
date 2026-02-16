use std::{env, process::ExitCode};

use crate::cartridge::Cartridge;

mod cartridge;
mod instruction;
mod labeller;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} cartridge_file", args[0]);
        return ExitCode::FAILURE;
    }

    let cartridge_filename = &args[1];
    let mut cartridge = Cartridge::load_from_file(cartridge_filename);
    cartridge.disassemble();
    cartridge.print_disassembly();

    return ExitCode::SUCCESS;
}
