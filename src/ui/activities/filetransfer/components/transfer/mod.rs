//! ## Transfer
//!
//! file transfer components

mod file_list;
mod file_list_with_search;

use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::event::{Key, KeyEvent};
use tuirealm::props::{Alignment, Borders, Color, TextSpan};
use tuirealm::{Component, Event, MockComponent, NoUserEvent, State, StateValue};

use self::file_list::FileList;
use self::file_list_with_search::FileListWithSearch;
use super::keybindings_helper::ExplorerKeyMatcher;
use super::{Msg, TransferMsg, UiMsg};
use crate::config::keybindings::{ExplorerKeyBindings, GlobalKeyBindings, KeyBindings};

#[derive(MockComponent)]
pub struct ExplorerFuzzy {
    component: FileListWithSearch,
    explorer_keys: ExplorerKeyBindings,
    global_keys: GlobalKeyBindings,
}

impl ExplorerFuzzy {
    pub fn new<S: AsRef<str>>(
        title: S,
        files: &[&str],
        bg: Color,
        fg: Color,
        hg: Color,
        keybindings: Option<&KeyBindings>,
    ) -> Self {
        let (explorer_keys, global_keys) = keybindings
            .map(|k| (k.explorer.clone(), k.global.clone()))
            .unwrap_or_else(|| {
                (
                    ExplorerKeyBindings::default(),
                    GlobalKeyBindings::default(),
                )
            });

        Self {
            component: FileListWithSearch::default()
                .background(bg)
                .borders(Borders::default().color(hg))
                .foreground(fg)
                .highlighted_color(hg)
                .title(title, Alignment::Left)
                .rows(files.iter().map(|x| vec![TextSpan::from(*x)]).collect()),
            explorer_keys,
            global_keys,
        }
    }

