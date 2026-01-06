//! ## Keybindings
//!
//! `keybindings` is the module which provides the keybindings configurations and the serializers

use std::fmt;
use std::str::FromStr;

use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use tuirealm::event::{Key, KeyModifiers};

/// A single key binding that can be serialized/deserialized
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct KeyBinding {
    pub key: Key,
    pub modifiers: KeyModifiers,
}

impl KeyBinding {
    #[allow(dead_code)]
    pub fn new(key: Key, modifiers: KeyModifiers) -> Self {
        Self { key, modifiers }
    }

    pub fn simple(key: Key) -> Self {
        Self {
            key,
            modifiers: KeyModifiers::NONE,
        }
    }

    pub fn ctrl(key: Key) -> Self {
        Self {
            key,
            modifiers: KeyModifiers::CONTROL,
        }
    }

    pub fn alt(key: Key) -> Self {
        Self {
            key,
            modifiers: KeyModifiers::ALT,
        }
    }

    #[allow(dead_code)]
    pub fn shift(key: Key) -> Self {
        Self {
            key,
            modifiers: KeyModifiers::SHIFT,
        }
    }

    /// Check if this keybinding matches the given key event
    #[allow(dead_code)]
    pub fn matches(&self, key: Key, modifiers: KeyModifiers) -> bool {
        self.key == key && self.modifiers == modifiers
    }
}

impl Default for KeyBinding {
    fn default() -> Self {
        Self {
            key: Key::Null,
            modifiers: KeyModifiers::NONE,
        }
    }
}

/// Parse a key binding from string format like "ctrl+a", "alt+j", "shift+up", "g", "enter", "f1"
impl FromStr for KeyBinding {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim().to_lowercase();
        let parts: Vec<&str> = s.split('+').collect();

        let mut modifiers = KeyModifiers::NONE;
        let key_str: &str;

        match parts.len() {
            1 => {
                key_str = parts[0];
            }
            2 => {
                match parts[0] {
                    "ctrl" | "control" | "c" => modifiers = KeyModifiers::CONTROL,
                    "alt" | "a" | "meta" | "m" => modifiers = KeyModifiers::ALT,
                    "shift" | "s" => modifiers = KeyModifiers::SHIFT,
                    _ => return Err(format!("Unknown modifier: {}", parts[0])),
                }
                key_str = parts[1];
            }
            3 => {
                // Handle combinations like ctrl+shift+a
                for modifier in &parts[0..2] {
                    match *modifier {
                        "ctrl" | "control" | "c" => modifiers |= KeyModifiers::CONTROL,
                        "alt" | "a" | "meta" | "m" => modifiers |= KeyModifiers::ALT,
                        "shift" | "s" => modifiers |= KeyModifiers::SHIFT,
                        _ => return Err(format!("Unknown modifier: {}", modifier)),
                    }
                }
                key_str = parts[2];
            }
            _ => return Err(format!("Invalid key binding format: {}", s)),
        }

        let key = parse_key(key_str)?;
        Ok(KeyBinding { key, modifiers })
    }
}

fn parse_key(s: &str) -> Result<Key, String> {
    match s {
        // Special keys
        "esc" | "escape" => Ok(Key::Esc),
        "enter" | "return" | "cr" => Ok(Key::Enter),
        "space" | "spc" => Ok(Key::Char(' ')),
        "tab" => Ok(Key::Tab),
        "backtab" | "btab" => Ok(Key::BackTab),
        "backspace" | "bs" => Ok(Key::Backspace),
        "delete" | "del" => Ok(Key::Delete),
        "insert" | "ins" => Ok(Key::Insert),
        "home" => Ok(Key::Home),
        "end" => Ok(Key::End),
        "pageup" | "pgup" => Ok(Key::PageUp),
        "pagedown" | "pgdn" => Ok(Key::PageDown),
        "up" => Ok(Key::Up),
        "down" => Ok(Key::Down),
        "left" => Ok(Key::Left),
        "right" => Ok(Key::Right),

        // Function keys
        "f1" => Ok(Key::Function(1)),
        "f2" => Ok(Key::Function(2)),
        "f3" => Ok(Key::Function(3)),
        "f4" => Ok(Key::Function(4)),
        "f5" => Ok(Key::Function(5)),
        "f6" => Ok(Key::Function(6)),
        "f7" => Ok(Key::Function(7)),
        "f8" => Ok(Key::Function(8)),
        "f9" => Ok(Key::Function(9)),
        "f10" => Ok(Key::Function(10)),
        "f11" => Ok(Key::Function(11)),
        "f12" => Ok(Key::Function(12)),

        // Single character
        s if s.len() == 1 => Ok(Key::Char(s.chars().next().unwrap())),

        _ => Err(format!("Unknown key: {}", s)),
    }
}

