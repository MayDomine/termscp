use std::path::{Path, PathBuf};

use ssh2_config::{ParseRule, SshConfig};

use super::Args;
use crate::filetransfer::params::GenericProtocolParams;
use crate::filetransfer::{FileTransferParams, FileTransferProtocol, ProtocolParams};
use crate::utils;

/// Address type
enum AddrType {
    Address,
    Bookmark,
    SshHost,
}

/// Args for remote connection
#[derive(Debug)]
pub struct RemoteArgs {
    pub host_bridge: Remote,
    pub remote: Remote,
    pub local_dir: Option<PathBuf>,
}

impl Default for RemoteArgs {
    fn default() -> Self {
        Self {
            host_bridge: Remote::None,
            remote: Remote::None,
            local_dir: None,
        }
    }
}

impl TryFrom<&Args> for RemoteArgs {
    type Error = String;

    fn try_from(args: &Args) -> Result<Self, Self::Error> {
        let mut remote_args = RemoteArgs::default();
        // validate arguments
        let total_hosts = args.bookmark.len() + args.ssh_host.len() + args.positional.len();
        if total_hosts > 3 {
            return Err("Too many arguments".to_string());
        }

        // parse bookmark first
        let last_item_index = total_hosts.checked_sub(1).unwrap_or_default();

        let mut hosts = vec![];

        for (i, (addr_type, arg)) in args
            .bookmark
            .iter()
            .map(|x| (AddrType::Bookmark, x))
            .chain(args.ssh_host.iter().map(|x| (AddrType::SshHost, x)))
            .chain(args.positional.iter().map(|x| (AddrType::Address, x)))
            .enumerate()
        {
            // check if has password
            let password = args.password.get(i).cloned();

            // check if is last item and so a possible local dir
            if i == last_item_index && Path::new(arg).exists() {
                remote_args.local_dir = Some(PathBuf::from(arg));
                continue;
            }

            let remote = match addr_type {
                AddrType::Address => Self::parse_remote_address(arg)
                    .map(|x| Remote::Host(HostParams::new(x, password)))?,
                AddrType::Bookmark => Remote::Bookmark(BookmarkParams::new(arg, password.as_ref())),
                AddrType::SshHost => Self::parse_ssh_host(arg)
                    .map(|x| Remote::Host(HostParams::new(x, password)))?,
            };

            // set remote
            hosts.push(remote);
        }

        // set args based on hosts len
        if hosts.len() == 1 {
            remote_args.remote = hosts.pop().unwrap();
        } else if hosts.len() == 2 {
            remote_args.host_bridge = hosts.pop().unwrap();
            remote_args.remote = hosts.pop().unwrap();
        }

        Ok(remote_args)
    }
}

impl RemoteArgs {
    /// Parse remote address
    fn parse_remote_address(remote: &str) -> Result<FileTransferParams, String> {
        utils::parser::parse_remote_opt(remote).map_err(|e| format!("Bad address option: {e}"))
    }

    /// Parse SSH config host alias (e.g., "hostA:/path/to/dir" or just "hostA")
    fn parse_ssh_host(host_arg: &str) -> Result<FileTransferParams, String> {
        // Parse host:path format
        let (host_alias, remote_path) = if let Some(colon_pos) = host_arg.find(':') {
            let alias = &host_arg[..colon_pos];
            let path = &host_arg[colon_pos + 1..];
            (alias, if path.is_empty() { None } else { Some(path) })
        } else {
            (host_arg, None)
        };

        // Load SSH config from default location
        let ssh_config = SshConfig::parse_default_file(ParseRule::ALLOW_UNKNOWN_FIELDS)
            .map_err(|e| format!("Could not parse SSH config: {e}"))?;

        // Find the host
        let host = ssh_config
            .get_hosts()
            .iter()
            .find(|h| {
                h.pattern
                    .iter()
                    .any(|p| !p.negated && p.pattern == host_alias)
            })
            .ok_or_else(|| format!("SSH host '{}' not found in ~/.ssh/config", host_alias))?;

        // Extract connection parameters
        let address = host
            .params
            .host_name
            .as_deref()
            .unwrap_or(host_alias)
            .to_string();
        let port = host.params.port.unwrap_or(22);
        let username = host.params.user.clone();

        let mut params = FileTransferParams::new(
            FileTransferProtocol::Sftp,
            ProtocolParams::Generic(
                GenericProtocolParams::default()
                    .address(address)
                    .port(port)
                    .username(username),
            ),
        );

        // Set remote path if specified
        if let Some(path) = remote_path {
            params.remote_path = Some(PathBuf::from(path));
        }

        Ok(params)
    }
}

/// Remote argument type
#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum Remote {
    /// Bookmark name argument
    Bookmark(BookmarkParams),
    /// Host argument
    Host(HostParams),
    /// Unspecified
    None,
}

impl Remote {
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }
}

/// Bookmark parameters
#[derive(Debug)]
pub struct BookmarkParams {
    /// bookmark name
    pub name: String,
    /// bookmark password
    pub password: Option<String>,
}