    fn matcher(&self) -> ExplorerKeyMatcher<'_> {
        ExplorerKeyMatcher::new(&self.explorer_keys, &self.global_keys)
    }

    fn on_search(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Left, ..
            }) => {
                self.perform(Cmd::Move(Direction::Left));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Right, ..
            }) => {
                self.perform(Cmd::Move(Direction::Right));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Home, ..
            }) => {
                self.perform(Cmd::GoTo(Position::Begin));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent { code: Key::End, .. }) => {
                self.perform(Cmd::GoTo(Position::End));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Delete, ..
            }) => {
                self.perform(Cmd::Cancel);
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Backspace,
                ..
            }) => {
                self.perform(Cmd::Delete);
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Tab | Key::Up | Key::Down,
                ..
            }) => {
                self.perform(Cmd::Change);
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Char(ch),
                ..
            }) => match self.perform(Cmd::Type(ch)) {
                CmdResult::Changed(State::One(StateValue::String(search))) => {
                    Some(Msg::Ui(UiMsg::FuzzySearch(search)))
                }
                _ => Some(Msg::None),
            },
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => {
                Some(Msg::Ui(UiMsg::CloseFindExplorer))
            }
            _ => None,
        }
    }

    fn on_file_list(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let key_ev = match &ev {
            Event::Keyboard(k) => k,
            _ => return None,
        };
        let matcher = self.matcher();

        // Navigation
        if matcher.is_move_down(key_ev) {
                self.perform(Cmd::Move(Direction::Down));
            return Some(Msg::None);
            }
        if matcher.is_move_up(key_ev) {
                self.perform(Cmd::Move(Direction::Up));
            return Some(Msg::None);
            }
        if matcher.is_move_down_page(key_ev) {
                self.perform(Cmd::Scroll(Direction::Down));
            return Some(Msg::None);
            }
        if matcher.is_move_up_page(key_ev) {
                self.perform(Cmd::Scroll(Direction::Up));
            return Some(Msg::None);
            }
        if matcher.is_move_to_top(key_ev) {
                self.perform(Cmd::GoTo(Position::Begin));
            return Some(Msg::None);
            }
        if matcher.is_move_to_bottom(key_ev) {
                self.perform(Cmd::GoTo(Position::End));
            return Some(Msg::None);
        }

        // Selection
        if matcher.is_mark_all(key_ev) {
                let _ = self.perform(Cmd::Custom(file_list::FILE_LIST_CMD_SELECT_ALL));
            return Some(Msg::Ui(UiMsg::MarkAll));
            }
        if matcher.is_unmark_all(key_ev) {
                let _ = self.perform(Cmd::Custom(file_list::FILE_LIST_CMD_DESELECT_ALL));
            return Some(Msg::Ui(UiMsg::MarkClear));
            }
        if matcher.is_mark_file(key_ev) {
                let CmdResult::Changed(State::One(StateValue::Usize(index))) =
                    self.perform(Cmd::Toggle)
                else {
                    return Some(Msg::None);
                };
            return Some(Msg::Ui(UiMsg::MarkFile(index)));
            }

        // Tab to switch focus
        if key_ev.code == Key::Tab {
                self.perform(Cmd::Change);
            return Some(Msg::None);
        }

        // ESC to close
        if key_ev.code == Key::Esc {
            return Some(Msg::Ui(UiMsg::CloseFindExplorer));
        }

        // Change panel
        if key_ev.code == Key::Left || key_ev.code == Key::Right {
            return Some(Msg::Ui(UiMsg::ChangeTransferWindow));
        }

        // Enter directory
        if matcher.is_enter_dir(key_ev) {
            return Some(Msg::Transfer(TransferMsg::EnterDirectory));
        }

        // Transfer file
        if matcher.is_transfer_file(key_ev) {
            return Some(Msg::Transfer(TransferMsg::TransferFile));
        }

        // Go back
        if matcher.is_go_back(key_ev) {
            return Some(Msg::Transfer(TransferMsg::GoToPreviousDirectory));
        }

        // View operations
        if matcher.is_toggle_hidden(key_ev) {
            return Some(Msg::Ui(UiMsg::ToggleHiddenFiles));
        }
        if matcher.is_sorting(key_ev) {
            return Some(Msg::Ui(UiMsg::ShowFileSortingPopup));
        }
        if matcher.is_delete_file(key_ev) {
            return Some(Msg::Ui(UiMsg::ShowDeletePopup));
        }
        if matcher.is_file_info(key_ev) {
            return Some(Msg::Ui(UiMsg::ShowFileInfoPopup));
        }
        if matcher.is_file_size(key_ev) {
            return Some(Msg::Transfer(TransferMsg::GetFileSize));
        }
        if matcher.is_save_as(key_ev) {
            return Some(Msg::Ui(UiMsg::ShowSaveAsPopup));
        }
        if matcher.is_open_file(key_ev) {
            return Some(Msg::Transfer(TransferMsg::OpenFile));
        }
        if matcher.is_open_with(key_ev) {
            return Some(Msg::Ui(UiMsg::ShowOpenWithPopup));
        }
        if matcher.is_chmod(key_ev) {
            return Some(Msg::Ui(UiMsg::ShowChmodPopup));
        }

        None
    }
}

impl Component<Msg, NoUserEvent> for ExplorerFuzzy {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        match self.component.focus() {
            file_list_with_search::Focus::List => self.on_file_list(ev),
            file_list_with_search::Focus::Search => self.on_search(ev),
        }
    }
}

#[derive(MockComponent)]
pub struct ExplorerFind {
    component: FileList,
    explorer_keys: ExplorerKeyBindings,
    global_keys: GlobalKeyBindings,
}

impl ExplorerFind {
    pub fn new<S: AsRef<str>>(
        title: S,
        files: &[&str],
        bg: Color,
        fg: Color,
        hg: Color,
        keybindings: Option<&KeyBindings>,
    ) -> Self {
        let (explorer_keys, global_keys) = keybindings
            .map(|k| (k.explorer.clone(), k.global.clone()))
            .unwrap_or_else(|| {
                (
                    ExplorerKeyBindings::default(),
                    GlobalKeyBindings::default(),
                )
            });

        Self {
            component: FileList::default()
                .background(bg)
                .borders(Borders::default().color(hg))
                .foreground(fg)
                .highlighted_color(hg)
                .title(title, Alignment::Left)
                .rows(files.iter().map(|x| vec![TextSpan::from(*x)]).collect()),
            explorer_keys,
            global_keys,
        }
    }

