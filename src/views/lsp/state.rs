use super::models::{Diagnostic, HoverInfo, Reference};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LspPanel {
    #[default]
    Diagnostics,
    Hover,
    References,
}

#[derive(Debug, Default)]
pub struct LspState {
    pub diagnostics: Vec<Diagnostic>,
    pub hover: Option<HoverInfo>,
    pub references: Vec<Reference>,
    pub active_panel: LspPanel,
    pub selected_diagnostic: usize,
    pub selected_reference: usize,
    pub scroll_offset: usize,
}

impl LspState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_diagnostics(&mut self, diagnostics: Vec<Diagnostic>) {
        self.diagnostics = diagnostics;
        self.selected_diagnostic = 0;
    }

    pub fn set_hover(&mut self, hover: Option<HoverInfo>) {
        self.hover = hover;
    }

    pub fn set_references(&mut self, refs: Vec<Reference>) {
        self.references = refs;
        self.selected_reference = 0;
    }

    pub fn cycle_panel(&mut self) {
        self.active_panel = match self.active_panel {
            LspPanel::Diagnostics => LspPanel::Hover,
            LspPanel::Hover => LspPanel::References,
            LspPanel::References => LspPanel::Diagnostics,
        };
    }

    pub fn select_next(&mut self) {
        match self.active_panel {
            LspPanel::Diagnostics if !self.diagnostics.is_empty() => {
                self.selected_diagnostic = (self.selected_diagnostic + 1) % self.diagnostics.len();
            }
            LspPanel::References if !self.references.is_empty() => {
                self.selected_reference = (self.selected_reference + 1) % self.references.len();
            }
            _ => {}
        }
    }

    pub fn select_prev(&mut self) {
        match self.active_panel {
            LspPanel::Diagnostics if !self.diagnostics.is_empty() => {
                self.selected_diagnostic = self.selected_diagnostic
                    .checked_sub(1)
                    .unwrap_or(self.diagnostics.len().saturating_sub(1));
            }
            LspPanel::References if !self.references.is_empty() => {
                self.selected_reference = self.selected_reference
                    .checked_sub(1)
                    .unwrap_or(self.references.len().saturating_sub(1));
            }
            _ => {}
        }
    }
}
