use super::models::{ApprovalStatus, SpecSection};

#[derive(Debug, Clone)]
pub struct SpecState {
    pub title: String,
    pub sections: Vec<SpecSection>,
    pub selected_section: usize,
    pub scroll_offset: usize,
    pub show_comments: bool,
    pub editing: bool,
}

impl Default for SpecState {
    fn default() -> Self {
        Self {
            title: String::from("Untitled Spec"),
            sections: Vec::new(),
            selected_section: 0,
            scroll_offset: 0,
            show_comments: false,
            editing: false,
        }
    }
}

impl SpecState {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            ..Default::default()
        }
    }

    pub fn add_section(&mut self, section: SpecSection) {
        self.sections.push(section);
    }

    pub fn current_section(&self) -> Option<&SpecSection> {
        self.sections.get(self.selected_section)
    }

    pub fn current_section_mut(&mut self) -> Option<&mut SpecSection> {
        self.sections.get_mut(self.selected_section)
    }

    pub fn select_next(&mut self) {
        if !self.sections.is_empty() && self.selected_section < self.sections.len() - 1 {
            self.selected_section += 1;
        }
    }

    pub fn select_prev(&mut self) {
        if self.selected_section > 0 {
            self.selected_section -= 1;
        }
    }

    pub fn toggle_collapse(&mut self) {
        if let Some(section) = self.current_section_mut() {
            section.collapsed = !section.collapsed;
        }
    }

    pub fn approve_current(&mut self) {
        if let Some(section) = self.current_section_mut() {
            section.approve();
        }
    }

    pub fn reject_current(&mut self) {
        if let Some(section) = self.current_section_mut() {
            section.reject();
        }
    }

    pub fn request_revision_current(&mut self) {
        if let Some(section) = self.current_section_mut() {
            section.request_revision();
        }
    }

    pub fn all_approved(&self) -> bool {
        !self.sections.is_empty()
            && self
                .sections
                .iter()
                .all(|s| s.status == ApprovalStatus::Approved)
    }

    pub fn approval_progress(&self) -> (usize, usize) {
        let approved = self
            .sections
            .iter()
            .filter(|s| s.status == ApprovalStatus::Approved)
            .count();
        (approved, self.sections.len())
    }
}