    fn matcher(&self) -> ExplorerKeyMatcher<'_> {
        ExplorerKeyMatcher::new(&self.explorer_keys, &self.global_keys)
    }
}

impl Component<Msg, NoUserEvent> for ExplorerFind {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let key_ev = match &ev {
            Event::Keyboard(k) => k,
            _ => return None,
        };
        let matcher = self.matcher();

        // Navigation
        if matcher.is_move_down(key_ev) {
                self.perform(Cmd::Move(Direction::Down));
            return Some(Msg::None);
            }
        if matcher.is_move_up(key_ev) {
                self.perform(Cmd::Move(Direction::Up));
            return Some(Msg::None);
            }
        if matcher.is_move_down_page(key_ev) {
                self.perform(Cmd::Scroll(Direction::Down));
            return Some(Msg::None);
            }
        if matcher.is_move_up_page(key_ev) {
                self.perform(Cmd::Scroll(Direction::Up));
            return Some(Msg::None);
            }
        if matcher.is_move_to_top(key_ev) {
                self.perform(Cmd::GoTo(Position::Begin));
            return Some(Msg::None);
            }
        if matcher.is_move_to_bottom(key_ev) {
                self.perform(Cmd::GoTo(Position::End));
            return Some(Msg::None);
        }

        // Selection
        if matcher.is_mark_all(key_ev) {
                let _ = self.perform(Cmd::Custom(file_list::FILE_LIST_CMD_SELECT_ALL));
            return Some(Msg::Ui(UiMsg::MarkAll));
            }
        if matcher.is_unmark_all(key_ev) {
                let _ = self.perform(Cmd::Custom(file_list::FILE_LIST_CMD_DESELECT_ALL));
            return Some(Msg::Ui(UiMsg::MarkClear));
            }
        if matcher.is_mark_file(key_ev) {
                let CmdResult::Changed(State::One(StateValue::Usize(index))) =
                    self.perform(Cmd::Toggle)
                else {
                    return Some(Msg::None);
                };
            return Some(Msg::Ui(UiMsg::MarkFile(index)));
        }

        // ESC to close
        if key_ev.code == Key::Esc {
            return Some(Msg::Ui(UiMsg::CloseFindExplorer));
        }

        // Change panel
        if matcher.is_change_panel(key_ev) {
            return Some(Msg::Ui(UiMsg::ChangeTransferWindow));
        }

        // Enter directory
        if matcher.is_enter_dir(key_ev) {
            return Some(Msg::Transfer(TransferMsg::EnterDirectory));
        }

        // Transfer file
        if matcher.is_transfer_file(key_ev) {
            return Some(Msg::Transfer(TransferMsg::TransferFile));
        }

        // Go back
        if matcher.is_go_back(key_ev) {
            return Some(Msg::Transfer(TransferMsg::GoToPreviousDirectory));
        }

        // View operations
        if matcher.is_toggle_hidden(key_ev) {
            return Some(Msg::Ui(UiMsg::ToggleHiddenFiles));
        }
        if matcher.is_sorting(key_ev) {
            return Some(Msg::Ui(UiMsg::ShowFileSortingPopup));
        }
        if matcher.is_delete_file(key_ev) {
            return Some(Msg::Ui(UiMsg::ShowDeletePopup));
        }
        if matcher.is_file_info(key_ev) {
            return Some(Msg::Ui(UiMsg::ShowFileInfoPopup));
        }
        if matcher.is_file_size(key_ev) {
            return Some(Msg::Transfer(TransferMsg::GetFileSize));
        }
        if matcher.is_save_as(key_ev) {
            return Some(Msg::Ui(UiMsg::ShowSaveAsPopup));
        }
        if matcher.is_open_file(key_ev) {
            return Some(Msg::Transfer(TransferMsg::OpenFile));
        }
        if matcher.is_open_with(key_ev) {
            return Some(Msg::Ui(UiMsg::ShowOpenWithPopup));
        }
        if matcher.is_chmod(key_ev) {
            return Some(Msg::Ui(UiMsg::ShowChmodPopup));
        }

