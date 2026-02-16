pub fn disassemble_instruction(prg_rom_contents: &[u8], contents_offset: usize, address: usize) -> (bool, usize, String) {
    let mut instruction_bytes_count = 1;

    let operand1 = prg_rom_contents[contents_offset + 1];
    let operand2 = prg_rom_contents[contents_offset + 2];

    let mut is_section_complete = false;
    let instruction_text;
    match prg_rom_contents[contents_offset] {
        0x09 => {
            instruction_bytes_count = 2;
            instruction_text = format!("ORA #${:02X}", operand1);
        },

        0x10 => {
            instruction_bytes_count = 2;
            let target_address = calculate_target_address((address + instruction_bytes_count) as u16, operand1);
            instruction_text = format!("BPL ${:04X}", target_address);
        },

        0x20 => {
            instruction_bytes_count = 3;
            let abs = create_u16(operand1, operand2);
            instruction_text = format!("JSR ${:04X}", abs);
        },

        0x4C => {
            instruction_bytes_count = 3;
            let abs = create_u16(operand1, operand2);
            instruction_text = format!("JMP ${:04X}", abs);
            is_section_complete = address as u16 == abs;
        },

        0x78 => instruction_text = format!("SEI"),

        0x8D => {
            instruction_bytes_count = 3;
            let abs = create_u16(operand1, operand2);
            instruction_text = format!("STA ${:04X}", abs);
        },

        0x9A => instruction_text = format!("TXS"),

        0xA0 => {
            instruction_bytes_count = 2;
            instruction_text = format!("LDY #${:02X}", operand1);
        },
        0xA2 => {
            instruction_bytes_count = 2;
            instruction_text = format!("LDX #${:02X}", operand1);
        },
        0xA9 => {
            instruction_bytes_count = 2;
            instruction_text = format!("LDA #${:02X}", operand1);
        },
        0xAD => {
            instruction_bytes_count = 3;
            let abs = create_u16(operand1, operand2);
            instruction_text = format!("LDA ${:04X}", abs);
        },

        0xB0 => {
            instruction_bytes_count = 2;
            let target_address = calculate_target_address((address + instruction_bytes_count) as u16, operand1);
            instruction_text = format!("BCS ${:04X}", target_address);
        },
        0xBD => {
            instruction_bytes_count = 3;
            let abs = create_u16(operand1, operand2);
            instruction_text = format!("LDA ${:04X}, X", abs);
        },

        0xC9 => {
            instruction_bytes_count = 2;
            instruction_text = format!("CMP #${:02X}", operand1);
        },
        0xCA => instruction_text = format!("DEX"),

        0xD0 => {
            instruction_bytes_count = 2;
            let target_address = ((address + instruction_bytes_count) as u16).wrapping_add(((operand1 as i8) as i16) as u16);
            instruction_text = format!("BNE ${:04X}", target_address);
        },
        0xD8 => instruction_text = format!("CLD"),

        0xEE => {
            instruction_bytes_count = 3;
            let abs = create_u16(operand1, operand2);
            instruction_text = format!("INC ${:04X}", abs);
        },

        unknown_opcode => panic!("Unknown opcode at address ${:04X}: ${:02X}", address, unknown_opcode),
    }

    let mut bytes = String::new();
    for i in 0..instruction_bytes_count {
        bytes = format!("{bytes} {:02X}", prg_rom_contents[contents_offset + i]);
    }
    let text_line = format!("    {instruction_text}        # {:04X} |{bytes}", address);

    (is_section_complete, instruction_bytes_count, text_line)
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
