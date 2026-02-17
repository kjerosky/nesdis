use crate::labeller::Labeller;

pub fn disassemble_instruction(prg_rom_contents: &[u8], contents_offset: usize, address: usize, labeller: &mut Labeller) -> (bool, usize, String, Option<usize>) {
    let operand1 = prg_rom_contents[contents_offset + 1];
    let operand2 = prg_rom_contents[contents_offset + 2];
    let absolute_address = create_u16(operand1, operand2);
    let absolute_address_formatted = format_absolute_address(absolute_address);
    let operand1_formatted = format!("${:02X}", operand1);

    let mut is_section_complete = false;
    let instruction_text;
    let instruction_bytes_count;
    let mut address_for_later_processing = None;

    // This page was invaluable for figuring out instruction specifics:
    // https://www.masswerk.at/6502/6502_instruction_set.html
    match prg_rom_contents[contents_offset] {
        0x05 => {
            instruction_bytes_count = 2;
            instruction_text = format!("ORA {operand1_formatted}");
        },
        0x09 => {
            instruction_bytes_count = 2;
            instruction_text = format!("ORA #{operand1_formatted}");
        },
        0x0A => {
            instruction_bytes_count = 1;
            instruction_text = format!("ASL A");
        },

        0x10 => {
            instruction_bytes_count = 2;
            let target_address = calculate_target_address((address + instruction_bytes_count) as u16, operand1);
            let label = labeller.request_label_for_branch_target(target_address);
            instruction_text = format!("BPL {label}");
            address_for_later_processing = Some(target_address);
        },
        0x18 => {
            instruction_bytes_count = 1;
            instruction_text = format!("CLC");
        },

        0x20 => {
            instruction_bytes_count = 3;
            let label = labeller.request_label_for_subroutine(absolute_address as usize);
            instruction_text = format!("JSR {label}");
            address_for_later_processing = Some(absolute_address as usize);
        },
        0x29 => {
            instruction_bytes_count = 2;
            instruction_text = format!("AND #{operand1_formatted}");
        },
        0x2A => {
            instruction_bytes_count = 1;
            instruction_text = format!("ROL A");
        },
        0x2C => {
            instruction_bytes_count = 3;
            instruction_text = format!("BIT {absolute_address_formatted}");
        },

        0x30 => {
            instruction_bytes_count = 2;
            let target_address = calculate_target_address((address + instruction_bytes_count) as u16, operand1);
            let label = labeller.request_label_for_branch_target(target_address);
            instruction_text = format!("BMI {label}");
            address_for_later_processing = Some(target_address);
        },
        0x31 => {
            instruction_bytes_count = 2;
            instruction_text = format!("AND ({operand1_formatted}), Y");
        },
        0x38 => {
            instruction_bytes_count = 1;
            instruction_text = format!("SEC");
        },
        0x3D => {
            instruction_bytes_count = 3;
            instruction_text = format!("AND {absolute_address_formatted}, X");
        },

        0x40 => {
            instruction_bytes_count = 1;
            instruction_text = format!("RTI");
            is_section_complete = true;
        },
        0x45 => {
            instruction_bytes_count = 2;
            instruction_text = format!("EOR {operand1_formatted}");
        },
        0x46 => {
            instruction_bytes_count = 2;
            instruction_text = format!("LSR {operand1_formatted}");
        },
        0x48 => {
            instruction_bytes_count = 1;
            instruction_text = format!("PHA");
        },
        0x49 => {
            instruction_bytes_count = 2;
            instruction_text = format!("EOR #{operand1_formatted}");
        },
        0x4A => {
            instruction_bytes_count = 1;
            instruction_text = format!("LSR A");
        },
        0x4C => {
            instruction_bytes_count = 3;
            let label = labeller.request_label_for_jump_target(absolute_address as usize);
            instruction_text = format!("JMP {label}");
            is_section_complete = true;
            address_for_later_processing = Some(absolute_address as usize);
        },

        0x60 => {
            instruction_bytes_count = 1;
            instruction_text = format!("RTS");
            is_section_complete = true;
        },
        0x65 => {
            instruction_bytes_count = 2;
            instruction_text = format!("ADC {operand1_formatted}");
        },
        0x68 => {
            instruction_bytes_count = 1;
            instruction_text = format!("PLA");
        },
        0x69 => {
            instruction_bytes_count = 2;
            instruction_text = format!("ADC #{operand1_formatted}");
        },
        0x6A => {
            instruction_bytes_count = 1;
            instruction_text = format!("ROR A");
        },
        0x6C => {
            instruction_bytes_count = 3;
            instruction_text = format!("JMP ({absolute_address_formatted})");
            is_section_complete = true;
        },
        0x6D => {
            instruction_bytes_count = 3;
            instruction_text = format!("ADC {absolute_address_formatted}");
        },

        0x78 => {
            instruction_bytes_count = 1;
            instruction_text = format!("SEI");
        },
        0x79 => {
            instruction_bytes_count = 3;
            instruction_text = format!("ADC {absolute_address_formatted}, Y");
        },
        0x7E => {
            instruction_bytes_count = 3;
            instruction_text = format!("ROR {absolute_address_formatted}, X");
        },

        0x84 => {
            instruction_bytes_count = 2;
            instruction_text = format!("STY {operand1_formatted}");
        },
        0x85 => {
            instruction_bytes_count = 2;
            instruction_text = format!("STA {operand1_formatted}");
        },
        0x86 => {
            instruction_bytes_count = 2;
            instruction_text = format!("STX {operand1_formatted}");
        },
        0x8A => {
            instruction_bytes_count = 1;
            instruction_text = format!("TXA");
        },
        0x8C => {
            instruction_bytes_count = 3;
            instruction_text = format!("STY {absolute_address_formatted}");
        },
        0x8D => {
            instruction_bytes_count = 3;
            instruction_text = format!("STA {absolute_address_formatted}");
        },
        0x8E => {
            instruction_bytes_count = 3;
            instruction_text = format!("STX {absolute_address_formatted}");
        },
        0x88 => {
            instruction_bytes_count = 1;
            instruction_text = format!("DEY");
        },

        0x90 => {
            instruction_bytes_count = 2;
            let target_address = calculate_target_address((address + instruction_bytes_count) as u16, operand1);
            let label = labeller.request_label_for_branch_target(target_address);
            instruction_text = format!("BCC {label}");
            address_for_later_processing = Some(target_address);
        },
        0x91 => {
            instruction_bytes_count = 2;
            instruction_text = format!("STA ({operand1_formatted}), Y");
        },
        0x98 => {
            instruction_bytes_count = 1;
            instruction_text = format!("TYA");
        },
        0x99 => {
            instruction_bytes_count = 3;
            instruction_text = format!("STA {absolute_address_formatted}, Y");
        },
        0x9A => {
            instruction_bytes_count = 1;
            instruction_text = format!("TXS");
        },
        0x9D => {
            instruction_bytes_count = 3;
            instruction_text = format!("STA {absolute_address_formatted}, X");
        },

        0xA0 => {
            instruction_bytes_count = 2;
            instruction_text = format!("LDY #{operand1_formatted}");
        },
        0xA2 => {
            instruction_bytes_count = 2;
            instruction_text = format!("LDX #{operand1_formatted}");
        },
        0xA4 => {
            instruction_bytes_count = 2;
            instruction_text = format!("LDY {operand1_formatted}");
        },
        0xA5 => {
            instruction_bytes_count = 2;
            instruction_text = format!("LDA {operand1_formatted}");
        },
        0xA6 => {
            instruction_bytes_count = 2;
            instruction_text = format!("LDX {operand1_formatted}");
        },
        0xA8 => {
            instruction_bytes_count = 1;
            instruction_text = format!("TAY");
        },
        0xA9 => {
            instruction_bytes_count = 2;
            instruction_text = format!("LDA #{operand1_formatted}");
        },
        0xAA => {
            instruction_bytes_count = 1;
            instruction_text = format!("TAX");
        },
        0xAC => {
            instruction_bytes_count = 3;
            instruction_text = format!("LDY {absolute_address_formatted}");
        },
        0xAD => {
            instruction_bytes_count = 3;
            instruction_text = format!("LDA {absolute_address_formatted}");
        },
        0xAE => {
            instruction_bytes_count = 3;
            instruction_text = format!("LDX {absolute_address_formatted}");
        },

        0xB0 => {
            instruction_bytes_count = 2;
            let target_address = calculate_target_address((address + instruction_bytes_count) as u16, operand1);
            let label = labeller.request_label_for_branch_target(target_address);
            instruction_text = format!("BCS {label}");
            address_for_later_processing = Some(target_address);
        },
        0xB1 => {
            instruction_bytes_count = 2;
            instruction_text = format!("LDA ({operand1_formatted}), Y");
        },
        0xB9 => {
            instruction_bytes_count = 3;
            instruction_text = format!("LDA {absolute_address_formatted}, Y");
        },
        0xBD => {
            instruction_bytes_count = 3;
            instruction_text = format!("LDA {absolute_address_formatted}, X");
        },
        0xBE => {
            instruction_bytes_count = 3;
            instruction_text = format!("LDX {absolute_address_formatted}, Y");
        },

        0xC0 => {
            instruction_bytes_count = 2;
            instruction_text = format!("CPY #{operand1_formatted}");
        },
        0xC5 => {
            instruction_bytes_count = 2;
            instruction_text = format!("CMP {operand1_formatted}");
        },
        0xC8 => {
            instruction_bytes_count = 1;
            instruction_text = format!("INY");
        },
        0xC9 => {
            instruction_bytes_count = 2;
            instruction_text = format!("CMP #{operand1_formatted}");
        },
        0xCA => {
            instruction_bytes_count = 1;
            instruction_text = format!("DEX");
        },
        0xCC => {
            instruction_bytes_count = 3;
            instruction_text = format!("CPY {absolute_address_formatted}");
        },
        0xCE => {
            instruction_bytes_count = 3;
            instruction_text = format!("DEC {absolute_address_formatted}");
        },

        0xD0 => {
            instruction_bytes_count = 2;
            let target_address = calculate_target_address((address + instruction_bytes_count) as u16, operand1);
            let label = labeller.request_label_for_branch_target(target_address);
            instruction_text = format!("BNE {label}");
            address_for_later_processing = Some(target_address);
        },
        0xD8 => {
            instruction_bytes_count = 1;
            instruction_text = format!("CLD");
        },
        0xDE => {
            instruction_bytes_count = 3;
            instruction_text = format!("DEC {absolute_address_formatted}, X");
        },

        0xE0 => {
            instruction_bytes_count = 2;
            instruction_text = format!("CPX #{operand1_formatted}");
        },
        0xE6 => {
            instruction_bytes_count = 2;
            instruction_text = format!("INC {operand1_formatted}");
        },
        0xE8 => {
            instruction_bytes_count = 1;
            instruction_text = format!("INX");
        },
        0xEE => {
            instruction_bytes_count = 3;
            instruction_text = format!("INC {absolute_address_formatted}");
        },

        0xF0 => {
            instruction_bytes_count = 2;
            let target_address = calculate_target_address((address + instruction_bytes_count) as u16, operand1);
            let label = labeller.request_label_for_branch_target(target_address);
            instruction_text = format!("BEQ {label}");
            address_for_later_processing = Some(target_address);
        },
        0xF9 => {
            instruction_bytes_count = 3;
            instruction_text = format!("SBC {absolute_address_formatted}, Y");
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

    (is_section_complete, instruction_bytes_count, text_line, address_for_later_processing)
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

// ---------------------------------------------------------------------------

fn format_absolute_address(address: u16) -> String {
    // These names are taken from the Mesen emulator, because they're well-named. 🙂
    match address {
        0x2000 => String::from("PpuControl_2000"),
        0x2001 => String::from("PpuMask_2001"),
        0x2002 => String::from("PpuStatus_2002"),
        0x2003 => String::from("OamAddr_2003"),
        0x2004 => String::from("OamData_2004"),
        0x2005 => String::from("PpuScroll_2005"),
        0x2006 => String::from("PpuAddr_2006"),
        0x2007 => String::from("PpuData_2007"),
        0x4000 => String::from("Sq0Duty_4000"),
        0x4001 => String::from("Sq0Sweep_4001"),
        0x4002 => String::from("Sq0Timer_4002"),
        0x4003 => String::from("Sq0Length_4003"),
        0x4004 => String::from("Sq1Duty_4004"),
        0x4005 => String::from("Sq1Sweep_4005"),
        0x4006 => String::from("Sq1Timer_4006"),
        0x4007 => String::from("Sq1Length_4007"),
        0x4008 => String::from("TrgLinear_4008"),
        0x400A => String::from("TrgTimer_400A"),
        0x400B => String::from("TrgLength_400B"),
        0x400C => String::from("NoiseVolume_400C"),
        0x400E => String::from("NoisePeriod_400E"),
        0x400F => String::from("NoiseLength_400F"),
        0x4010 => String::from("DmcFreq_4010"),
        0x4011 => String::from("DmcCounter_4011"),
        0x4012 => String::from("DmcAddress_4012"),
        0x4013 => String::from("DmcLength_4013"),
        0x4014 => String::from("SpriteDma_4014"),
        0x4015 => String::from("ApuStatus_4015"),
        0x4016 => String::from("Ctrl1_4016"),
        0x4017 => String::from("Ctrl2_FrameCtr_4017"),
        non_reserved_address => format!("${:04X}", non_reserved_address),
    }
}