        None
    }
}

#[derive(MockComponent)]
pub struct ExplorerLocal {
    component: FileList,
    explorer_keys: ExplorerKeyBindings,
    global_keys: GlobalKeyBindings,
}

impl ExplorerLocal {
    pub fn new<S: AsRef<str>>(
        title: S,
        files: &[&str],
        bg: Color,
        fg: Color,
        hg: Color,
        keybindings: Option<&KeyBindings>,
    ) -> Self {
        let (explorer_keys, global_keys) = keybindings
            .map(|k| (k.explorer.clone(), k.global.clone()))
            .unwrap_or_else(|| {
                (
                    ExplorerKeyBindings::default(),
                    GlobalKeyBindings::default(),
                )
            });

        Self {
            component: FileList::default()
                .background(bg)
                .borders(Borders::default().color(hg))
                .foreground(fg)
                .highlighted_color(hg)
                .title(title, Alignment::Left)
                .rows(files.iter().map(|x| vec![TextSpan::from(*x)]).collect())
                .dot_dot(true),
            explorer_keys,
            global_keys,
        }
    }

    fn matcher(&self) -> ExplorerKeyMatcher<'_> {
        ExplorerKeyMatcher::new(&self.explorer_keys, &self.global_keys)
    }
}

impl Component<Msg, NoUserEvent> for ExplorerLocal {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let key_ev = match &ev {
            Event::Keyboard(k) => k,
            _ => return None,
        };
        let matcher = self.matcher();

        // Navigation
        if matcher.is_move_down(key_ev) {
                self.perform(Cmd::Move(Direction::Down));
            return Some(Msg::None);
            }
        if matcher.is_move_up(key_ev) {
                self.perform(Cmd::Move(Direction::Up));
            return Some(Msg::None);
            }
        if matcher.is_move_down_page(key_ev) {
                self.perform(Cmd::Scroll(Direction::Down));
            return Some(Msg::None);
            }
        if matcher.is_move_up_page(key_ev) {
                self.perform(Cmd::Scroll(Direction::Up));
            return Some(Msg::None);
            }
        if matcher.is_move_to_top(key_ev) {
                self.perform(Cmd::GoTo(Position::Begin));
            return Some(Msg::None);
            }
        if matcher.is_move_to_bottom(key_ev) {
                self.perform(Cmd::GoTo(Position::End));
            return Some(Msg::None);
        }

        // Selection
        if matcher.is_mark_all(key_ev) {
                let _ = self.perform(Cmd::Custom(file_list::FILE_LIST_CMD_SELECT_ALL));
            return Some(Msg::Ui(UiMsg::MarkAll));
            }
        if matcher.is_unmark_all(key_ev) {
                let _ = self.perform(Cmd::Custom(file_list::FILE_LIST_CMD_DESELECT_ALL));
            return Some(Msg::Ui(UiMsg::MarkClear));
            }
        if matcher.is_mark_file(key_ev) {
                let CmdResult::Changed(State::One(StateValue::Usize(index))) =
                    self.perform(Cmd::Toggle)
                else {
                    return Some(Msg::None);
                };
            return Some(Msg::Ui(UiMsg::MarkFile(index)));
        }

        // ESC to disconnect
        if matcher.is_disconnect(key_ev) {
            return Some(Msg::Ui(UiMsg::ShowDisconnectPopup));
        }

        // Change panel
        if matcher.is_change_panel(key_ev) {
            return Some(Msg::Ui(UiMsg::ChangeTransferWindow));
        }

