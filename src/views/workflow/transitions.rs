use crate::views::workflow::models::ViberPhase;

pub struct TransitionRule {
    pub from: ViberPhase,
    pub to: Vec<ViberPhase>,
}

pub fn valid_transitions() -> Vec<TransitionRule> {
    vec![
        TransitionRule {
            from: ViberPhase::Idea,
            to: vec![ViberPhase::Decompose],
        },
        TransitionRule {
            from: ViberPhase::Decompose,
            to: vec![ViberPhase::Questionnaire, ViberPhase::Idea],
        },
        TransitionRule {
            from: ViberPhase::Questionnaire,
            to: vec![ViberPhase::SpecGen, ViberPhase::Decompose],
        },
        TransitionRule {
            from: ViberPhase::SpecGen,
            to: vec![ViberPhase::TaskBreakdown, ViberPhase::Questionnaire],
        },
        TransitionRule {
            from: ViberPhase::TaskBreakdown,
            to: vec![ViberPhase::Scaffold, ViberPhase::SpecGen],
        },
        TransitionRule {
            from: ViberPhase::Scaffold,
            to: vec![ViberPhase::Implementation, ViberPhase::TaskBreakdown],
        },
        TransitionRule {
            from: ViberPhase::Implementation,
            to: vec![ViberPhase::Polish, ViberPhase::Scaffold],
        },
        TransitionRule {
            from: ViberPhase::Polish,
            to: vec![ViberPhase::Validation, ViberPhase::Implementation],
        },
        TransitionRule {
            from: ViberPhase::Validation,
            to: vec![ViberPhase::Implementation, ViberPhase::Polish],
        },
    ]
}

pub fn can_transition(from: ViberPhase, to: ViberPhase) -> bool {
    valid_transitions()
        .iter()
        .find(|r| r.from == from)
        .map(|r| r.to.contains(&to))
        .unwrap_or(false)
}

pub fn next_phase(current: ViberPhase) -> Option<ViberPhase> {
    valid_transitions()
        .iter()
        .find(|r| r.from == current)
        .and_then(|r| r.to.first().copied())
}

pub fn prev_phase(current: ViberPhase) -> Option<ViberPhase> {
    valid_transitions()
        .iter()
        .find(|r| r.from == current)
        .and_then(|r| r.to.get(1).copied())
}

pub fn is_edit_cycle_phase(phase: ViberPhase) -> bool {
    matches!(
        phase,
        ViberPhase::Implementation | ViberPhase::Polish | ViberPhase::Validation
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_forward_transitions() {
        assert!(can_transition(ViberPhase::Idea, ViberPhase::Decompose));
        assert!(can_transition(ViberPhase::Implementation, ViberPhase::Polish));
    }

    #[test]
    fn test_backward_transitions() {
        assert!(can_transition(ViberPhase::Validation, ViberPhase::Implementation));
        assert!(can_transition(ViberPhase::Polish, ViberPhase::Implementation));
    }

    #[test]
    fn test_invalid_transitions() {
        assert!(!can_transition(ViberPhase::Idea, ViberPhase::Validation));
        assert!(!can_transition(ViberPhase::Scaffold, ViberPhase::Idea));
    }

    #[test]
    fn test_edit_cycle_phases() {
        assert!(is_edit_cycle_phase(ViberPhase::Implementation));
        assert!(is_edit_cycle_phase(ViberPhase::Polish));
        assert!(is_edit_cycle_phase(ViberPhase::Validation));
        assert!(!is_edit_cycle_phase(ViberPhase::Idea));
    }

    #[test]
    fn test_next_phase() {
        assert_eq!(next_phase(ViberPhase::Idea), Some(ViberPhase::Decompose));
        assert_eq!(next_phase(ViberPhase::Polish), Some(ViberPhase::Validation));
    }
}
