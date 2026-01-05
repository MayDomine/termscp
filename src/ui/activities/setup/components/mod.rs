//! ## Components
//!
//! setup activity components

use super::{CommonMsg, ConfigMsg, Msg, SshMsg, ThemeMsg, ViewLayout};
use crate::config::keybindings::{KeyBindings, SetupKeyBindings};

mod commons;
mod config;
mod ssh;
mod theme;

pub(super) use commons::{ErrorPopup, Footer, Header, Keybindings, QuitPopup, SavePopup};
pub(super) use config::{
    CheckUpdates, DefaultProtocol, GroupDirs, HiddenFiles, LocalFileFmt, NotificationsEnabled,
    NotificationsThreshold, PromptOnFileReplace, RemoteFileFmt, SshConfig, TextEditor,
};
pub(super) use ssh::{DelSshKeyPopup, SshHost, SshKeys, SshUsername};
pub(super) use theme::*;
use tui_realm_stdlib::Phantom;
use tuirealm::event::{Event, KeyEvent, NoUserEvent};
use tuirealm::{Component, MockComponent};

// -- helper function
fn binding_matches(event: &KeyEvent, binding: &crate::config::keybindings::KeyBinding) -> bool {
    event.code == binding.key && event.modifiers == binding.modifiers
}

// -- global listener

#[derive(MockComponent)]
pub struct GlobalListener {
    component: Phantom,
    setup_keys: SetupKeyBindings,
}

impl Default for GlobalListener {
    fn default() -> Self {
        Self {
            component: Phantom::default(),
            setup_keys: SetupKeyBindings::default(),
        }
    }
}

impl GlobalListener {
    pub fn new(keybindings: Option<&KeyBindings>) -> Self {
        Self {
            component: Phantom::default(),
            setup_keys: keybindings
                .map(|k| k.setup.clone())
                .unwrap_or_default(),
        }
    }
}

impl Component<Msg, NoUserEvent> for GlobalListener {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(ref key_ev) => {
                // Quit
                if binding_matches(key_ev, &self.setup_keys.quit)
                    || binding_matches(key_ev, &self.setup_keys.quit_alt)
                {
                    return Some(Msg::Common(CommonMsg::ShowQuitPopup));
                }
                // Change tab
                if binding_matches(key_ev, &self.setup_keys.change_tab) {
                    return Some(Msg::Common(CommonMsg::ChangeLayout));
                }
                // Show help
                if binding_matches(key_ev, &self.setup_keys.help)
                    || binding_matches(key_ev, &self.setup_keys.help_alt)
                {
                    return Some(Msg::Common(CommonMsg::ShowKeybindings));
                }
                // Revert changes
                if binding_matches(key_ev, &self.setup_keys.revert) {
                    return Some(Msg::Common(CommonMsg::RevertChanges));
                }
                // Save
                if binding_matches(key_ev, &self.setup_keys.save)
                    || binding_matches(key_ev, &self.setup_keys.save_alt)
                {
                    return Some(Msg::Common(CommonMsg::ShowSavePopup));
                }
                None
            }
            Event::WindowResize(_, _) => Some(Msg::Common(CommonMsg::WindowResized)),
            _ => None,
        }
    }
}