        // Go back
        if matcher.is_go_back(key_ev) {
            return Some(Msg::Transfer(TransferMsg::GoToPreviousDirectory));
        }

        // Enter directory or go to parent
        if matcher.is_enter_dir(key_ev) {
                if matches!(self.component.state(), State::One(StateValue::String(_))) {
                return Some(Msg::Transfer(TransferMsg::GoToParentDirectory));
                } else {
                return Some(Msg::Transfer(TransferMsg::EnterDirectory));
            }
        }

        // Transfer file (space by default)
        if matcher.is_transfer_file(key_ev) {
                if matches!(self.component.state(), State::One(StateValue::String(_))) {
                return Some(Msg::None);
                } else {
                return Some(Msg::Transfer(TransferMsg::TransferFile));
            }
        }

        // View operations
        if matcher.is_toggle_hidden(key_ev) {
            return Some(Msg::Ui(UiMsg::ToggleHiddenFiles));
        }
        if matcher.is_sorting(key_ev) {
            return Some(Msg::Ui(UiMsg::ShowFileSortingPopup));
        }

        // File operations
        if matcher.is_copy_file(key_ev) {
            return Some(Msg::Ui(UiMsg::ShowCopyPopup));
        }
        if matcher.is_mkdir(key_ev) {
            return Some(Msg::Ui(UiMsg::ShowMkdirPopup));
        }
        if matcher.is_delete_file(key_ev) {
            return Some(Msg::Ui(UiMsg::ShowDeletePopup));
        }
        if matcher.is_fuzzy_search(key_ev) {
            return Some(Msg::Transfer(TransferMsg::InitFuzzySearch));
        }
        if matcher.is_goto_path(key_ev) {
            return Some(Msg::Ui(UiMsg::ShowGotoPopup));
        }
        if matcher.is_file_info(key_ev) {
            return Some(Msg::Ui(UiMsg::ShowFileInfoPopup));
        }
        if matcher.is_symlink(key_ev) {
            return Some(Msg::Ui(UiMsg::ShowSymlinkPopup));
        }
        if matcher.is_reload_dir(key_ev) {
            return Some(Msg::Transfer(TransferMsg::ReloadDir));
        }
        if matcher.is_new_file(key_ev) {
            return Some(Msg::Ui(UiMsg::ShowNewFilePopup));
        }
        if matcher.is_edit_file(key_ev) {
            return Some(Msg::Transfer(TransferMsg::OpenTextFile));
        }
        if matcher.is_pending_queue(key_ev) {
            return Some(Msg::Ui(UiMsg::GoToTransferQueue));
        }
        if matcher.is_rename_file(key_ev) {
            return Some(Msg::Ui(UiMsg::ShowRenamePopup));
        }
        if matcher.is_file_size(key_ev) {
            return Some(Msg::Transfer(TransferMsg::GetFileSize));
        }
        if matcher.is_save_as(key_ev) {
            return Some(Msg::Ui(UiMsg::ShowSaveAsPopup));
        }
        if matcher.is_watcher(key_ev) {
            return Some(Msg::Ui(UiMsg::ShowWatcherPopup));
        }
        if matcher.is_watched_paths(key_ev) {
            return Some(Msg::Ui(UiMsg::ShowWatchedPathsList));
        }
        if matcher.is_go_to_parent(key_ev) {
            return Some(Msg::Transfer(TransferMsg::GoToParentDirectory));
        }
        if matcher.is_terminal(key_ev) {
            return Some(Msg::Ui(UiMsg::ShowTerminal));
        }
        if matcher.is_sync_browsing(key_ev) {
            return Some(Msg::Ui(UiMsg::ToggleSyncBrowsing));
        }
        if matcher.is_open_file(key_ev) {
            return Some(Msg::Transfer(TransferMsg::OpenFile));
        }
        if matcher.is_open_with(key_ev) {
            return Some(Msg::Ui(UiMsg::ShowOpenWithPopup));
        }
        if matcher.is_chmod(key_ev) {
            return Some(Msg::Ui(UiMsg::ShowChmodPopup));
        }
        if matcher.is_filter(key_ev) {
            return Some(Msg::Ui(UiMsg::ShowFilterPopup));
        }

