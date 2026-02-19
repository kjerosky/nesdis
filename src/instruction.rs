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
        0x00 => {
            instruction_bytes_count = 1;
            instruction_text = format!("BRK");
        },
        0x01 => {
            instruction_bytes_count = 2;
            instruction_text = format!("ORA ({operand1_formatted},X)");
        },
        0x05 => {
            instruction_bytes_count = 2;
            instruction_text = format!("ORA {operand1_formatted}");
        },
        0x06 => {
            instruction_bytes_count = 2;
            instruction_text = format!("ASL {operand1_formatted}");
        },
        0x08 => {
            instruction_bytes_count = 1;
            instruction_text = format!("PHP");
        },
        0x09 => {
            instruction_bytes_count = 2;
            instruction_text = format!("ORA #{operand1_formatted}");
        },
        0x0A => {
            instruction_bytes_count = 1;
            instruction_text = format!("ASL A");
        },
        0x0D => {
            instruction_bytes_count = 3;
            instruction_text = format!("ORA {absolute_address_formatted}");
        },
        0x0E => {
            instruction_bytes_count = 3;
            instruction_text = format!("ASL {absolute_address_formatted}");
        },

        0x10 => {
            instruction_bytes_count = 2;
            let target_address = calculate_target_address((address + instruction_bytes_count) as u16, operand1);
            let label = labeller.request_label_for_branch_target(target_address);
            instruction_text = format!("BPL {label}");
            address_for_later_processing = Some(target_address);
        },
        0x11 => {
            instruction_bytes_count = 2;
            instruction_text = format!("ORA ({operand1_formatted}),Y");
        },
        0x15 => {
            instruction_bytes_count = 2;
            instruction_text = format!("ORA {operand1_formatted},X");
        },
        0x16 => {
            instruction_bytes_count = 2;
            instruction_text = format!("ASL {operand1_formatted},X");
        },
        0x18 => {
            instruction_bytes_count = 1;
            instruction_text = format!("CLC");
        },
        0x19 => {
            instruction_bytes_count = 3;
            instruction_text = format!("ORA {absolute_address_formatted},Y");
        },
        0x1D => {
            instruction_bytes_count = 3;
            instruction_text = format!("ORA {absolute_address_formatted},X");
        },
        0x1E => {
            instruction_bytes_count = 3;
            instruction_text = format!("ASL {absolute_address_formatted},X");
        },

        0x20 => {
            instruction_bytes_count = 3;
            let label = labeller.request_label_for_subroutine(absolute_address as usize);
            instruction_text = format!("JSR {label}");
            address_for_later_processing = Some(absolute_address as usize);
        },
        0x21 => {
            instruction_bytes_count = 2;
            instruction_text = format!("AND ({operand1_formatted},X)");
        },
        0x24 => {
            instruction_bytes_count = 2;
            instruction_text = format!("BIT {operand1_formatted}");
        },
        0x25 => {
            instruction_bytes_count = 2;
            instruction_text = format!("AND {operand1_formatted}");
        },
        0x26 => {
            instruction_bytes_count = 2;
            instruction_text = format!("ROL {operand1_formatted}");
        },
        0x28 => {
            instruction_bytes_count = 1;
            instruction_text = format!("PLP");
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
        0x2D => {
            instruction_bytes_count = 3;
            instruction_text = format!("AND {absolute_address_formatted}");
        },
        0x2E => {
            instruction_bytes_count = 3;
            instruction_text = format!("ROL {absolute_address_formatted}");
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
            instruction_text = format!("AND ({operand1_formatted}),Y");
        },
        0x35 => {
            instruction_bytes_count = 2;
            instruction_text = format!("AND {operand1_formatted},X");
        },
        0x36 => {
            instruction_bytes_count = 2;
            instruction_text = format!("ROL {operand1_formatted},X");
        },
        0x38 => {
            instruction_bytes_count = 1;
            instruction_text = format!("SEC");
        },
        0x39 => {
            instruction_bytes_count = 3;
            instruction_text = format!("AND {absolute_address_formatted},Y");
        },
        0x3D => {
            instruction_bytes_count = 3;
            instruction_text = format!("AND {absolute_address_formatted},X");
        },
        0x3E => {
            instruction_bytes_count = 3;
            instruction_text = format!("ROL {absolute_address_formatted},X");
        },

        0x40 => {
            instruction_bytes_count = 1;
            instruction_text = format!("RTI");
            is_section_complete = true;
        },
        0x41 => {
            instruction_bytes_count = 2;
            instruction_text = format!("EOR ({operand1_formatted},X)");
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
        0x4D => {
            instruction_bytes_count = 3;
            instruction_text = format!("EOR {absolute_address_formatted}");
        },
        0x4E => {
            instruction_bytes_count = 3;
            instruction_text = format!("LSR {absolute_address_formatted}");
        },

        0x50 => {
            instruction_bytes_count = 2;
            let target_address = calculate_target_address((address + instruction_bytes_count) as u16, operand1);
            let label = labeller.request_label_for_branch_target(target_address);
            instruction_text = format!("BVC {label}");
            address_for_later_processing = Some(target_address);
        },
        0x51 => {
            instruction_bytes_count = 2;
            instruction_text = format!("EOR ({operand1_formatted}),Y");
        },
        0x55 => {
            instruction_bytes_count = 2;
            instruction_text = format!("EOR {operand1_formatted},X");
        },
        0x56 => {
            instruction_bytes_count = 2;
            instruction_text = format!("LSR {operand1_formatted},X");
        },
        0x58 => {
            instruction_bytes_count = 1;
            instruction_text = format!("CLI");
        },
        0x59 => {
            instruction_bytes_count = 3;
            instruction_text = format!("EOR {absolute_address_formatted},Y");
        },
        0x5D => {
            instruction_bytes_count = 3;
            instruction_text = format!("EOR {absolute_address_formatted},X");
        },
        0x5E => {
            instruction_bytes_count = 3;
            instruction_text = format!("LSR {absolute_address_formatted},X");
        },

        0x60 => {
            instruction_bytes_count = 1;
            instruction_text = format!("RTS");
            is_section_complete = true;
        },
        0x61 => {
            instruction_bytes_count = 2;
            instruction_text = format!("ADC ({operand1_formatted},X)");
        },
        0x65 => {
            instruction_bytes_count = 2;
            instruction_text = format!("ADC {operand1_formatted}");
        },
        0x66 => {
            instruction_bytes_count = 2;
            instruction_text = format!("ROR {operand1_formatted}");
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
        0x6E => {
            instruction_bytes_count = 3;
            instruction_text = format!("ROR {absolute_address_formatted}");
        },

        0x70 => {
            instruction_bytes_count = 2;
            let target_address = calculate_target_address((address + instruction_bytes_count) as u16, operand1);
            let label = labeller.request_label_for_branch_target(target_address);
            instruction_text = format!("BVS {label}");
            address_for_later_processing = Some(target_address);
        },
        0x71 => {
            instruction_bytes_count = 2;
            instruction_text = format!("ADC ({operand1_formatted}),Y");
        },
        0x75 => {
            instruction_bytes_count = 2;
            instruction_text = format!("ADC {operand1_formatted},X");
        },
        0x76 => {
            instruction_bytes_count = 2;
            instruction_text = format!("ROR {operand1_formatted},X");
        },
        0x78 => {
            instruction_bytes_count = 1;
            instruction_text = format!("SEI");
        },
        0x79 => {
            instruction_bytes_count = 3;
            instruction_text = format!("ADC {absolute_address_formatted},Y");
        },
        0x7D => {
            instruction_bytes_count = 3;
            instruction_text = format!("ADC {absolute_address_formatted},X");
        },
        0x7E => {
            instruction_bytes_count = 3;
            instruction_text = format!("ROR {absolute_address_formatted},X");
        },

        0x81 => {
            instruction_bytes_count = 2;
            instruction_text = format!("STA ({operand1_formatted},X)");
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
        0x88 => {
            instruction_bytes_count = 1;
            instruction_text = format!("DEY");
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

        0x90 => {
            instruction_bytes_count = 2;
            let target_address = calculate_target_address((address + instruction_bytes_count) as u16, operand1);
            let label = labeller.request_label_for_branch_target(target_address);
            instruction_text = format!("BCC {label}");
            address_for_later_processing = Some(target_address);
        },
        0x91 => {
            instruction_bytes_count = 2;
            instruction_text = format!("STA ({operand1_formatted}),Y");
        },
        0x94 => {
            instruction_bytes_count = 2;
            instruction_text = format!("STY {operand1_formatted},X");
        },
        0x95 => {
            instruction_bytes_count = 2;
            instruction_text = format!("STA {operand1_formatted},X");
        },
        0x96 => {
            instruction_bytes_count = 2;
            instruction_text = format!("STX {operand1_formatted},Y");
        },
        0x98 => {
            instruction_bytes_count = 1;
            instruction_text = format!("TYA");
        },
        0x99 => {
            instruction_bytes_count = 3;
            instruction_text = format!("STA {absolute_address_formatted},Y");
        },
        0x9A => {
            instruction_bytes_count = 1;
            instruction_text = format!("TXS");
        },
        0x9D => {
            instruction_bytes_count = 3;
            instruction_text = format!("STA {absolute_address_formatted},X");
        },

        0xA0 => {
            instruction_bytes_count = 2;
            instruction_text = format!("LDY #{operand1_formatted}");
        },
        0xA1 => {
            instruction_bytes_count = 2;
            instruction_text = format!("LDA ({operand1_formatted},X)");
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
            instruction_text = format!("LDA ({operand1_formatted}),Y");
        },
        0xB4 => {
            instruction_bytes_count = 2;
            instruction_text = format!("LDY {operand1_formatted},X");
        },
        0xB5 => {
            instruction_bytes_count = 2;
            instruction_text = format!("LDA {operand1_formatted},X");
        },
        0xB6 => {
            instruction_bytes_count = 2;
            instruction_text = format!("LDX {operand1_formatted},Y");
        },
        0xB8 => {
            instruction_bytes_count = 1;
            instruction_text = format!("CLV");
        },
        0xB9 => {
            instruction_bytes_count = 3;
            instruction_text = format!("LDA {absolute_address_formatted},Y");
        },
        0xBA => {
            instruction_bytes_count = 1;
            instruction_text = format!("TSX");
        },
        0xBC => {
            instruction_bytes_count = 3;
            instruction_text = format!("LDY {absolute_address_formatted},X");
        },
        0xBD => {
            instruction_bytes_count = 3;
            instruction_text = format!("LDA {absolute_address_formatted},X");
        },
        0xBE => {
            instruction_bytes_count = 3;
            instruction_text = format!("LDX {absolute_address_formatted},Y");
        },

        0xC0 => {
            instruction_bytes_count = 2;
            instruction_text = format!("CPY #{operand1_formatted}");
        },
        0xC1 => {
            instruction_bytes_count = 2;
            instruction_text = format!("CMP ({operand1_formatted},X)");
        },
        0xC4 => {
            instruction_bytes_count = 2;
            instruction_text = format!("CPY {operand1_formatted}");
        },
        0xC5 => {
            instruction_bytes_count = 2;
            instruction_text = format!("CMP {operand1_formatted}");
        },
        0xC6 => {
            instruction_bytes_count = 2;
            instruction_text = format!("DEC {operand1_formatted}");
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
        0xCD => {
            instruction_bytes_count = 3;
            instruction_text = format!("CMP {absolute_address_formatted}");
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
        0xD1 => {
            instruction_bytes_count = 2;
            instruction_text = format!("CMP ({operand1_formatted}),Y");
        },
        0xD5 => {
            instruction_bytes_count = 2;
            instruction_text = format!("CMP {operand1_formatted},X");
        },
        0xD6 => {
            instruction_bytes_count = 2;
            instruction_text = format!("DEC {operand1_formatted},X");
        },
        0xD8 => {
            instruction_bytes_count = 1;
            instruction_text = format!("CLD");
        },
        0xD9 => {
            instruction_bytes_count = 3;
            instruction_text = format!("CMP {absolute_address_formatted},Y");
        },
        0xDD => {
            instruction_bytes_count = 3;
            instruction_text = format!("CMP {absolute_address_formatted},X");
        },
        0xDE => {
            instruction_bytes_count = 3;
            instruction_text = format!("DEC {absolute_address_formatted},X");
        },

        0xE0 => {
            instruction_bytes_count = 2;
            instruction_text = format!("CPX #{operand1_formatted}");
        },
        0xE1 => {
            instruction_bytes_count = 2;
            instruction_text = format!("SBC ({operand1_formatted},X)");
        },
        0xE4 => {
            instruction_bytes_count = 2;
            instruction_text = format!("CPX {operand1_formatted}");
        },
        0xE5 => {
            instruction_bytes_count = 2;
            instruction_text = format!("SBC {operand1_formatted}");
        },
        0xE6 => {
            instruction_bytes_count = 2;
            instruction_text = format!("INC {operand1_formatted}");
        },
        0xE8 => {
            instruction_bytes_count = 1;
            instruction_text = format!("INX");
        },
        0xE9 => {
            instruction_bytes_count = 2;
            instruction_text = format!("SBC #{operand1_formatted}");
        },
        0xEA => {
            instruction_bytes_count = 1;
            instruction_text = format!("NOP");
        },
        0xEC => {
            instruction_bytes_count = 3;
            instruction_text = format!("CPX {absolute_address_formatted}");
        },
        0xED => {
            instruction_bytes_count = 3;
            instruction_text = format!("SBC {absolute_address_formatted}");
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
        0xF1 => {
            instruction_bytes_count = 2;
            instruction_text = format!("SBC ({operand1_formatted}),Y");
        },
        0xF5 => {
            instruction_bytes_count = 2;
            instruction_text = format!("SBC {operand1_formatted},X");
        },
        0xF6 => {
            instruction_bytes_count = 2;
            instruction_text = format!("INC {operand1_formatted},X");
        },
        0xF8 => {
            instruction_bytes_count = 1;
            instruction_text = format!("SED");
        },
        0xF9 => {
            instruction_bytes_count = 3;
            instruction_text = format!("SBC {absolute_address_formatted},Y");
        },
        0xFD => {
            instruction_bytes_count = 3;
            instruction_text = format!("SBC {absolute_address_formatted},X");
        },
        0xFE => {
            instruction_bytes_count = 3;
            instruction_text = format!("INC {absolute_address_formatted},X");
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

// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_disasm(bytes: [u8; 3], expected: &str) {
        let mut labeller = Labeller::new();
        let (_, _, output, _) =
            disassemble_instruction(&bytes, 0, 0x8000, &mut labeller);

        assert!(
            output.contains(expected),
            "Expected `{}` in `{}`",
            expected,
            output
        );
    }

    #[test] fn adc_imm()  { assert_disasm([0x69,0x12,0], "ADC #$12"); }
    #[test] fn adc_zp()   { assert_disasm([0x65,0x12,0], "ADC $12"); }
    #[test] fn adc_zpx()  { assert_disasm([0x75,0x12,0], "ADC $12,X"); }
    #[test] fn adc_abs()  { assert_disasm([0x6D,0x34,0x12], "ADC $1234"); }
    #[test] fn adc_absx() { assert_disasm([0x7D,0x34,0x12], "ADC $1234,X"); }
    #[test] fn adc_absy() { assert_disasm([0x79,0x34,0x12], "ADC $1234,Y"); }
    #[test] fn adc_indx() { assert_disasm([0x61,0x12,0], "ADC ($12,X)"); }
    #[test] fn adc_indy() { assert_disasm([0x71,0x12,0], "ADC ($12),Y"); }

    #[test] fn and_imm()  { assert_disasm([0x29,0x12,0], "AND #$12"); }
    #[test] fn and_zp()   { assert_disasm([0x25,0x12,0], "AND $12"); }
    #[test] fn and_zpx()  { assert_disasm([0x35,0x12,0], "AND $12,X"); }
    #[test] fn and_abs()  { assert_disasm([0x2D,0x34,0x12], "AND $1234"); }
    #[test] fn and_absx() { assert_disasm([0x3D,0x34,0x12], "AND $1234,X"); }
    #[test] fn and_absy() { assert_disasm([0x39,0x34,0x12], "AND $1234,Y"); }
    #[test] fn and_indx() { assert_disasm([0x21,0x12,0], "AND ($12,X)"); }
    #[test] fn and_indy() { assert_disasm([0x31,0x12,0], "AND ($12),Y"); }

    #[test] fn asl_acc()  { assert_disasm([0x0A,0,0], "ASL A"); }
    #[test] fn asl_zp()   { assert_disasm([0x06,0x12,0], "ASL $12"); }
    #[test] fn asl_zpx()  { assert_disasm([0x16,0x12,0], "ASL $12,X"); }
    #[test] fn asl_abs()  { assert_disasm([0x0E,0x34,0x12], "ASL $1234"); }
    #[test] fn asl_absx() { assert_disasm([0x1E,0x34,0x12], "ASL $1234,X"); }

    #[test] fn bcc() { assert_disasm([0x90,0x12,0], "BCC branch_target_0"); }
    #[test] fn bcs() { assert_disasm([0xB0,0x12,0], "BCS branch_target_0"); }
    #[test] fn beq() { assert_disasm([0xF0,0x12,0], "BEQ branch_target_0"); }
    #[test] fn bmi() { assert_disasm([0x30,0x12,0], "BMI branch_target_0"); }
    #[test] fn bne() { assert_disasm([0xD0,0x12,0], "BNE branch_target_0"); }
    #[test] fn bpl() { assert_disasm([0x10,0x12,0], "BPL branch_target_0"); }
    #[test] fn bvc() { assert_disasm([0x50,0x12,0], "BVC branch_target_0"); }
    #[test] fn bvs() { assert_disasm([0x70,0x12,0], "BVS branch_target_0"); }

    #[test] fn bit_zp()  { assert_disasm([0x24,0x12,0], "BIT $12"); }
    #[test] fn bit_abs() { assert_disasm([0x2C,0x34,0x12], "BIT $1234"); }

    #[test] fn brk() { assert_disasm([0x00,0,0], "BRK"); }

    #[test] fn cmp_imm()  { assert_disasm([0xC9,0x12,0], "CMP #$12"); }
    #[test] fn cmp_zp()   { assert_disasm([0xC5,0x12,0], "CMP $12"); }
    #[test] fn cmp_zpx()  { assert_disasm([0xD5,0x12,0], "CMP $12,X"); }
    #[test] fn cmp_abs()  { assert_disasm([0xCD,0x34,0x12], "CMP $1234"); }
    #[test] fn cmp_absx() { assert_disasm([0xDD,0x34,0x12], "CMP $1234,X"); }
    #[test] fn cmp_absy() { assert_disasm([0xD9,0x34,0x12], "CMP $1234,Y"); }
    #[test] fn cmp_indx() { assert_disasm([0xC1,0x12,0], "CMP ($12,X)"); }
    #[test] fn cmp_indy() { assert_disasm([0xD1,0x12,0], "CMP ($12),Y"); }

    #[test] fn cpx_imm() { assert_disasm([0xE0,0x12,0], "CPX #$12"); }
    #[test] fn cpx_zp()  { assert_disasm([0xE4,0x12,0], "CPX $12"); }
    #[test] fn cpx_abs() { assert_disasm([0xEC,0x34,0x12], "CPX $1234"); }

    #[test] fn cpy_imm() { assert_disasm([0xC0,0x12,0], "CPY #$12"); }
    #[test] fn cpy_zp()  { assert_disasm([0xC4,0x12,0], "CPY $12"); }
    #[test] fn cpy_abs() { assert_disasm([0xCC,0x34,0x12], "CPY $1234"); }

    #[test] fn dec_zp()   { assert_disasm([0xC6,0x12,0], "DEC $12"); }
    #[test] fn dec_zpx()  { assert_disasm([0xD6,0x12,0], "DEC $12,X"); }
    #[test] fn dec_abs()  { assert_disasm([0xCE,0x34,0x12], "DEC $1234"); }
    #[test] fn dec_absx() { assert_disasm([0xDE,0x34,0x12], "DEC $1234,X"); }

    #[test] fn eor_imm()  { assert_disasm([0x49,0x12,0], "EOR #$12"); }
    #[test] fn eor_zp()   { assert_disasm([0x45,0x12,0], "EOR $12"); }
    #[test] fn eor_zpx()  { assert_disasm([0x55,0x12,0], "EOR $12,X"); }
    #[test] fn eor_abs()  { assert_disasm([0x4D,0x34,0x12], "EOR $1234"); }
    #[test] fn eor_absx() { assert_disasm([0x5D,0x34,0x12], "EOR $1234,X"); }
    #[test] fn eor_absy() { assert_disasm([0x59,0x34,0x12], "EOR $1234,Y"); }
    #[test] fn eor_indx() { assert_disasm([0x41,0x12,0], "EOR ($12,X)"); }
    #[test] fn eor_indy() { assert_disasm([0x51,0x12,0], "EOR ($12),Y"); }

    #[test] fn jmp_abs() { assert_disasm([0x4C,0x00,0x12], "JMP jump_target_0"); }
    #[test] fn jmp_ind() { assert_disasm([0x6C,0x34,0x12], "JMP ($1234)"); }
    #[test] fn jsr()     { assert_disasm([0x20,0x00,0x12], "JSR subroutine_0"); }

    #[test] fn rts() { assert_disasm([0x60,0,0], "RTS"); }
    #[test] fn rti() { assert_disasm([0x40,0,0], "RTI"); }

    #[test] fn nop() { assert_disasm([0xEA,0,0], "NOP"); }

    #[test] fn pha() { assert_disasm([0x48,0,0], "PHA"); }
    #[test] fn php() { assert_disasm([0x08,0,0], "PHP"); }
    #[test] fn pla() { assert_disasm([0x68,0,0], "PLA"); }
    #[test] fn plp() { assert_disasm([0x28,0,0], "PLP"); }

    #[test] fn sec() { assert_disasm([0x38,0,0], "SEC"); }
    #[test] fn sed() { assert_disasm([0xF8,0,0], "SED"); }
    #[test] fn sei() { assert_disasm([0x78,0,0], "SEI"); }

    #[test] fn clc() { assert_disasm([0x18,0,0], "CLC"); }
    #[test] fn cld() { assert_disasm([0xD8,0,0], "CLD"); }
    #[test] fn cli() { assert_disasm([0x58,0,0], "CLI"); }
    #[test] fn clv() { assert_disasm([0xB8,0,0], "CLV"); }

    #[test] fn tax() { assert_disasm([0xAA,0,0], "TAX"); }
    #[test] fn tay() { assert_disasm([0xA8,0,0], "TAY"); }
    #[test] fn tsx() { assert_disasm([0xBA,0,0], "TSX"); }
    #[test] fn txa() { assert_disasm([0x8A,0,0], "TXA"); }
    #[test] fn txs() { assert_disasm([0x9A,0,0], "TXS"); }
    #[test] fn tya() { assert_disasm([0x98,0,0], "TYA"); }

    #[test] fn inc_zp()   { assert_disasm([0xE6,0x12,0], "INC $12"); }
    #[test] fn inc_zpx()  { assert_disasm([0xF6,0x12,0], "INC $12,X"); }
    #[test] fn inc_abs()  { assert_disasm([0xEE,0x34,0x12], "INC $1234"); }
    #[test] fn inc_absx() { assert_disasm([0xFE,0x34,0x12], "INC $1234,X"); }

    #[test] fn lda_imm()  { assert_disasm([0xA9,0x12,0], "LDA #$12"); }
    #[test] fn lda_zp()   { assert_disasm([0xA5,0x12,0], "LDA $12"); }
    #[test] fn lda_zpx()  { assert_disasm([0xB5,0x12,0], "LDA $12,X"); }
    #[test] fn lda_abs()  { assert_disasm([0xAD,0x34,0x12], "LDA $1234"); }
    #[test] fn lda_absx() { assert_disasm([0xBD,0x34,0x12], "LDA $1234,X"); }
    #[test] fn lda_absy() { assert_disasm([0xB9,0x34,0x12], "LDA $1234,Y"); }
    #[test] fn lda_indx() { assert_disasm([0xA1,0x12,0], "LDA ($12,X)"); }
    #[test] fn lda_indy() { assert_disasm([0xB1,0x12,0], "LDA ($12),Y"); }

    #[test] fn ldx_imm()  { assert_disasm([0xA2,0x12,0], "LDX #$12"); }
    #[test] fn ldx_zp()   { assert_disasm([0xA6,0x12,0], "LDX $12"); }
    #[test] fn ldx_zpy()  { assert_disasm([0xB6,0x12,0], "LDX $12,Y"); }
    #[test] fn ldx_abs()  { assert_disasm([0xAE,0x34,0x12], "LDX $1234"); }
    #[test] fn ldx_absy() { assert_disasm([0xBE,0x34,0x12], "LDX $1234,Y"); }

    #[test] fn ldy_imm()  { assert_disasm([0xA0,0x12,0], "LDY #$12"); }
    #[test] fn ldy_zp()   { assert_disasm([0xA4,0x12,0], "LDY $12"); }
    #[test] fn ldy_zpx()  { assert_disasm([0xB4,0x12,0], "LDY $12,X"); }
    #[test] fn ldy_abs()  { assert_disasm([0xAC,0x34,0x12], "LDY $1234"); }
    #[test] fn ldy_absx() { assert_disasm([0xBC,0x34,0x12], "LDY $1234,X"); }

    #[test] fn lsr_acc()  { assert_disasm([0x4A,0,0], "LSR A"); }
    #[test] fn lsr_zp()   { assert_disasm([0x46,0x12,0], "LSR $12"); }
    #[test] fn lsr_zpx()  { assert_disasm([0x56,0x12,0], "LSR $12,X"); }
    #[test] fn lsr_abs()  { assert_disasm([0x4E,0x34,0x12], "LSR $1234"); }
    #[test] fn lsr_absx() { assert_disasm([0x5E,0x34,0x12], "LSR $1234,X"); }

    #[test] fn ora_imm()  { assert_disasm([0x09,0x12,0], "ORA #$12"); }
    #[test] fn ora_zp()   { assert_disasm([0x05,0x12,0], "ORA $12"); }
    #[test] fn ora_zpx()  { assert_disasm([0x15,0x12,0], "ORA $12,X"); }
    #[test] fn ora_abs()  { assert_disasm([0x0D,0x34,0x12], "ORA $1234"); }
    #[test] fn ora_absx() { assert_disasm([0x1D,0x34,0x12], "ORA $1234,X"); }
    #[test] fn ora_absy() { assert_disasm([0x19,0x34,0x12], "ORA $1234,Y"); }
    #[test] fn ora_indx() { assert_disasm([0x01,0x12,0], "ORA ($12,X)"); }
    #[test] fn ora_indy() { assert_disasm([0x11,0x12,0], "ORA ($12),Y"); }

    #[test] fn rol_acc()  { assert_disasm([0x2A,0,0], "ROL A"); }
    #[test] fn rol_zp()   { assert_disasm([0x26,0x12,0], "ROL $12"); }
    #[test] fn rol_zpx()  { assert_disasm([0x36,0x12,0], "ROL $12,X"); }
    #[test] fn rol_abs()  { assert_disasm([0x2E,0x34,0x12], "ROL $1234"); }
    #[test] fn rol_absx() { assert_disasm([0x3E,0x34,0x12], "ROL $1234,X"); }

    #[test] fn ror_acc()  { assert_disasm([0x6A,0,0], "ROR A"); }
    #[test] fn ror_zp()   { assert_disasm([0x66,0x12,0], "ROR $12"); }
    #[test] fn ror_zpx()  { assert_disasm([0x76,0x12,0], "ROR $12,X"); }
    #[test] fn ror_abs()  { assert_disasm([0x6E,0x34,0x12], "ROR $1234"); }
    #[test] fn ror_absx() { assert_disasm([0x7E,0x34,0x12], "ROR $1234,X"); }

    #[test] fn sbc_imm()  { assert_disasm([0xE9,0x12,0], "SBC #$12"); }
    #[test] fn sbc_zp()   { assert_disasm([0xE5,0x12,0], "SBC $12"); }
    #[test] fn sbc_zpx()  { assert_disasm([0xF5,0x12,0], "SBC $12,X"); }
    #[test] fn sbc_abs()  { assert_disasm([0xED,0x34,0x12], "SBC $1234"); }
    #[test] fn sbc_absx() { assert_disasm([0xFD,0x34,0x12], "SBC $1234,X"); }
    #[test] fn sbc_absy() { assert_disasm([0xF9,0x34,0x12], "SBC $1234,Y"); }
    #[test] fn sbc_indx() { assert_disasm([0xE1,0x12,0], "SBC ($12,X)"); }
    #[test] fn sbc_indy() { assert_disasm([0xF1,0x12,0], "SBC ($12),Y"); }

    #[test] fn sta_zp()   { assert_disasm([0x85,0x12,0], "STA $12"); }
    #[test] fn sta_zpx()  { assert_disasm([0x95,0x12,0], "STA $12,X"); }
    #[test] fn sta_abs()  { assert_disasm([0x8D,0x34,0x12], "STA $1234"); }
    #[test] fn sta_absx() { assert_disasm([0x9D,0x34,0x12], "STA $1234,X"); }
    #[test] fn sta_absy() { assert_disasm([0x99,0x34,0x12], "STA $1234,Y"); }
    #[test] fn sta_indx() { assert_disasm([0x81,0x12,0], "STA ($12,X)"); }
    #[test] fn sta_indy() { assert_disasm([0x91,0x12,0], "STA ($12),Y"); }

    #[test] fn stx_zp()  { assert_disasm([0x86,0x12,0], "STX $12"); }
    #[test] fn stx_zpy() { assert_disasm([0x96,0x12,0], "STX $12,Y"); }
    #[test] fn stx_abs() { assert_disasm([0x8E,0x34,0x12], "STX $1234"); }

    #[test] fn sty_zp()  { assert_disasm([0x84,0x12,0], "STY $12"); }
    #[test] fn sty_zpx() { assert_disasm([0x94,0x12,0], "STY $12,X"); }
    #[test] fn sty_abs() { assert_disasm([0x8C,0x34,0x12], "STY $1234"); }

    #[test] fn inx() { assert_disasm([0xE8,0,0], "INX"); }
    #[test] fn iny() { assert_disasm([0xC8,0,0], "INY"); }
    #[test] fn dex() { assert_disasm([0xCA,0,0], "DEX"); }
    #[test] fn dey() { assert_disasm([0x88,0,0], "DEY"); }
}