fn key_to_string(key: &Key) -> String {
    match key {
        Key::Esc => "esc".to_string(),
        Key::Enter => "enter".to_string(),
        Key::Char(' ') => "space".to_string(),
        Key::Char(c) => c.to_string(),
        Key::Tab => "tab".to_string(),
        Key::BackTab => "backtab".to_string(),
        Key::Backspace => "backspace".to_string(),
        Key::Delete => "delete".to_string(),
        Key::Insert => "insert".to_string(),
        Key::Home => "home".to_string(),
        Key::End => "end".to_string(),
        Key::PageUp => "pageup".to_string(),
        Key::PageDown => "pagedown".to_string(),
        Key::Up => "up".to_string(),
        Key::Down => "down".to_string(),
        Key::Left => "left".to_string(),
        Key::Right => "right".to_string(),
        Key::Function(n) => format!("f{}", n),
        _ => "unknown".to_string(),
    }
}

impl fmt::Display for KeyBinding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut parts = Vec::new();

        if self.modifiers.contains(KeyModifiers::CONTROL) {
            parts.push("ctrl".to_string());
        }
        if self.modifiers.contains(KeyModifiers::ALT) {
            parts.push("alt".to_string());
        }
        if self.modifiers.contains(KeyModifiers::SHIFT) {
            parts.push("shift".to_string());
        }

        parts.push(key_to_string(&self.key));
        write!(f, "{}", parts.join("+"))
    }
}

impl Serialize for KeyBinding {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for KeyBinding {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct KeyBindingVisitor;

        impl Visitor<'_> for KeyBindingVisitor {
            type Value = KeyBinding;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a key binding string like 'ctrl+a', 'j', 'enter', 'f1'")
            }

            fn visit_str<E>(self, value: &str) -> Result<KeyBinding, E>
            where
                E: de::Error,
            {
                KeyBinding::from_str(value).map_err(de::Error::custom)
            }
        }

        deserializer.deserialize_str(KeyBindingVisitor)
    }
}

/// Keybindings for file explorer actions
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub struct ExplorerKeyBindings {
    // Navigation
    pub move_up: KeyBinding,
    pub move_down: KeyBinding,
    pub move_up_page: KeyBinding,
    pub move_down_page: KeyBinding,
    pub move_to_top: KeyBinding,
    pub move_to_bottom: KeyBinding,
    pub go_to_parent: KeyBinding,
    pub go_back: KeyBinding,
    pub enter_dir: KeyBinding,
    #[serde(default = "default_enter_dir_alt")]
    pub enter_dir_alt: KeyBinding,
    pub change_panel: KeyBinding,

    // File operations
    pub transfer_file: KeyBinding,
    pub copy_file: KeyBinding,
    pub rename_file: KeyBinding,
    pub delete_file: KeyBinding,
    pub mkdir: KeyBinding,
    pub new_file: KeyBinding,
    pub edit_file: KeyBinding,
    pub open_file: KeyBinding,
    pub open_with: KeyBinding,
    pub save_as: KeyBinding,
    pub chmod: KeyBinding,
    pub symlink: KeyBinding,
    pub reload_dir: KeyBinding,

    // Selection
    pub mark_file: KeyBinding,
    pub mark_all: KeyBinding,
    pub unmark_all: KeyBinding,

    // View
    pub toggle_hidden: KeyBinding,
    pub file_info: KeyBinding,
    pub file_size: KeyBinding,
    pub sorting: KeyBinding,
    pub filter: KeyBinding,

    // Search
    pub fuzzy_search: KeyBinding,
    pub goto_path: KeyBinding,

    // Misc
    pub terminal: KeyBinding,
    pub sync_browsing: KeyBinding,
    pub watcher: KeyBinding,
    pub watched_paths: KeyBinding,
    pub pending_queue: KeyBinding,
}

/// Default value for enter_dir_alt (used when field is missing in config)
fn default_enter_dir_alt() -> KeyBinding {
    // Default to 'l' as vim-style alternative, but Enter is the primary
    KeyBinding::simple(Key::Char('l'))
}

