//! ## Components
//!
//! file transfer activity components

use tui_realm_stdlib::Phantom;
use tuirealm::event::Event;
use tuirealm::{Component, MockComponent, NoUserEvent};

use super::{Msg, PendingActionMsg, TransferMsg, UiMsg};
use crate::config::keybindings::{GlobalKeyBindings, KeyBindings};

// -- export
pub mod keybindings_helper;
mod log;
mod misc;
mod popups;
mod selected_files;
mod terminal;
mod transfer;

pub use misc::FooterBar;
pub use popups::{
    ATTR_FILES, ChmodPopup, CopyPopup, DeletePopup, DisconnectPopup, ErrorPopup, FatalPopup,
    FileInfoPopup, FilterPopup, GotoPopup, KeybindingsPopup, MkdirPopup, NewfilePopup,
    OpenWithPopup, ProgressBarFull, ProgressBarPartial, QuitPopup, RenamePopup, ReplacePopup,
    SaveAsPopup, SortingPopup, StatusBarLocal, StatusBarRemote, SymlinkPopup,
    SyncBrowsingMkdirPopup, WaitPopup, WalkdirWaitPopup, WatchedPathsList, WatcherPopup,
};
pub use transfer::{ExplorerFind, ExplorerFuzzy, ExplorerLocal, ExplorerRemote};

pub use self::log::Log;
pub use self::selected_files::SelectedFilesList;
pub use self::terminal::Terminal;

#[derive(MockComponent)]
pub struct GlobalListener {
    component: Phantom,
    global_keys: GlobalKeyBindings,
}

impl Default for GlobalListener {
    fn default() -> Self {
        Self {
            component: Phantom::default(),
            global_keys: GlobalKeyBindings::default(),
        }
    }
}

impl GlobalListener {
    pub fn new(keybindings: Option<&KeyBindings>) -> Self {
        Self {
            component: Phantom::default(),
            global_keys: keybindings
                .map(|k| k.global.clone())
                .unwrap_or_default(),
        }
    }
}

impl Component<Msg, NoUserEvent> for GlobalListener {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(ref key_ev) => {
                // Check disconnect
                if keybindings_helper::key_matches(key_ev, &self.global_keys.disconnect) {
                    return Some(Msg::Ui(UiMsg::ShowDisconnectPopup));
                }
                // Check quit
                if keybindings_helper::key_matches(key_ev, &self.global_keys.quit)
                    || keybindings_helper::key_matches(key_ev, &self.global_keys.quit_alt)
                {
                    return Some(Msg::Ui(UiMsg::ShowQuitPopup));
                }
                // Check help
                if keybindings_helper::key_matches(key_ev, &self.global_keys.help)
                    || keybindings_helper::key_matches(key_ev, &self.global_keys.help_alt)
                {
                    return Some(Msg::Ui(UiMsg::ShowKeybindingsPopup));
                }
                None
            }
            Event::WindowResize(_, _) => Some(Msg::Ui(UiMsg::WindowResized)),
            _ => None,
        }
    }
}
