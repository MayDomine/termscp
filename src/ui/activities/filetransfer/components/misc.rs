//! ## Components
//!
//! file transfer activity components

use tui_realm_stdlib::Span;
use tuirealm::props::{Color, TextSpan};
use tuirealm::{Component, Event, MockComponent, NoUserEvent};

use super::Msg;
use crate::config::keybindings::{KeyBinding, KeyBindings};

/// Format a keybinding for display
fn format_key(binding: &KeyBinding) -> String {
    binding.to_string().to_uppercase()
}

#[derive(MockComponent)]
pub struct FooterBar {
    component: Span,
}

impl FooterBar {
    pub fn new(key_color: Color, keybindings: Option<&KeyBindings>) -> Self {
        let spans = if let Some(kb) = keybindings {
            let global = &kb.global;
            let explorer = &kb.explorer;
            vec![
                TextSpan::from(format!("<{}>", format_key(&global.help))).bold().fg(key_color),
                TextSpan::from(" Help "),
                TextSpan::from(format!("<{}>", format_key(&explorer.change_panel))).bold().fg(key_color),
                TextSpan::from(" Tab "),
                TextSpan::from(format!("<{}>", format_key(&explorer.transfer_file))).bold().fg(key_color),
                TextSpan::from(" Transfer "),
                TextSpan::from(format!("<{}>", format_key(&explorer.enter_dir))).bold().fg(key_color),
                TextSpan::from(" Enter "),
                TextSpan::from(format!("<{}>", format_key(&explorer.save_as))).bold().fg(key_color),
                TextSpan::from(" Save "),
                TextSpan::from(format!("<{}>", format_key(&explorer.open_file))).bold().fg(key_color),
                TextSpan::from(" Open "),
                TextSpan::from(format!("<{}>", format_key(&explorer.edit_file))).bold().fg(key_color),
                TextSpan::from(" Edit "),
                TextSpan::from(format!("<{}>", format_key(&explorer.copy_file))).bold().fg(key_color),
                TextSpan::from(" Copy "),
                TextSpan::from(format!("<{}>", format_key(&explorer.rename_file))).bold().fg(key_color),
                TextSpan::from(" Rename "),
                TextSpan::from(format!("<{}>", format_key(&explorer.mkdir))).bold().fg(key_color),
                TextSpan::from(" Mkdir "),
                TextSpan::from(format!("<{}>", format_key(&explorer.delete_file))).bold().fg(key_color),
                TextSpan::from(" Del "),
                TextSpan::from(format!("<{}>", format_key(&global.quit))).bold().fg(key_color),
                TextSpan::from(" Quit "),
            ]
        } else {
            // Default fallback
            vec![
                TextSpan::from("<F1|H>").bold().fg(key_color),
                TextSpan::from(" Help "),
                TextSpan::from("<TAB>").bold().fg(key_color),
                TextSpan::from(" Change tab "),
                TextSpan::from("<SPACE>").bold().fg(key_color),
                TextSpan::from(" Transfer "),
                TextSpan::from("<ENTER>").bold().fg(key_color),
                TextSpan::from(" Enter dir "),
                TextSpan::from("<F2|S>").bold().fg(key_color),
                TextSpan::from(" Save as "),
                TextSpan::from("<F3|V>").bold().fg(key_color),
                TextSpan::from(" View "),
                TextSpan::from("<F4|O>").bold().fg(key_color),
                TextSpan::from(" Edit "),
                TextSpan::from("<F5|C>").bold().fg(key_color),
                TextSpan::from(" Copy "),
                TextSpan::from("<F6|R>").bold().fg(key_color),
                TextSpan::from(" Rename "),
                TextSpan::from("<F7|D>").bold().fg(key_color),
                TextSpan::from(" Make dir "),
                TextSpan::from("<F8|DEL>").bold().fg(key_color),
                TextSpan::from(" Delete "),
                TextSpan::from("<F10|Q>").bold().fg(key_color),
                TextSpan::from(" Quit "),
            ]
        };

        Self {
            component: Span::default().spans(spans),
        }
    }
}

impl Component<Msg, NoUserEvent> for FooterBar {
    fn on(&mut self, _: Event<NoUserEvent>) -> Option<Msg> {
        None
    }
}
