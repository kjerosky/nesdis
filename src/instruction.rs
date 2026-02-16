pub fn print_disassembled_instruction(prg_rom_contents: &[u8], contents_offset: usize, address: usize) -> usize {
    let mut instruction_bytes_count = 1;

    let operand1 = prg_rom_contents[contents_offset + 1];
    let operand2 = prg_rom_contents[contents_offset + 2];

    print!("    ");
    match prg_rom_contents[contents_offset] {
        0x09 => {
            instruction_bytes_count = 2;
            print!("ORA #${:02X}", operand1);
        },

        0x10 => {
            instruction_bytes_count = 2;
            let target_address = calculate_target_address((address + instruction_bytes_count) as u16, operand1);
            print!("BPL ${:04X}", target_address);
        },

        0x20 => {
            instruction_bytes_count = 3;
            let abs = create_u16(operand1, operand2);
            print!("JSR ${:04X}", abs);
        },

        0x4C => {
            instruction_bytes_count = 3;
            let abs = create_u16(operand1, operand2);
            print!("JMP ${:04X}", abs);
        },

        0x78 => print!("SEI"),

        0x8D => {
            instruction_bytes_count = 3;
            let abs = create_u16(operand1, operand2);
            print!("STA ${:04X}", abs);
        },

        0x9A => print!("TXS"),

        0xA0 => {
            instruction_bytes_count = 2;
            print!("LDY #${:02X}", operand1);
        },
        0xA2 => {
            instruction_bytes_count = 2;
            print!("LDX #${:02X}", operand1);
        },
        0xA9 => {
            instruction_bytes_count = 2;
            print!("LDA #${:02X}", operand1);
        },
        0xAD => {
            instruction_bytes_count = 3;
            let abs = create_u16(operand1, operand2);
            print!("LDA ${:04X}", abs);
        },

        0xB0 => {
            instruction_bytes_count = 2;
            let target_address = calculate_target_address((address + instruction_bytes_count) as u16, operand1);
            print!("BCS ${:04X}", target_address);
        },
        0xBD => {
            instruction_bytes_count = 3;
            let abs = create_u16(operand1, operand2);
            print!("LDA ${:04X}, X", abs);
        },

        0xC9 => {
            instruction_bytes_count = 2;
            print!("CMP #${:02X}", operand1);
        },
        0xCA => print!("DEX"),

        0xD0 => {
            instruction_bytes_count = 2;
            let target_address = ((address + instruction_bytes_count) as u16).wrapping_add(((operand1 as i8) as i16) as u16);
            print!("BNE ${:04X}", target_address);
        },
        0xD8 => print!("CLD"),

        0xEE => {
            instruction_bytes_count = 3;
            let abs = create_u16(operand1, operand2);
            print!("INC ${:04X}", abs);
        },

        unknown_opcode => panic!("Unknown opcode at address ${:04X}: ${:02X}", address, unknown_opcode),
    }

    print!("        # {:04X} |", address);
    for i in 0..instruction_bytes_count {
        print!(" {:02X}", prg_rom_contents[contents_offset + i]);
    }
    println!();

    instruction_bytes_count
}

// ---------------------------------------------------------------------------

fn create_u16(low_byte: u8, high_byte: u8) -> u16 {
    ((high_byte as u16) << 8) | (low_byte as u16)
}

// ---------------------------------------------------------------------------

fn calculate_target_address(address: u16, signed_offset: u8) -> u16 {
    let sign_extended_offset = ((signed_offset as i8) as i16) as u16;
    (address as u16).wrapping_add(sign_extended_offset)
}
