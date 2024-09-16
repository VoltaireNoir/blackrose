//! penrose :: minimal configuration
//!
//! This file will give you a functional if incredibly minimal window manager that
//! has multiple workspaces and simple client / workspace movement.
use penrose::{
    builtin::{
        actions::{
            exit,
            floating::{sink_focused, MouseDragHandler, MouseResizeHandler},
            modify_with, send_layout_message, spawn,
        },
        layout::{
            messages::{ExpandMain, IncMain, Mirror, ShrinkMain, UnwrapTransformer},
            transformers::{Gaps, ReserveTop},
            MainAndStack, Monocle,
        },
    },
    core::{
        bindings::{
            click_handler, parse_keybindings_with_xmodmap, KeyEventHandler, MouseEventHandler,
            MouseState,
        },
        layout::LayoutStack,
        Config, WindowManager,
    },
    extensions::hooks::add_ewmh_hooks,
    map, stack,
    x11rb::RustConn,
    Result,
};

use penrose_ui::{Position, TextStyle};
use std::collections::HashMap;
use tracing_subscriber::{self, prelude::*};

const MAX_MAIN: u32 = 1;
const RATIO: f32 = 0.6;
const RATIO_STEP: f32 = 0.1;
const OUTER_PX: u32 = 5;
const INNER_PX: u32 = 5;
const BAR_HEIGHT_PX: u32 = 20;

const FONT: &str = "FiraCode Nerd Font";
const BLACK: u32 = 0x282828ff;
const WHITE: u32 = 0xebdbb2ff;
const GREY: u32 = 0x3c3836ff;
const BLUE: u32 = 0x458588ff;

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .finish()
        .init();

    let conn = RustConn::new()?;
    let key_bindings = parse_keybindings_with_xmodmap(raw_key_bindings())?;
    let wm = WindowManager::new(config(), key_bindings, mouse_bindings(), conn)?;

    add_status_bar(wm).run()
}

fn raw_key_bindings() -> HashMap<String, Box<dyn KeyEventHandler<RustConn>>> {
    let mut raw_bindings = map! {
        map_keys: |k: &str| k.to_string();

        "M-j" => modify_with(|cs| cs.focus_down()),
        "M-k" => modify_with(|cs| cs.focus_up()),
        "M-w" => modify_with(|cs| cs.kill_focused()),
        "M-S-Return" => modify_with(|cs| cs.rotate_focus_to_head()),
        "M-m" => modify_with(|cs| cs.focus_head()),
        "M-S-j" => modify_with(|cs| cs.swap_down()),
        "M-S-k" => modify_with(|cs| cs.swap_up()),
        "M-Tab" => modify_with(|cs| cs.toggle_tag()),
        "M-bracketright" => modify_with(|cs| cs.next_screen()),
        "M-bracketleft" => modify_with(|cs| cs.previous_screen()),
        "M-space" => modify_with(|cs| cs.next_layout()),
        "M-S-space" => modify_with(|cs| cs.previous_layout()),
        "M-period" => send_layout_message(|| IncMain(1)),
        "M-comma" => send_layout_message(|| IncMain(-1)),
        "M-l" => send_layout_message(|| ExpandMain),
        "M-h" => send_layout_message(|| ShrinkMain),
        "M-n" => send_layout_message(|| UnwrapTransformer),
        "M-slash" => send_layout_message(|| Mirror),
        "M-p" => spawn("dmenu_run"),
        "M-Return" => spawn("wezterm"),
        "M-S-q" => exit(),
    };

    for tag in &["1", "2", "3", "4", "5", "6", "7", "8", "9"] {
        raw_bindings.extend([
            (
                format!("M-{tag}"),
                modify_with(move |client_set| client_set.focus_tag(tag)),
            ),
            (
                format!("M-S-{tag}"),
                modify_with(move |client_set| client_set.move_focused_to_tag(tag)),
            ),
        ]);
    }

    raw_bindings
}

fn mouse_bindings() -> HashMap<MouseState, Box<dyn MouseEventHandler<RustConn>>> {
    use penrose::core::bindings::{
        ModifierKey::{Meta, Shift},
        MouseButton::{Left, Middle, Right},
    };

    map! {
        map_keys: |(button, modifiers)| MouseState { button, modifiers };

        (Left, vec![Shift, Meta]) => MouseDragHandler::boxed_default(),
        (Right, vec![Shift, Meta]) => MouseResizeHandler::boxed_default(),
        (Middle, vec![Shift, Meta]) => click_handler(sink_focused()),
    }
}

fn config() -> Config<RustConn> {
    let mut config = Config {
        default_layouts: layouts(),
        ..Default::default()
    };
    config.compose_or_set_startup_hook(blackrose::hooks::startup_progs);
    config.compose_or_set_manage_hook(blackrose::hooks::manage_place_at_tail);
    add_ewmh_hooks(config)
}

fn layouts() -> LayoutStack {
    stack!(
        MainAndStack::side(MAX_MAIN, RATIO, RATIO_STEP),
        Monocle::boxed()
    )
    .map(|layout| ReserveTop::wrap(Gaps::wrap(layout, OUTER_PX, INNER_PX), BAR_HEIGHT_PX))
}

fn add_status_bar(wm: WindowManager<RustConn>) -> WindowManager<RustConn> {
    let style = TextStyle {
        fg: WHITE.into(),
        bg: Some(BLACK.into()),
        padding: (2, 2),
    };
    let bar =
        penrose_ui::status_bar(BAR_HEIGHT_PX, FONT, 8, style, BLUE, GREY, Position::Top).unwrap();
    bar.add_to(wm)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bindings_parse_correctly_with_xmodmap() {
        let res = parse_keybindings_with_xmodmap(raw_key_bindings());

        if let Err(e) = res {
            panic!("{e}");
        }
    }
}
