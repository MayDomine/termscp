//! ## KeyBindings Helper
//!
//! Helper module for matching keybindings in components

use tuirealm::event::{Key, KeyEvent};

use crate::config::keybindings::{ExplorerKeyBindings, GlobalKeyBindings, KeyBinding};

/// Check if a key event matches a keybinding
pub fn key_matches(event: &KeyEvent, binding: &KeyBinding) -> bool {
    event.code == binding.key && event.modifiers == binding.modifiers
}


/// Explorer keybinding matcher for file transfer activity
pub struct ExplorerKeyMatcher<'a> {
    pub explorer: &'a ExplorerKeyBindings,
    pub global: &'a GlobalKeyBindings,
}

impl<'a> ExplorerKeyMatcher<'a> {
    pub fn new(explorer: &'a ExplorerKeyBindings, global: &'a GlobalKeyBindings) -> Self {
        Self { explorer, global }
    }

    // Navigation
    pub fn is_move_up(&self, ev: &KeyEvent) -> bool {
        key_matches(ev, &self.explorer.move_up) || ev.code == Key::Up
    }

    pub fn is_move_down(&self, ev: &KeyEvent) -> bool {
        key_matches(ev, &self.explorer.move_down) || ev.code == Key::Down
    }

    pub fn is_move_up_page(&self, ev: &KeyEvent) -> bool {
        key_matches(ev, &self.explorer.move_up_page) || ev.code == Key::PageUp
    }

    pub fn is_move_down_page(&self, ev: &KeyEvent) -> bool {
        key_matches(ev, &self.explorer.move_down_page) || ev.code == Key::PageDown
    }

    pub fn is_move_to_top(&self, ev: &KeyEvent) -> bool {
        key_matches(ev, &self.explorer.move_to_top) || ev.code == Key::Home
    }

    pub fn is_move_to_bottom(&self, ev: &KeyEvent) -> bool {
        key_matches(ev, &self.explorer.move_to_bottom) || ev.code == Key::End
    }

    pub fn is_go_to_parent(&self, ev: &KeyEvent) -> bool {
        key_matches(ev, &self.explorer.go_to_parent)
    }

    pub fn is_go_back(&self, ev: &KeyEvent) -> bool {
        key_matches(ev, &self.explorer.go_back) || ev.code == Key::Backspace
    }

    pub fn is_enter_dir(&self, ev: &KeyEvent) -> bool {
        key_matches(ev, &self.explorer.enter_dir) || ev.code == Key::Enter
    }

    pub fn is_change_panel(&self, ev: &KeyEvent) -> bool {
        key_matches(ev, &self.explorer.change_panel)
            || ev.code == Key::Tab
            || ev.code == Key::BackTab
            || ev.code == Key::Left
            || ev.code == Key::Right
    }

    // File operations
    pub fn is_transfer_file(&self, ev: &KeyEvent) -> bool {
        key_matches(ev, &self.explorer.transfer_file)
    }

    pub fn is_copy_file(&self, ev: &KeyEvent) -> bool {
        key_matches(ev, &self.explorer.copy_file) || ev.code == Key::Function(5)
    }

    pub fn is_rename_file(&self, ev: &KeyEvent) -> bool {
        key_matches(ev, &self.explorer.rename_file) || ev.code == Key::Function(6)
    }

    pub fn is_delete_file(&self, ev: &KeyEvent) -> bool {
        key_matches(ev, &self.explorer.delete_file)
            || ev.code == Key::Delete
            || ev.code == Key::Function(8)
    }

    pub fn is_mkdir(&self, ev: &KeyEvent) -> bool {
        key_matches(ev, &self.explorer.mkdir) || ev.code == Key::Function(7)
    }

    pub fn is_new_file(&self, ev: &KeyEvent) -> bool {
        key_matches(ev, &self.explorer.new_file)
    }

    pub fn is_edit_file(&self, ev: &KeyEvent) -> bool {
        key_matches(ev, &self.explorer.edit_file) || ev.code == Key::Function(4)
    }

    pub fn is_open_file(&self, ev: &KeyEvent) -> bool {
        key_matches(ev, &self.explorer.open_file) || ev.code == Key::Function(3)
    }

    pub fn is_open_with(&self, ev: &KeyEvent) -> bool {
        key_matches(ev, &self.explorer.open_with)
    }

    pub fn is_save_as(&self, ev: &KeyEvent) -> bool {
        key_matches(ev, &self.explorer.save_as) || ev.code == Key::Function(2)
    }

    pub fn is_chmod(&self, ev: &KeyEvent) -> bool {
        key_matches(ev, &self.explorer.chmod)
    }

    pub fn is_symlink(&self, ev: &KeyEvent) -> bool {
        key_matches(ev, &self.explorer.symlink)
    }

    pub fn is_reload_dir(&self, ev: &KeyEvent) -> bool {
        key_matches(ev, &self.explorer.reload_dir)
    }

    // Selection
    pub fn is_mark_file(&self, ev: &KeyEvent) -> bool {
        key_matches(ev, &self.explorer.mark_file)
    }

    pub fn is_mark_all(&self, ev: &KeyEvent) -> bool {
        key_matches(ev, &self.explorer.mark_all)
    }

    pub fn is_unmark_all(&self, ev: &KeyEvent) -> bool {
        key_matches(ev, &self.explorer.unmark_all)
    }

    // View
    pub fn is_toggle_hidden(&self, ev: &KeyEvent) -> bool {
        key_matches(ev, &self.explorer.toggle_hidden)
    }

    pub fn is_file_info(&self, ev: &KeyEvent) -> bool {
        key_matches(ev, &self.explorer.file_info)
    }

    pub fn is_file_size(&self, ev: &KeyEvent) -> bool {
        key_matches(ev, &self.explorer.file_size)
    }

    pub fn is_sorting(&self, ev: &KeyEvent) -> bool {
        key_matches(ev, &self.explorer.sorting)
    }

    pub fn is_filter(&self, ev: &KeyEvent) -> bool {
        key_matches(ev, &self.explorer.filter)
    }

    // Search
    pub fn is_fuzzy_search(&self, ev: &KeyEvent) -> bool {
        key_matches(ev, &self.explorer.fuzzy_search)
    }

    pub fn is_goto_path(&self, ev: &KeyEvent) -> bool {
        key_matches(ev, &self.explorer.goto_path)
    }

    // Misc
    pub fn is_terminal(&self, ev: &KeyEvent) -> bool {
        key_matches(ev, &self.explorer.terminal)
    }

    pub fn is_sync_browsing(&self, ev: &KeyEvent) -> bool {
        key_matches(ev, &self.explorer.sync_browsing)
    }

    pub fn is_watcher(&self, ev: &KeyEvent) -> bool {
        key_matches(ev, &self.explorer.watcher)
    }

    pub fn is_watched_paths(&self, ev: &KeyEvent) -> bool {
        key_matches(ev, &self.explorer.watched_paths)
    }

    pub fn is_pending_queue(&self, ev: &KeyEvent) -> bool {
        key_matches(ev, &self.explorer.pending_queue)
    }

    // Global
    pub fn is_disconnect(&self, ev: &KeyEvent) -> bool {
        key_matches(ev, &self.global.disconnect)
    }
}

