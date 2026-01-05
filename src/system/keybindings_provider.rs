//! ## KeyBindingsProvider
//!
//! `keybindings_provider` is the module which provides an API between the keybindings configuration and the system

use std::fs::OpenOptions;
use std::path::{Path, PathBuf};

use crate::config::keybindings::KeyBindings;
use crate::config::serialization::{SerializerError, SerializerErrorKind, deserialize, serialize};

/// KeyBindingsProvider provides a high level API to communicate with the termscp keybindings
pub struct KeyBindingsProvider {
    keybindings: KeyBindings,
    keybindings_path: PathBuf,
    degraded: bool,
}

impl KeyBindingsProvider {
    /// Instantiates a new `KeyBindingsProvider`
    pub fn new(keybindings_path: &Path) -> Result<Self, SerializerError> {
        let default_keybindings: KeyBindings = KeyBindings::default();
        info!(
            "Setting up keybindings provider with keybindings path {}",
            keybindings_path.display(),
        );
        // Create provider
        let mut provider: KeyBindingsProvider = KeyBindingsProvider {
            keybindings: default_keybindings,
            keybindings_path: keybindings_path.to_path_buf(),
            degraded: false,
        };
        // If Config file doesn't exist, create it
        if !keybindings_path.exists() {
            if let Err(err) = provider.save() {
                error!("Couldn't write keybindings file: {}", err);
                return Err(err);
            }
            debug!("Keybindings file didn't exist; created file");
        } else {
            // otherwise Load configuration from file
            if let Err(err) = provider.load() {
                error!("Couldn't read keybindings file: {}", err);
                return Err(err);
            }
            debug!("Read keybindings file");
        }
        Ok(provider)
    }

    /// Create a new keybindings provider which won't work with file system.
    /// This is done in order to prevent a lot of `unwrap_or` on Ui
    pub fn degraded() -> Self {
        Self {
            keybindings: KeyBindings::default(),
            keybindings_path: PathBuf::default(),
            degraded: true,
        }
    }

    // -- getters

    /// Returns keybindings as reference
    pub fn keybindings(&self) -> &KeyBindings {
        &self.keybindings
    }

    /// Returns a mutable reference to the keybindings
    #[allow(dead_code)]
    pub fn keybindings_mut(&mut self) -> &mut KeyBindings {
        &mut self.keybindings
    }

    // -- io

    /// Load keybindings from file
    pub fn load(&mut self) -> Result<(), SerializerError> {
        if self.degraded {
            warn!("Configuration won't be loaded, since degraded; reloading default...");
            self.keybindings = KeyBindings::default();
            return Err(SerializerError::new_ex(
                SerializerErrorKind::Generic,
                String::from("Can't access keybindings file"),
            ));
        }
        // Open keybindings file for read
        debug!("Loading keybindings from file...");
        match OpenOptions::new()
            .read(true)
            .open(self.keybindings_path.as_path())
        {
            Ok(reader) => {
                // Deserialize
                match deserialize(Box::new(reader)) {
                    Ok(keybindings) => {
                        self.keybindings = keybindings;
                        Ok(())
                    }
                    Err(err) => Err(err),
                }
            }
            Err(err) => {
                error!("Failed to read keybindings: {}", err);
                Err(SerializerError::new_ex(
                    SerializerErrorKind::Io,
                    err.to_string(),
                ))
            }
        }
    }

    /// Save keybindings to file
    pub fn save(&self) -> Result<(), SerializerError> {
        if self.degraded {
            warn!("Configuration won't be saved, since in degraded mode");
            return Err(SerializerError::new_ex(
                SerializerErrorKind::Generic,
                String::from("Can't access keybindings file"),
            ));
        }
        // Open file
        debug!("Writing keybindings");
        match OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(self.keybindings_path.as_path())
        {
            Ok(writer) => serialize(self.keybindings(), Box::new(writer)),
            Err(err) => {
                error!("Failed to write keybindings: {}", err);
                Err(SerializerError::new_ex(
                    SerializerErrorKind::Io,
                    err.to_string(),
                ))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use tempfile::TempDir;
    use tuirealm::event::Key;

    use super::*;
    use crate::config::keybindings::KeyBinding;

    #[test]
    fn test_system_keybindings_provider_new() {
        let tmp_dir: tempfile::TempDir = TempDir::new().ok().unwrap();
        let keybindings_path: PathBuf = get_keybindings_path(tmp_dir.path());
        // Initialize a new keybindings provider
        let mut provider: KeyBindingsProvider =
            KeyBindingsProvider::new(keybindings_path.as_path()).unwrap();
        // Verify client
        assert_eq!(
            provider.keybindings().explorer.move_up,
            KeyBinding::simple(Key::Up)
        );
        assert_eq!(provider.keybindings_path, keybindings_path);
        assert_eq!(provider.degraded, false);
        // Mutation
        provider.keybindings_mut().explorer.move_up = KeyBinding::simple(Key::Char('k'));
        assert_eq!(
            provider.keybindings().explorer.move_up,
            KeyBinding::simple(Key::Char('k'))
        );
    }

    #[test]
    fn test_system_keybindings_provider_load_and_save() {
        let tmp_dir: tempfile::TempDir = TempDir::new().ok().unwrap();
        let keybindings_path: PathBuf = get_keybindings_path(tmp_dir.path());
        // Initialize a new keybindings provider
        let mut provider: KeyBindingsProvider =
            KeyBindingsProvider::new(keybindings_path.as_path()).unwrap();
        // Write
        provider.keybindings_mut().explorer.move_up = KeyBinding::simple(Key::Char('k'));
        assert!(provider.save().is_ok());
        provider.keybindings_mut().explorer.move_up = KeyBinding::simple(Key::Char('j'));
        // Reload
        assert!(provider.load().is_ok());
        // Unchanged
        assert_eq!(
            provider.keybindings().explorer.move_up,
            KeyBinding::simple(Key::Char('k'))
        );
        // Instantiate a new provider
        let provider: KeyBindingsProvider =
            KeyBindingsProvider::new(keybindings_path.as_path()).unwrap();
        assert_eq!(
            provider.keybindings().explorer.move_up,
            KeyBinding::simple(Key::Char('k'))
        ); // Unchanged
    }

    #[test]
    fn test_system_keybindings_provider_degraded() {
        let mut provider: KeyBindingsProvider = KeyBindingsProvider::degraded();
        assert_eq!(
            provider.keybindings().explorer.move_up,
            KeyBinding::simple(Key::Up)
        );
        assert_eq!(provider.degraded, true);
        provider.keybindings_mut().explorer.move_up = KeyBinding::simple(Key::Char('k'));
        assert!(provider.load().is_err());
        assert_eq!(
            provider.keybindings().explorer.move_up,
            KeyBinding::simple(Key::Up)
        );
        assert!(provider.save().is_err());
    }

    #[test]
    fn test_system_keybindings_provider_err() {
        assert!(KeyBindingsProvider::new(Path::new("/tmp/oifoif/omar")).is_err());
    }

    /// Get paths for keybindings file
    fn get_keybindings_path(dir: &Path) -> PathBuf {
        let mut p: PathBuf = PathBuf::from(dir);
        p.push("keybindings.toml");
        p
    }
}