/// Host parameters
#[derive(Debug)]
pub struct HostParams {
    /// file transfer parameters
    pub file_transfer_params: FileTransferParams,
    /// host password specified in arguments
    pub password: Option<String>,
}

impl BookmarkParams {
    pub fn new<S: AsRef<str>>(name: S, password: Option<S>) -> Self {
        Self {
            name: name.as_ref().to_string(),
            password: password.map(|x| x.as_ref().to_string()),
        }
    }
}

impl HostParams {
    pub fn new<S: AsRef<str>>(params: FileTransferParams, password: Option<S>) -> Self {
        Self {
            file_transfer_params: params,
            password: password.map(|x| x.as_ref().to_string()),
        }
    }
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_should_make_remote_args_from_args_one_remote() {
        let args = Args {
            positional: vec!["scp://host1".to_string()],
            ..Default::default()
        };

        let remote_args = RemoteArgs::try_from(&args).unwrap();
        assert!(matches!(remote_args.host_bridge, Remote::None));
        assert!(matches!(remote_args.remote, Remote::Host(_)));
        assert_eq!(remote_args.local_dir, None);
    }

    #[test]
    fn test_should_make_remote_args_from_args_two_remotes() {
        let args = Args {
            positional: vec!["scp://host1".to_string(), "scp://host2".to_string()],
            ..Default::default()
        };

        let remote_args = RemoteArgs::try_from(&args).unwrap();
        assert!(matches!(remote_args.host_bridge, Remote::Host(_)));
        assert!(matches!(remote_args.remote, Remote::Host(_)));
        assert_eq!(remote_args.local_dir, None);
    }

    #[test]
    #[cfg(posix)]
    fn test_should_make_remote_args_from_two_remotes_and_local_dir() {
        let args = Args {
            positional: vec![
                "scp://host1".to_string(),
                "scp://host2".to_string(),
                "/home".to_string(),
            ],
            ..Default::default()
        };

        let remote_args = RemoteArgs::try_from(&args).unwrap();
        assert!(matches!(remote_args.host_bridge, Remote::Host(_)));
        assert!(matches!(remote_args.remote, Remote::Host(_)));
        assert_eq!(remote_args.local_dir, Some(PathBuf::from("/home")));
    }

    #[test]
    fn test_should_make_remote_args_from_args_one_bookmarks() {
        let args = Args {
            bookmark: vec!["foo".to_string()],
            ..Default::default()
        };

        let remote_args = RemoteArgs::try_from(&args).unwrap();
        assert!(matches!(remote_args.host_bridge, Remote::None));
        assert!(matches!(remote_args.remote, Remote::Bookmark(_)));
        assert_eq!(remote_args.local_dir, None);
    }

    #[test]
    fn test_should_make_remote_args_from_args_two_bookmarks() {
        let args = Args {
            bookmark: vec!["foo".to_string(), "bar".to_string()],
            ..Default::default()
        };

        let remote_args = RemoteArgs::try_from(&args).unwrap();
        assert!(matches!(remote_args.host_bridge, Remote::Bookmark(_)));
        assert!(matches!(remote_args.remote, Remote::Bookmark(_)));
        assert_eq!(remote_args.local_dir, None);
    }

    #[test]
    #[cfg(posix)]
    fn test_should_make_remote_args_from_two_bookmarks_and_local_dir() {
        let args = Args {
            bookmark: vec!["foo".to_string(), "bar".to_string()],
            positional: vec!["/home".to_string()],
            ..Default::default()
        };

        let remote_args = RemoteArgs::try_from(&args).unwrap();
        assert!(matches!(remote_args.host_bridge, Remote::Bookmark(_)));
        assert!(matches!(remote_args.remote, Remote::Bookmark(_)));
        assert_eq!(remote_args.local_dir, Some(PathBuf::from("/home")));
    }

    #[test]
    fn test_should_make_remote_args_from_one_bookmark_and_one_remote() {
        let args = Args {
            bookmark: vec!["foo".to_string()],
            positional: vec!["scp://host1".to_string()],
            ..Default::default()
        };

        let remote_args = RemoteArgs::try_from(&args).unwrap();

        assert!(matches!(remote_args.host_bridge, Remote::Host(_)));
        assert!(matches!(remote_args.remote, Remote::Bookmark(_)));
        assert_eq!(remote_args.local_dir, None);
    }

    #[test]
    #[cfg(posix)]
    fn test_should_make_remote_args_from_one_bookmark_and_one_remote_with_local_dir() {
        let args = Args {
            positional: vec!["scp://host1".to_string(), "/home".to_string()],
            bookmark: vec!["foo".to_string()],
            ..Default::default()
        };

        let remote_args = RemoteArgs::try_from(&args).unwrap();

        assert!(matches!(remote_args.host_bridge, Remote::Host(_)));
        assert!(matches!(remote_args.remote, Remote::Bookmark(_)));
        assert_eq!(remote_args.local_dir, Some(PathBuf::from("/home")));
    }
}