        None
    }
}

#[derive(MockComponent)]
pub struct ExplorerRemote {
    component: FileList,
    explorer_keys: ExplorerKeyBindings,
    global_keys: GlobalKeyBindings,
}

impl ExplorerRemote {
    pub fn new<S: AsRef<str>>(
        title: S,
        files: &[&str],
        bg: Color,
        fg: Color,
        hg: Color,
        keybindings: Option<&KeyBindings>,
    ) -> Self {
        let (explorer_keys, global_keys) = keybindings
            .map(|k| (k.explorer.clone(), k.global.clone()))
            .unwrap_or_else(|| {
                (
                    ExplorerKeyBindings::default(),
                    GlobalKeyBindings::default(),
                )
            });

        Self {
            component: FileList::default()
                .background(bg)
                .borders(Borders::default().color(hg))
                .foreground(fg)
                .highlighted_color(hg)
                .title(title, Alignment::Left)
                .rows(files.iter().map(|x| vec![TextSpan::from(*x)]).collect())
                .dot_dot(true),
            explorer_keys,
            global_keys,
        }
    }

    fn matcher(&self) -> ExplorerKeyMatcher<'_> {
        ExplorerKeyMatcher::new(&self.explorer_keys, &self.global_keys)
    }
}

impl Component<Msg, NoUserEvent> for ExplorerRemote {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let key_ev = match &ev {
            Event::Keyboard(k) => k,
            _ => return None,
        };
        let matcher = self.matcher();

        // Navigation
        if matcher.is_move_down(key_ev) {
                self.perform(Cmd::Move(Direction::Down));
            return Some(Msg::None);
            }
        if matcher.is_move_up(key_ev) {
                self.perform(Cmd::Move(Direction::Up));
            return Some(Msg::None);
            }
        if matcher.is_move_down_page(key_ev) {
                self.perform(Cmd::Scroll(Direction::Down));
            return Some(Msg::None);
            }
        if matcher.is_move_up_page(key_ev) {
                self.perform(Cmd::Scroll(Direction::Up));
            return Some(Msg::None);
            }
        if matcher.is_move_to_top(key_ev) {
                self.perform(Cmd::GoTo(Position::Begin));
            return Some(Msg::None);
            }
        if matcher.is_move_to_bottom(key_ev) {
                self.perform(Cmd::GoTo(Position::End));
            return Some(Msg::None);
        }

        // Selection
        if matcher.is_mark_all(key_ev) {
                let _ = self.perform(Cmd::Custom(file_list::FILE_LIST_CMD_SELECT_ALL));
            return Some(Msg::Ui(UiMsg::MarkAll));
            }
        if matcher.is_unmark_all(key_ev) {
                let _ = self.perform(Cmd::Custom(file_list::FILE_LIST_CMD_DESELECT_ALL));
            return Some(Msg::Ui(UiMsg::MarkClear));
            }
        if matcher.is_mark_file(key_ev) {
                let CmdResult::Changed(State::One(StateValue::Usize(index))) =
                    self.perform(Cmd::Toggle)
                else {
                    return Some(Msg::None);
                };
            return Some(Msg::Ui(UiMsg::MarkFile(index)));
        }

        // ESC to disconnect
        if matcher.is_disconnect(key_ev) {
            return Some(Msg::Ui(UiMsg::ShowDisconnectPopup));
        }

        // Change panel
        if matcher.is_change_panel(key_ev) {
            return Some(Msg::Ui(UiMsg::ChangeTransferWindow));
        }

        // Go back
        if matcher.is_go_back(key_ev) {
            return Some(Msg::Transfer(TransferMsg::GoToPreviousDirectory));
        }