impl Default for ExplorerKeyBindings {
    fn default() -> Self {
        Self {
            // Navigation
            move_up: KeyBinding::simple(Key::Up),
            move_down: KeyBinding::simple(Key::Down),
            move_up_page: KeyBinding::simple(Key::PageUp),
            move_down_page: KeyBinding::simple(Key::PageDown),
            move_to_top: KeyBinding::simple(Key::Home),
            move_to_bottom: KeyBinding::simple(Key::End),
            go_to_parent: KeyBinding::simple(Key::Char('u')),
            go_back: KeyBinding::simple(Key::Backspace),
            enter_dir: KeyBinding::simple(Key::Enter),
            enter_dir_alt: default_enter_dir_alt(),
            change_panel: KeyBinding::simple(Key::Tab),

            // File operations
            transfer_file: KeyBinding::simple(Key::Char(' ')),
            copy_file: KeyBinding::simple(Key::Char('c')),
            rename_file: KeyBinding::simple(Key::Char('r')),
            delete_file: KeyBinding::simple(Key::Char('e')),
            mkdir: KeyBinding::simple(Key::Char('d')),
            new_file: KeyBinding::simple(Key::Char('n')),
            edit_file: KeyBinding::simple(Key::Char('o')),
            open_file: KeyBinding::simple(Key::Char('v')),
            open_with: KeyBinding::simple(Key::Char('w')),
            save_as: KeyBinding::simple(Key::Char('s')),
            chmod: KeyBinding::simple(Key::Char('z')),
            symlink: KeyBinding::simple(Key::Char('k')),
            reload_dir: KeyBinding::simple(Key::Char('l')),

            // Selection
            mark_file: KeyBinding::simple(Key::Char('m')),
            mark_all: KeyBinding::ctrl(Key::Char('a')),
            unmark_all: KeyBinding::alt(Key::Char('a')),

            // View
            toggle_hidden: KeyBinding::simple(Key::Char('a')),
            file_info: KeyBinding::simple(Key::Char('i')),
            file_size: KeyBinding::ctrl(Key::Char('s')),
            sorting: KeyBinding::simple(Key::Char('b')),
            filter: KeyBinding::simple(Key::Char('/')),

            // Search
            fuzzy_search: KeyBinding::simple(Key::Char('f')),
            goto_path: KeyBinding::simple(Key::Char('g')),

            // Misc
            terminal: KeyBinding::simple(Key::Char('x')),
            sync_browsing: KeyBinding::simple(Key::Char('y')),
            watcher: KeyBinding::simple(Key::Char('t')),
            watched_paths: KeyBinding::ctrl(Key::Char('t')),
            pending_queue: KeyBinding::simple(Key::Char('p')),
        }
    }
}

/// Global keybindings that work across all activities
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub struct GlobalKeyBindings {
    pub quit: KeyBinding,
    pub quit_alt: KeyBinding,
    pub disconnect: KeyBinding,
    pub help: KeyBinding,
    pub help_alt: KeyBinding,
}

impl Default for GlobalKeyBindings {
    fn default() -> Self {
        Self {
            quit: KeyBinding::simple(Key::Char('q')),
            quit_alt: KeyBinding::simple(Key::Function(10)),
            disconnect: KeyBinding::simple(Key::Esc),
            help: KeyBinding::simple(Key::Char('h')),
            help_alt: KeyBinding::simple(Key::Function(1)),
        }
    }
}

/// Keybindings for the authentication activity
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub struct AuthKeyBindings {
    pub quit: KeyBinding,
    pub quit_alt: KeyBinding,
    pub setup: KeyBinding,
    pub help: KeyBinding,
    pub help_alt: KeyBinding,
    pub release_notes: KeyBinding,
    pub save_bookmark: KeyBinding,
}

impl Default for AuthKeyBindings {
    fn default() -> Self {
        Self {
            quit: KeyBinding::simple(Key::Esc),
            quit_alt: KeyBinding::simple(Key::Function(10)),
            setup: KeyBinding::ctrl(Key::Char('c')),
            help: KeyBinding::ctrl(Key::Char('h')),
            help_alt: KeyBinding::simple(Key::Function(1)),
            release_notes: KeyBinding::ctrl(Key::Char('r')),
            save_bookmark: KeyBinding::ctrl(Key::Char('s')),
        }
    }
}

