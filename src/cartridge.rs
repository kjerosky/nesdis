use std::{collections::HashMap, fs::File, io::Read, vec};

use crate::{instruction::disassemble_instruction, labeller::{self, Labeller}};

const NES_HEADER_BYTES: usize = 16;

pub struct Cartridge {
    mapper_id: u8,
    prg_rom_bank_count: u8,
    chr_rom_bank_count: u8,
    prg_rom_contents: Vec<u8>,
    chr_rom_contents: Vec<u8>,

    global_labels: HashMap<usize, String>,
    labeller: Labeller,
    text_lines: HashMap<usize, TextLine>,
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

            global_labels: HashMap::new(),
            labeller: Labeller::new(),
            text_lines: HashMap::new(),
        }
    }

    // -----------------------------------------------------------------------

    pub fn disassemble(&mut self) {
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

    fn disassemble_from_entry_point(&mut self, entry_point: usize, entry_point_label: &str) {
        self.global_labels.insert(entry_point, entry_point_label.to_string());

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
                    &self.prg_rom_contents, current_address - 0x8000, current_address, &mut self.labeller);

                self.text_lines.insert(
                    current_address,
                    TextLine {
                        contents: text_line,
                        bytes: current_instruction_bytes,
                    }
                );

                is_processing_complete = is_section_complete;
                current_address += current_instruction_bytes;
            }
        }
    }

    // -----------------------------------------------------------------------

    pub fn print_disassembly(&self) {
        let mut address = 0usize;
        while address < 65536 {
            if let Some(global_label) = self.global_labels.get(&address) {
                println!("{global_label}: [{:04X}]", address);
            }

            if let Some(branch_label) = self.labeller.get_branch_target_label(address) {
                println!("{branch_label}: [{:04X}]", address);
            }

            if let Some(jump_label) = self.labeller.get_jump_target_label(address) {
                println!("{jump_label}: [{:04X}]", address);
            }

            if let Some(subroutine_label) = self.labeller.get_subroutine_label(address) {
                println!("{subroutine_label}: [{:04X}]", address);
            }

            if let Some(text_line) = self.text_lines.get(&address) {
                println!("{}", text_line.contents);
                address += text_line.bytes;
            } else {
                address += 1;
            }
        }
    }
}

// ---------------------------------------------------------------------------

struct TextLine {
    contents: String,
    bytes: usize,
}