        // Enter directory or go to parent
        if matcher.is_enter_dir(key_ev) {
                if matches!(self.component.state(), State::One(StateValue::String(_))) {
                return Some(Msg::Transfer(TransferMsg::GoToParentDirectory));
                } else {
                return Some(Msg::Transfer(TransferMsg::EnterDirectory));
            }
        }

        // Transfer file (space by default)
        if matcher.is_transfer_file(key_ev) {
                if matches!(self.component.state(), State::One(StateValue::String(_))) {
                return Some(Msg::None);
                } else {
                return Some(Msg::Transfer(TransferMsg::TransferFile));
            }
        }

        // View operations
        if matcher.is_toggle_hidden(key_ev) {
            return Some(Msg::Ui(UiMsg::ToggleHiddenFiles));
        }
        if matcher.is_sorting(key_ev) {
            return Some(Msg::Ui(UiMsg::ShowFileSortingPopup));
        }

        // File operations
        if matcher.is_copy_file(key_ev) {
            return Some(Msg::Ui(UiMsg::ShowCopyPopup));
        }
        if matcher.is_mkdir(key_ev) {
            return Some(Msg::Ui(UiMsg::ShowMkdirPopup));
        }
        if matcher.is_delete_file(key_ev) {
            return Some(Msg::Ui(UiMsg::ShowDeletePopup));
        }
        if matcher.is_fuzzy_search(key_ev) {
            return Some(Msg::Transfer(TransferMsg::InitFuzzySearch));
        }
        if matcher.is_goto_path(key_ev) {
            return Some(Msg::Ui(UiMsg::ShowGotoPopup));
        }
        if matcher.is_file_info(key_ev) {
            return Some(Msg::Ui(UiMsg::ShowFileInfoPopup));
        }
        if matcher.is_symlink(key_ev) {
            return Some(Msg::Ui(UiMsg::ShowSymlinkPopup));
        }
        if matcher.is_reload_dir(key_ev) {
            return Some(Msg::Transfer(TransferMsg::ReloadDir));
        }
        if matcher.is_new_file(key_ev) {
            return Some(Msg::Ui(UiMsg::ShowNewFilePopup));
        }
        if matcher.is_edit_file(key_ev) {
            return Some(Msg::Transfer(TransferMsg::OpenTextFile));
        }
        if matcher.is_pending_queue(key_ev) {
            return Some(Msg::Ui(UiMsg::GoToTransferQueue));
        }
        if matcher.is_rename_file(key_ev) {
            return Some(Msg::Ui(UiMsg::ShowRenamePopup));
        }
        if matcher.is_file_size(key_ev) {
            return Some(Msg::Transfer(TransferMsg::GetFileSize));
        }
        if matcher.is_save_as(key_ev) {
            return Some(Msg::Ui(UiMsg::ShowSaveAsPopup));
        }
        if matcher.is_watcher(key_ev) {
            return Some(Msg::Ui(UiMsg::ShowWatcherPopup));
        }
        if matcher.is_watched_paths(key_ev) {
            return Some(Msg::Ui(UiMsg::ShowWatchedPathsList));
        }
        if matcher.is_go_to_parent(key_ev) {
            return Some(Msg::Transfer(TransferMsg::GoToParentDirectory));
        }
        if matcher.is_terminal(key_ev) {
            return Some(Msg::Ui(UiMsg::ShowTerminal));
        }
        if matcher.is_sync_browsing(key_ev) {
            return Some(Msg::Ui(UiMsg::ToggleSyncBrowsing));
        }
        if matcher.is_open_file(key_ev) {
            return Some(Msg::Transfer(TransferMsg::OpenFile));
        }
        if matcher.is_open_with(key_ev) {
            return Some(Msg::Ui(UiMsg::ShowOpenWithPopup));
        }
        if matcher.is_chmod(key_ev) {
            return Some(Msg::Ui(UiMsg::ShowChmodPopup));
        }
        if matcher.is_filter(key_ev) {
            return Some(Msg::Ui(UiMsg::ShowFilterPopup));
        }

        None
    }
}