/// Keybindings for the setup activity
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub struct SetupKeyBindings {
    pub quit: KeyBinding,
    pub quit_alt: KeyBinding,
    pub change_tab: KeyBinding,
    pub help: KeyBinding,
    pub help_alt: KeyBinding,
    pub revert: KeyBinding,
    pub save: KeyBinding,
    pub save_alt: KeyBinding,
}

impl Default for SetupKeyBindings {
    fn default() -> Self {
        Self {
            quit: KeyBinding::simple(Key::Esc),
            quit_alt: KeyBinding::simple(Key::Function(10)),
            change_tab: KeyBinding::simple(Key::Tab),
            help: KeyBinding::ctrl(Key::Char('h')),
            help_alt: KeyBinding::simple(Key::Function(1)),
            revert: KeyBinding::ctrl(Key::Char('r')),
            save: KeyBinding::ctrl(Key::Char('s')),
            save_alt: KeyBinding::simple(Key::Function(4)),
        }
    }
}

/// Complete keybindings configuration
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub struct KeyBindings {
    pub global: GlobalKeyBindings,
    pub explorer: ExplorerKeyBindings,
    pub auth: AuthKeyBindings,
    pub setup: SetupKeyBindings,
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            global: GlobalKeyBindings::default(),
            explorer: ExplorerKeyBindings::default(),
            auth: AuthKeyBindings::default(),
            setup: SetupKeyBindings::default(),
        }
    }
}

