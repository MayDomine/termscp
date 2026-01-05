//! ## Components
//!
//! auth activity components

use super::{FileTransferProtocol, FormMsg, Msg, UiMsg};
use crate::config::keybindings::{AuthKeyBindings, KeyBindings};

mod bookmarks;
mod form;
mod popup;
mod text;

pub use bookmarks::{
    BookmarkName, BookmarkSavePassword, BookmarksList, DeleteBookmarkPopup, DeleteRecentPopup,
    RecentsList,
};
#[cfg(posix)]
pub use form::InputSmbWorkgroup;
pub use form::{
    HostBridgeProtocolRadio, InputAddress, InputKubeClientCert, InputKubeClientKey,
    InputKubeClusterUrl, InputKubeNamespace, InputKubeUsername, InputLocalDirectory, InputPassword,
    InputPort, InputRemoteDirectory, InputS3AccessKey, InputS3Bucket, InputS3Endpoint,
    InputS3Profile, InputS3Region, InputS3SecretAccessKey, InputS3SecurityToken,
    InputS3SessionToken, InputSmbShare, InputUsername, InputWebDAVUri, RadioS3NewPathStyle,
    RemoteProtocolRadio,
};
pub use popup::{
    ErrorPopup, InfoPopup, InstallUpdatePopup, Keybindings, QuitPopup, ReleaseNotes, WaitPopup,
    WindowSizeError,
};
pub use text::{HelpFooter, NewVersionDisclaimer, Subtitle, Title};
use tui_realm_stdlib::Phantom;
use tuirealm::event::{Event, KeyEvent, NoUserEvent};
use tuirealm::{Component, MockComponent};

fn binding_matches(event: &KeyEvent, binding: &crate::config::keybindings::KeyBinding) -> bool {
    event.code == binding.key && event.modifiers == binding.modifiers
}

// -- global listener

#[derive(MockComponent)]
pub struct GlobalListener {
    component: Phantom,
    auth_keys: AuthKeyBindings,
}

impl Default for GlobalListener {
    fn default() -> Self {
        Self {
            component: Phantom::default(),
            auth_keys: AuthKeyBindings::default(),
        }
    }
}

impl GlobalListener {
    pub fn new(keybindings: Option<&KeyBindings>) -> Self {
        Self {
            component: Phantom::default(),
            auth_keys: keybindings
                .map(|k| k.auth.clone())
                .unwrap_or_default(),
        }
    }
}

impl Component<Msg, NoUserEvent> for GlobalListener {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(ref key_ev) => {
                // Quit
                if binding_matches(key_ev, &self.auth_keys.quit)
                    || binding_matches(key_ev, &self.auth_keys.quit_alt)
                {
                    return Some(Msg::Ui(UiMsg::ShowQuitPopup));
                }
                // Enter setup
                if binding_matches(key_ev, &self.auth_keys.setup) {
                    return Some(Msg::Form(FormMsg::EnterSetup));
                }
                // Show help
                if binding_matches(key_ev, &self.auth_keys.help)
                    || binding_matches(key_ev, &self.auth_keys.help_alt)
                {
                    return Some(Msg::Ui(UiMsg::ShowKeybindingsPopup));
                }
                // Show release notes
                if binding_matches(key_ev, &self.auth_keys.release_notes) {
                    return Some(Msg::Ui(UiMsg::ShowReleaseNotes));
                }
                // Save bookmark
                if binding_matches(key_ev, &self.auth_keys.save_bookmark) {
                    return Some(Msg::Ui(UiMsg::ShowSaveBookmarkPopup));
                }
                None
            }
            Event::WindowResize(_, _) => Some(Msg::Ui(UiMsg::WindowResized)),
            _ => None,
        }
    }
}
