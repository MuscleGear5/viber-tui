#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DiffLineKind {
    #[default]
    Context,
    Added,
    Removed,
    Modified,
}

#[derive(Debug, Clone)]
pub struct DiffLine {
    pub left_num: Option<usize>,
    pub right_num: Option<usize>,
    pub left_content: String,
    pub right_content: String,
    pub kind: DiffLineKind,
}

impl DiffLine {
    pub fn context(left_num: usize, right_num: usize, content: &str) -> Self {
        Self {
            left_num: Some(left_num),
            right_num: Some(right_num),
            left_content: content.to_string(),
            right_content: content.to_string(),
            kind: DiffLineKind::Context,
        }
    }

    pub fn added(right_num: usize, content: &str) -> Self {
        Self {
            left_num: None,
            right_num: Some(right_num),
            left_content: String::new(),
            right_content: content.to_string(),
            kind: DiffLineKind::Added,
        }
    }

    pub fn removed(left_num: usize, content: &str) -> Self {
        Self {
            left_num: Some(left_num),
            right_num: None,
            left_content: content.to_string(),
            right_content: String::new(),
            kind: DiffLineKind::Removed,
        }
    }

    pub fn modified(left_num: usize, right_num: usize, left: &str, right: &str) -> Self {
        Self {
            left_num: Some(left_num),
            right_num: Some(right_num),
            left_content: left.to_string(),
            right_content: right.to_string(),
            kind: DiffLineKind::Modified,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DiffHunk {
    pub left_start: usize,
    pub right_start: usize,
    pub lines: Vec<DiffLine>,
    pub accepted: Option<bool>,
}

impl DiffHunk {
    pub fn new(left_start: usize, right_start: usize) -> Self {
        Self {
            left_start,
            right_start,
            lines: Vec::new(),
            accepted: None,
        }
    }

    pub fn add_line(&mut self, line: DiffLine) {
        self.lines.push(line);
    }

    pub fn accept(&mut self) {
        self.accepted = Some(true);
    }

    pub fn reject(&mut self) {
        self.accepted = Some(false);
    }

    pub fn is_pending(&self) -> bool {
        self.accepted.is_none()
    }
}

#[derive(Debug, Clone)]
pub struct DiffState {
    pub left_path: String,
    pub right_path: String,
    pub hunks: Vec<DiffHunk>,
    pub selected_hunk: usize,
    pub scroll: usize,
}

impl Default for DiffState {
    fn default() -> Self {
        Self {
            left_path: String::new(),
            right_path: String::new(),
            hunks: Vec::new(),
            selected_hunk: 0,
            scroll: 0,
        }
    }
}

impl DiffState {
    pub fn new(left_path: &str, right_path: &str) -> Self {
        Self {
            left_path: left_path.to_string(),
            right_path: right_path.to_string(),
            ..Default::default()
        }
    }

    pub fn add_hunk(&mut self, hunk: DiffHunk) {
        self.hunks.push(hunk);
    }

    pub fn select_next(&mut self) {
        if !self.hunks.is_empty() && self.selected_hunk < self.hunks.len() - 1 {
            self.selected_hunk += 1;
        }
    }

    pub fn select_prev(&mut self) {
        self.selected_hunk = self.selected_hunk.saturating_sub(1);
    }

    pub fn accept_current(&mut self) {
        if let Some(hunk) = self.hunks.get_mut(self.selected_hunk) {
            hunk.accept();
        }
    }

    pub fn reject_current(&mut self) {
        if let Some(hunk) = self.hunks.get_mut(self.selected_hunk) {
            hunk.reject();
        }
    }
}