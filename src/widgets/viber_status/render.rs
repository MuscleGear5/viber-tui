use ratatui::prelude::*;
use crate::theme::{palette, AnimationState};
use super::models::{VibeLevel, ViberPhase};
use super::state::ViberState;

const VIBER_EYE_FRAMES: [&str; 8] = [
    "\u{F06D0}", // nf-md-eye (open)
    "\u{F06D1}", // nf-md-eye_outline
    "\u{F06D0}", // nf-md-eye
    "\u{F06D2}", // nf-md-eye_off (blink)
    "\u{F06D0}", // nf-md-eye
    "\u{F06D1}", // nf-md-eye_outline
    "\u{F06D0}", // nf-md-eye
    "\u{F06D0}", // nf-md-eye
];

pub fn viber_eye(animation: &AnimationState) -> &'static str {
    VIBER_EYE_FRAMES[animation.viber_eye_frame()]
}

pub fn phase_line(state: &ViberState) -> Line<'static> {
    let phase = &state.phase;
    let mut spans = vec![
        Span::styled(phase.icon(), Style::default().fg(palette::VIBER)),
        Span::raw(" "),
    ];

    if let Some(num) = phase.number() {
        spans.push(Span::styled(
            format!("P{}", num),
            Style::default().fg(palette::TEXT_SECONDARY),
        ));
        spans.push(Span::raw(" "));
    }

    spans.push(Span::styled(
        phase.label(),
        Style::default().fg(palette::TEXT_PRIMARY).bold(),
    ));

    Line::from(spans)
}

pub fn agents_line(state: &ViberState) -> Line<'static> {
    let color = if state.active_agents > 0 {
        palette::VIBER
    } else {
        palette::TEXT_MUTED
    };

    Line::from(vec![
        Span::styled("\u{F0C0}", Style::default().fg(color)), // nf-fa-users
        Span::raw(" "),
        Span::styled(
            format!("{}/{}", state.active_agents, state.total_agents),
            Style::default().fg(palette::TEXT_PRIMARY),
        ),
        Span::styled(" agents", Style::default().fg(palette::TEXT_SECONDARY)),
    ])
}

pub fn vibe_line(state: &ViberState) -> Line<'static> {
    let (color, pulse) = match state.vibe_level {
        VibeLevel::Nominal => (palette::SUCCESS, false),
        VibeLevel::Drifting => (palette::VIBER_DRIFT, false),
        VibeLevel::Warning => (palette::WARNING, true),
        VibeLevel::Critical => (palette::VIBER_DANGER, true),
    };

    let style = if pulse {
        Style::default().fg(color).bold()
    } else {
        Style::default().fg(color)
    };

    Line::from(vec![
        Span::styled(state.vibe_level.icon(), style),
        Span::raw(" "),
        Span::styled(state.vibe_level.label(), style),
        Span::raw(" "),
        Span::styled(
            format!("{}%", state.compliance_percent()),
            Style::default().fg(palette::TEXT_SECONDARY),
        ),
    ])
}

pub fn intervention_line(state: &ViberState) -> Option<Line<'static>> {
    state.active_power.map(|power| {
        Line::from(vec![
            Span::styled(
                power.icon(),
                Style::default().fg(palette::VIBER_DANGER).bold(),
            ),
            Span::raw(" "),
            Span::styled(
                power.label(),
                Style::default().fg(palette::VIBER_DANGER).bold(),
            ),
        ])
    })
}

pub fn eye_color(state: &ViberState, animation: &AnimationState) -> Color {
    if state.is_intervening {
        let intensity = animation.pulse();
        if intensity > 0.5 {
            palette::VIBER_DANGER
        } else {
            palette::VIBER
        }
    } else {
        match state.vibe_level {
            VibeLevel::Nominal => palette::VIBER,
            VibeLevel::Drifting => palette::VIBER_DRIFT,
            VibeLevel::Warning => palette::WARNING,
            VibeLevel::Critical => palette::VIBER_DANGER,
        }
    }
}

pub fn phase_color(phase: &ViberPhase) -> Color {
    match phase {
        ViberPhase::Idle => palette::TEXT_MUTED,
        ViberPhase::IdeaCapture | ViberPhase::Decomposition => palette::PURPLE,
        ViberPhase::Questionnaire | ViberPhase::SpecGeneration => palette::BLUE,
        ViberPhase::TaskDecomposition | ViberPhase::Scaffold => palette::CYAN,
        ViberPhase::Implementation | ViberPhase::Polish => palette::GREEN,
        ViberPhase::Validation | ViberPhase::Delivery => palette::SUCCESS,
    }
}
