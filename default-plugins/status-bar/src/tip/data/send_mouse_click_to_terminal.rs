use ansi_term::{
    unstyled_len, ANSIString, ANSIStrings,
    Color::{Fixed, RGB},
    Style,
};

use crate::LinePart;
use zellij_tile::prelude::*;
use zellij_tile_utils::palette_match;

macro_rules! strings {
    ($ANSIStrings:expr) => {{
        let strings: &[ANSIString] = $ANSIStrings;

        let ansi_strings = ANSIStrings(strings);

        LinePart {
            part: format!("{}", ansi_strings),
            len: unstyled_len(&ansi_strings),
        }
    }};
}

pub fn mouse_click_to_terminal_full(help: &ModeInfo) -> LinePart {
    // Tip: SHIFT + <mouse-click> bypasses Zellij and sends the mouse click directly to the terminal
    let green_color = palette_match!(help.style.colors.text_unselected.emphasis_2);
    let orange_color = palette_match!(help.style.colors.text_unselected.emphasis_0);

    strings!(&[
        Style::new().paint(" 提示: "),
        Style::new().fg(orange_color).bold().paint("Shift"),
        Style::new().paint(" + <"),
        Style::new().fg(green_color).bold().paint("鼠标点击"),
        Style::new().paint("> 会绕过 Zellij，并将点击直接发送到终端。"),
    ])
}

pub fn mouse_click_to_terminal_medium(help: &ModeInfo) -> LinePart {
    // Tip: SHIFT + <mouse-click> sends the click directly to the terminal
    let green_color = palette_match!(help.style.colors.text_unselected.emphasis_2);
    let orange_color = palette_match!(help.style.colors.text_unselected.emphasis_0);
    strings!(&[
        Style::new().paint(" 提示: "),
        Style::new().fg(orange_color).bold().paint("Shift"),
        Style::new().paint(" + <"),
        Style::new().fg(green_color).bold().paint("鼠标点击"),
        Style::new().paint("> 会将点击直接发送到终端。"),
    ])
}

pub fn mouse_click_to_terminal_short(help: &ModeInfo) -> LinePart {
    // Tip: SHIFT + <mouse-click>  => sends click to terminal.
    let green_color = palette_match!(help.style.colors.text_unselected.emphasis_2);
    let orange_color = palette_match!(help.style.colors.text_unselected.emphasis_0);

    strings!(&[
        Style::new().paint(" 提示: "),
        Style::new().fg(orange_color).bold().paint("Shift"),
        Style::new().paint(" + <"),
        Style::new().fg(green_color).bold().paint("鼠标点击"),
        Style::new().paint("> => 发送点击到终端。"),
    ])
}
