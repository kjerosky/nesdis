use std::collections::HashMap;

const BRANCH_LABEL_PREFIX: &str = "branch_target";
const JUMP_LABEL_PREFIX: &str = "jump_target";
const SUBROUTINE_LABEL_PREFIX: &str = "subroutine";

pub struct Labeller {
    next_branch_target_id: usize,
    next_jump_target_id: usize,
    next_subroutine_id: usize,

    branch_targets_to_labels: HashMap<usize, String>,
    jump_targets_to_labels: HashMap<usize, String>,
    subroutines_to_labels: HashMap<usize, String>,
}

// ---------------------------------------------------------------------------

impl Labeller {
    pub fn new() -> Self {
        Self {
            next_branch_target_id: 0,
            next_jump_target_id: 0,
            next_subroutine_id: 0,

            branch_targets_to_labels: HashMap::new(),
            jump_targets_to_labels: HashMap::new(),
            subroutines_to_labels: HashMap::new(),
        }
    }

    // -----------------------------------------------------------------------

    pub fn request_label_for_branch_target(&mut self, address: usize) -> String {
        if let Some(existing_label) = self.branch_targets_to_labels.get(&address) {
            return existing_label.clone();
        }

        let label_id = self.next_branch_target_id;
        self.next_branch_target_id += 1;

        let label = format!("{BRANCH_LABEL_PREFIX}_{label_id}");
        self.branch_targets_to_labels.insert(address, label.clone());

        label
    }

    // -----------------------------------------------------------------------

    pub fn request_label_for_jump_target(&mut self, address: usize) -> String {
        if let Some(existing_label) = self.jump_targets_to_labels.get(&address) {
            return existing_label.clone();
        }

        let label_id = self.next_jump_target_id;
        self.next_jump_target_id += 1;

        let label = format!("{JUMP_LABEL_PREFIX}_{label_id}");
        self.jump_targets_to_labels.insert(address, label.clone());

        label
    }

    // -----------------------------------------------------------------------

    pub fn request_label_for_subroutine(&mut self, address: usize) -> String {
        if let Some(existing_label) = self.subroutines_to_labels.get(&address) {
            return existing_label.clone();
        }

        let label_id = self.next_subroutine_id;
        self.next_subroutine_id += 1;

        let label = format!("{SUBROUTINE_LABEL_PREFIX}_{label_id}");
        self.subroutines_to_labels.insert(address, label.clone());

        label
    }

    // -----------------------------------------------------------------------

    pub fn get_branch_target_label(&self, address: usize) -> Option<&String> {
        self.branch_targets_to_labels.get(&address)
    }

    // -----------------------------------------------------------------------

    pub fn get_jump_target_label(&self, address: usize) -> Option<&String> {
        self.jump_targets_to_labels.get(&address)
    }

    // -----------------------------------------------------------------------

    pub fn get_subroutine_label(&self, address: usize) -> Option<&String> {
        self.subroutines_to_labels.get(&address)
    }
}
