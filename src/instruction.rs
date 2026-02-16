use crate::labeller::Labeller;

pub fn disassemble_instruction(prg_rom_contents: &[u8], contents_offset: usize, address: usize, labeller: &mut Labeller) -> (bool, usize, String) {
    let operand1 = prg_rom_contents[contents_offset + 1];
    let operand2 = prg_rom_contents[contents_offset + 2];

    let mut is_section_complete = false;
    let instruction_text;
    let instruction_bytes_count;
    match prg_rom_contents[contents_offset] {
        0x09 => {
            instruction_bytes_count = 2;
            instruction_text = format!("ORA #${:02X}", operand1);
        },

        0x10 => {
            instruction_bytes_count = 2;
            let target_address = calculate_target_address((address + instruction_bytes_count) as u16, operand1);
            let label = labeller.request_label_for_branch_target(target_address);
            instruction_text = format!("BPL {label}");
        },
        0x18 => {
            instruction_bytes_count = 1;
            instruction_text = format!("CLC");
        },

        0x20 => {
            instruction_bytes_count = 3;
            let abs = create_u16(operand1, operand2);
            let label = labeller.request_label_for_subroutine(abs as usize);
            instruction_text = format!("JSR {label}");
        },
        0x29 => {
            instruction_bytes_count = 2;
            instruction_text = format!("AND #${:02X}", operand1);
        },

        0x38 => {
            instruction_bytes_count = 1;
            instruction_text = format!("SEC");
        },

        0x40 => {
            instruction_bytes_count = 1;
            instruction_text = format!("RTI");
            is_section_complete = true;
        },
        0x45 => {
            instruction_bytes_count = 2;
            instruction_text = format!("EOR ${:02X}", operand1);
        },
        0x48 => {
            instruction_bytes_count = 1;
            instruction_text = format!("PHA");
        },
        0x4A => {
            instruction_bytes_count = 1;
            instruction_text = format!("LSR A");
        },
        0x4C => {
            instruction_bytes_count = 3;
            let abs = create_u16(operand1, operand2);
            let label = labeller.request_label_for_jump_target(abs as usize);
            instruction_text = format!("JMP {label}");
            is_section_complete = address as u16 == abs;
        },

        0x68 => {
            instruction_bytes_count = 1;
            instruction_text = format!("PLA");
        },

        0x78 => {
            instruction_bytes_count = 1;
            instruction_text = format!("SEI");
        },
        0x7E => {
            instruction_bytes_count = 3;
            let abs = create_u16(operand1, operand2);
            instruction_text = format!("ROR ${:04X}, X", abs);
        },

        0x85 => {
            instruction_bytes_count = 2;
            instruction_text = format!("STA ${:02X}", operand1);
        },
        0x8D => {
            instruction_bytes_count = 3;
            let abs = create_u16(operand1, operand2);
            instruction_text = format!("STA ${:04X}", abs);
        },
        0x88 => {
            instruction_bytes_count = 1;
            instruction_text = format!("DEY");
        },

        0x9A => {
            instruction_bytes_count = 1;
            instruction_text = format!("TXS");
        },
        0x9D => {
            instruction_bytes_count = 3;
            let abs = create_u16(operand1, operand2);
            instruction_text = format!("STA ${:04X}, X", abs);
        },

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
        0xAC => {
            instruction_bytes_count = 3;
            let abs = create_u16(operand1, operand2);
            instruction_text = format!("LDY ${:04X}", abs);
        },
        0xAD => {
            instruction_bytes_count = 3;
            let abs = create_u16(operand1, operand2);
            instruction_text = format!("LDA ${:04X}", abs);
        },
        0xAE => {
            instruction_bytes_count = 3;
            let abs = create_u16(operand1, operand2);
            instruction_text = format!("LDX ${:04X}", abs);
        },

        0xB0 => {
            instruction_bytes_count = 2;
            let target_address = calculate_target_address((address + instruction_bytes_count) as u16, operand1);
            let label = labeller.request_label_for_branch_target(target_address);
            instruction_text = format!("BCS {label}");
        },
        0xBD => {
            instruction_bytes_count = 3;
            let abs = create_u16(operand1, operand2);
            instruction_text = format!("LDA ${:04X}, X", abs);
        },
        0xBE => {
            instruction_bytes_count = 3;
            let abs = create_u16(operand1, operand2);
            instruction_text = format!("LDX ${:04X}, Y", abs);
        },

        0xC9 => {
            instruction_bytes_count = 2;
            instruction_text = format!("CMP #${:02X}", operand1);
        },
        0xCA => {
            instruction_bytes_count = 1;
            instruction_text = format!("DEX");
        },
        0xC8 => {
            instruction_bytes_count = 1;
            instruction_text = format!("INY");
        },
        0xCE => {
            instruction_bytes_count = 3;
            let abs = create_u16(operand1, operand2);
            instruction_text = format!("DEC ${:04X}", abs);
        },

        0xD0 => {
            instruction_bytes_count = 2;
            let target_address = calculate_target_address((address + instruction_bytes_count) as u16, operand1);
            let label = labeller.request_label_for_branch_target(target_address);
            instruction_text = format!("BNE {label}");
        },
        0xD8 => {
            instruction_bytes_count = 1;
            instruction_text = format!("CLD");
        },
        0xDE => {
            instruction_bytes_count = 3;
            let abs = create_u16(operand1, operand2);
            instruction_text = format!("DEC ${:04X}, X", abs);
        },

        0xE0 => {
            instruction_bytes_count = 2;
            instruction_text = format!("CPX #${:02X}", operand1);
        },
        0xE6 => {
            instruction_bytes_count = 2;
            instruction_text = format!("INC ${:02X}", operand1);
        },
        0xE8 => {
            instruction_bytes_count = 1;
            instruction_text = format!("INX");
        },
        0xEE => {
            instruction_bytes_count = 3;
            let abs = create_u16(operand1, operand2);
            instruction_text = format!("INC ${:04X}", abs);
        },

        0xF0 => {
            instruction_bytes_count = 2;
            let target_address = calculate_target_address((address + instruction_bytes_count) as u16, operand1);
            let label = labeller.request_label_for_branch_target(target_address);
            instruction_text = format!("BEQ {label}");
        },

        unknown_opcode => {
            // We still want to print out what's been processed correctly up until this point,
            // so we'll just tell it to stop here and have some info for the instruction text.
            is_section_complete = true;
            instruction_bytes_count = 1;
            instruction_text = format!("\n***\nUNKNOWN OPCODE AT ADDRESS ${:04X}: ${:02X}\n***", address, unknown_opcode);
        },
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

fn calculate_target_address(address: u16, signed_offset: u8) -> usize {
    let sign_extended_offset = ((signed_offset as i8) as i16) as u16;
    (address as u16).wrapping_add(sign_extended_offset) as usize
}
