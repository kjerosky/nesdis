use std::{fs::File, io::Read, vec};

use crate::instruction::disassemble_instruction;

const NES_HEADER_BYTES: usize = 16;

pub struct Cartridge {
    mapper_id: u8,
    prg_rom_bank_count: u8,
    chr_rom_bank_count: u8,
    prg_rom_contents: Vec<u8>,
    chr_rom_contents: Vec<u8>,
}

// ---------------------------------------------------------------------------

impl Cartridge {
    pub fn load_from_file(filename: &str) -> Self {
        println!("------------------------------------------------------------------------------");

        let mut file = match File::open(filename) {
            Ok(file) => file,
            Err(error) => panic!("[ERROR] Could not open cartridge file: {error}"),
        };

        let mut header = [0u8; NES_HEADER_BYTES];
        if let Err(error) = file.read_exact(&mut header) {
            panic!("[ERROR] Could not read cartridge file header: {error}");
        }

        let mapper_id = (header[7] & 0xF0) | (header[6] >> 4);
        let prg_rom_bank_count = header[4];
        let chr_rom_bank_count = header[5];

        println!("Mapper: {mapper_id}");
        println!("PRG ROM bank count: {prg_rom_bank_count}");
        println!("CHR ROM bank count: {chr_rom_bank_count}");

        let mut prg_rom_contents = vec![0u8; prg_rom_bank_count as usize * 16384];
        if let Err(error) = file.read_exact(&mut prg_rom_contents) {
            panic!("[ERROR] Could not load cartridge from file: {error}");
        };

        let mut chr_rom_contents = vec![0u8; chr_rom_bank_count as usize * 8192];
        if let Err(error) = file.read_exact(&mut chr_rom_contents) {
            panic!("[ERROR] Could not load cartridge from file: {error}");
        };

        Self {
            mapper_id,
            prg_rom_bank_count,
            chr_rom_bank_count,
            prg_rom_contents,
            chr_rom_contents,
        }
    }

    // -----------------------------------------------------------------------

    pub fn disassemble(&self) {
        println!("------------------------------------------------------------------------------");

        if self.mapper_id != 0 {
            panic!("[ERROR] Mapper is {}, but only mapper 0 is supported right now!", self.mapper_id);
        }

        let vectors_base_address: usize = if self.prg_rom_bank_count > 1 { 0x7FFA } else { 0x3FFA };
        let nmi_vector = (self.prg_rom_contents[vectors_base_address + 1] as usize) << 8;
        let nmi_vector = nmi_vector | (self.prg_rom_contents[vectors_base_address] as usize);
        let reset_vector = (self.prg_rom_contents[vectors_base_address + 3] as usize) << 8;
        let reset_vector = reset_vector | (self.prg_rom_contents[vectors_base_address + 2] as usize);
        let irq_vector = (self.prg_rom_contents[vectors_base_address + 5] as usize) << 8;
        let irq_vector = irq_vector | (self.prg_rom_contents[vectors_base_address + 4] as usize);

        println!("  nmi vector: ${:04X}", nmi_vector);
        println!("reset vector: ${:04X}", reset_vector);
        println!("  irq vector: ${:04X}", irq_vector);
        println!("------------------------------------------------------------------------------");

        self.disassemble_from_entry_point(reset_vector, "RESET");
        //todo disassemble for the nmi and irq vectors
    }

    // -----------------------------------------------------------------------

    fn disassemble_from_entry_point(&self, entry_point: usize, entry_point_label: &str) {
        println!("{entry_point_label}: [{:04X}]", entry_point);

        let mut entry_points: Vec<usize> = Vec::new();
        entry_points.push(entry_point);

        while !entry_points.is_empty() {
            let mut current_address = match entry_points.pop() {
                Some(address) => address,
                None => panic!("[ERROR] Attempted to get a new entry point that didn't exist...this shouldn't happen!"),
            };

            let mut is_processing_complete = false;
            while !is_processing_complete {
                let (is_section_complete, current_instruction_bytes, text_line) = disassemble_instruction(
                    &self.prg_rom_contents, current_address - 0x8000, current_address);

                //todo we need to assemble the lines later, but we'll just print the line here for now
                println!("{text_line}");

                is_processing_complete = is_section_complete;
                current_address += current_instruction_bytes;
            }
        }
    }
}