impl KeyBindings {
    /// Create vim-style keybindings inspired by yazi, ranger, and other file managers
    #[allow(dead_code)]
    pub fn vim_style() -> Self {
        Self {
            global: GlobalKeyBindings {
                quit: KeyBinding::simple(Key::Char('q')),
                quit_alt: KeyBinding::simple(Key::Char('Q')),
                disconnect: KeyBinding::simple(Key::Esc),
                help: KeyBinding::simple(Key::Char('?')),
                help_alt: KeyBinding::simple(Key::Function(1)),
            },
            explorer: ExplorerKeyBindings {
                // Vim-style navigation (j/k like yazi/ranger)
                move_up: KeyBinding::simple(Key::Char('k')),
                move_down: KeyBinding::simple(Key::Char('j')),
                move_up_page: KeyBinding::ctrl(Key::Char('u')),
                move_down_page: KeyBinding::ctrl(Key::Char('d')),
                move_to_top: KeyBinding::simple(Key::Char('g')),
                move_to_bottom: KeyBinding::simple(Key::Char('G')),
                go_to_parent: KeyBinding::simple(Key::Char('h')),
                go_back: KeyBinding::simple(Key::Char('-')),
                enter_dir: KeyBinding::simple(Key::Char('l')),
                enter_dir_alt: KeyBinding::simple(Key::Enter),
                change_panel: KeyBinding::simple(Key::Tab),

                // File operations (yazi/ranger style)
                transfer_file: KeyBinding::simple(Key::Char('p')),
                copy_file: KeyBinding::simple(Key::Char('c')),
                rename_file: KeyBinding::simple(Key::Char('r')),
                delete_file: KeyBinding::simple(Key::Char('d')),
                mkdir: KeyBinding::simple(Key::Char('a')),
                new_file: KeyBinding::simple(Key::Char('A')),
                edit_file: KeyBinding::simple(Key::Char('e')),
                open_file: KeyBinding::simple(Key::Char('o')),
                open_with: KeyBinding::simple(Key::Char('O')),
                save_as: KeyBinding::simple(Key::Char('S')),
                chmod: KeyBinding::simple(Key::Char('z')),
                symlink: KeyBinding::simple(Key::Char('K')),
                reload_dir: KeyBinding::ctrl(Key::Char('r')),

                // Selection (ranger style: space to mark)
                mark_file: KeyBinding::simple(Key::Char(' ')),
                mark_all: KeyBinding::simple(Key::Char('V')),
                unmark_all: KeyBinding::simple(Key::Char('u')),

                // View
                toggle_hidden: KeyBinding::simple(Key::Char('.')),
                file_info: KeyBinding::simple(Key::Char('i')),
                file_size: KeyBinding::simple(Key::Char('I')),
                sorting: KeyBinding::simple(Key::Char('s')),
                filter: KeyBinding::simple(Key::Char('F')),

                // Search (vim style: / to search)
                fuzzy_search: KeyBinding::simple(Key::Char('/')),
                goto_path: KeyBinding::simple(Key::Char(':')),

                // Misc
                terminal: KeyBinding::simple(Key::Char('!')),
                sync_browsing: KeyBinding::simple(Key::Char('y')),
                watcher: KeyBinding::simple(Key::Char('w')),
                watched_paths: KeyBinding::simple(Key::Char('W')),
                pending_queue: KeyBinding::simple(Key::Char('p')),
            },
            auth: AuthKeyBindings {
                quit: KeyBinding::simple(Key::Esc),
                quit_alt: KeyBinding::ctrl(Key::Char('q')),
                setup: KeyBinding::ctrl(Key::Char('c')),
                help: KeyBinding::simple(Key::Char('?')),
                help_alt: KeyBinding::simple(Key::Function(1)),
                release_notes: KeyBinding::ctrl(Key::Char('n')),
                save_bookmark: KeyBinding::ctrl(Key::Char('s')),
            },
            setup: SetupKeyBindings {
                quit: KeyBinding::simple(Key::Esc),
                quit_alt: KeyBinding::ctrl(Key::Char('q')),
                change_tab: KeyBinding::simple(Key::Tab),
                help: KeyBinding::simple(Key::Char('?')),
                help_alt: KeyBinding::simple(Key::Function(1)),
                revert: KeyBinding::ctrl(Key::Char('r')),
                save: KeyBinding::ctrl(Key::Char('s')),
                save_alt: KeyBinding::ctrl(Key::Char('w')),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_key_binding_from_str() {
        assert_eq!(
            KeyBinding::from_str("j").unwrap(),
            KeyBinding::simple(Key::Char('j'))
        );
        assert_eq!(
            KeyBinding::from_str("ctrl+a").unwrap(),
            KeyBinding::ctrl(Key::Char('a'))
        );
        assert_eq!(
            KeyBinding::from_str("alt+j").unwrap(),
            KeyBinding::alt(Key::Char('j'))
        );
        assert_eq!(
            KeyBinding::from_str("shift+up").unwrap(),
            KeyBinding::shift(Key::Up)
        );
        assert_eq!(
            KeyBinding::from_str("enter").unwrap(),
            KeyBinding::simple(Key::Enter)
        );
        assert_eq!(
            KeyBinding::from_str("f1").unwrap(),
            KeyBinding::simple(Key::Function(1))
        );
        assert_eq!(
            KeyBinding::from_str("space").unwrap(),
            KeyBinding::simple(Key::Char(' '))
        );
    }

    #[test]
    fn test_key_binding_to_string() {
        assert_eq!(KeyBinding::simple(Key::Char('j')).to_string(), "j");
        assert_eq!(KeyBinding::ctrl(Key::Char('a')).to_string(), "ctrl+a");
        assert_eq!(KeyBinding::alt(Key::Char('j')).to_string(), "alt+j");
        assert_eq!(KeyBinding::shift(Key::Up).to_string(), "shift+up");
        assert_eq!(KeyBinding::simple(Key::Enter).to_string(), "enter");
        assert_eq!(KeyBinding::simple(Key::Function(1)).to_string(), "f1");
    }

    #[test]
    fn test_key_binding_matches() {
        let kb = KeyBinding::ctrl(Key::Char('a'));
        assert!(kb.matches(Key::Char('a'), KeyModifiers::CONTROL));
        assert!(!kb.matches(Key::Char('a'), KeyModifiers::NONE));
        assert!(!kb.matches(Key::Char('b'), KeyModifiers::CONTROL));
    }

    #[test]
    fn test_default_keybindings() {
        let kb = KeyBindings::default();
        assert_eq!(kb.explorer.move_up, KeyBinding::simple(Key::Up));
        assert_eq!(kb.explorer.move_down, KeyBinding::simple(Key::Down));
    }

    #[test]
    fn test_vim_style_keybindings() {
        let kb = KeyBindings::vim_style();
        assert_eq!(kb.explorer.move_up, KeyBinding::simple(Key::Char('k')));
        assert_eq!(kb.explorer.move_down, KeyBinding::simple(Key::Char('j')));
        assert_eq!(kb.explorer.enter_dir, KeyBinding::simple(Key::Char('l')));
        assert_eq!(kb.explorer.go_to_parent, KeyBinding::simple(Key::Char('h')));
    }
}

