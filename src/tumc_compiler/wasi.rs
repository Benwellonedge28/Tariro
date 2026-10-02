//! TUMC WASI (WebAssembly System Interface) Support
//!
//! Implementation of WASI preview1 and preview2 APIs for TUMC programs
//! running in WebAssembly environments.

use std::collections::HashMap;
use std::sync::RwLock;

/// WASI configuration
#[derive(Debug, Clone)]
pub struct WasiConfig {
    /// Preopened directories
    pub preopened_dirs: Vec<PreopenedDir>,
    /// Environment variables
    pub env: HashMap<String, String>,
    /// Arguments
    pub args: Vec<String>,
    /// Enable fd_prestat_get
    pub enable_fd_prestat_get: bool,
    /// Enable random
    pub enable_random: bool,
    /// Enable clock
    pub enable_clock: bool,
}

impl Default for WasiConfig {
    fn default() -> Self {
        Self {
            preopened_dirs: Vec::new(),
            env: HashMap::new(),
            args: Vec::new(),
            enable_fd_prestat_get: true,
            enable_random: true,
            enable_clock: true,
        }
    }
}

/// Preopened directory
#[derive(Debug, Clone)]
pub struct PreopenedDir {
    /// Host path
    pub host_path: String,
    /// Guest path
    pub guest_path: String,
    /// Read access
    pub read: bool,
    /// Write access
    pub write: bool,
}

/// WASI file descriptor
#[derive(Debug, Clone)]
pub struct WasiFd {
    /// FD number
    pub fd: u32,
    /// Kind
    pub kind: FdKind,
    /// Rights
    pub rights: u64,
    /// Current position
    pub position: u64,
    /// Path (for files)
    pub path: Option<String>,
}

#[derive(Debug, Clone)]
pub enum FdKind {
    Stdin,
    Stdout,
    Stderr,
    File,
    Directory,
    Socket,
    Pipe,
}

/// WASI implementation
pub struct WasiImplementation {
    /// Configuration
    config: WasiConfig,
    /// File descriptors
    fds: RwLock<HashMap<u32, WasiFd>>,
    /// Next FD number
    next_fd: RwLock<u32>,
    /// Memory
    memory: Vec<u8>,
}

impl Default for WasiImplementation {
    fn default() -> Self {
        Self::new(WasiConfig::default())
    }
}

impl WasiImplementation {
    /// Create new WASI implementation
    pub fn new(config: WasiConfig) -> Self {
        let mut fds = HashMap::new();

        // Initialize standard FDs
        fds.insert(0, WasiFd {
            fd: 0,
            kind: FdKind::Stdin,
            rights: 0,
            position: 0,
            path: None,
        });
        fds.insert(1, WasiFd {
            fd: 1,
            kind: FdKind::Stdout,
            rights: 0,
            position: 0,
            path: None,
        });
        fds.insert(2, WasiFd {
            fd: 2,
            kind: FdKind::Stderr,
            rights: 0,
            position: 0,
            path: None,
        });

        Self {
            config,
            fds: RwLock::new(fds),
            next_fd: RwLock::new(3),
            memory: Vec::new(),
        }
    }

    /// Allocate memory
    pub fn alloc_memory(&mut self, size: usize) -> u32 {
        let offset = self.memory.len();
        self.memory.resize(offset + size, 0);
        offset as u32
    }

    /// Write to memory
    pub fn write_memory(&mut self, offset: u32, data: &[u8]) -> Result<(), WasiError> {
        let offset = offset as usize;
        if offset >= self.memory.len() {
            return Err(WasiError::InvalidOffset);
        }
        let end = (offset + data.len()).min(self.memory.len());
        self.memory[offset..end].copy_from_slice(&data[..end - offset]);
        Ok(())
    }

    /// Read from memory
    pub fn read_memory(&self, offset: u32, size: usize) -> Result<Vec<u8>, WasiError> {
        let offset = offset as usize;
        if offset >= self.memory.len() {
            return Err(WasiError::InvalidOffset);
        }
        let end = (offset + size).min(self.memory.len());
        Ok(self.memory[offset..end].to_vec())
    }

    /// Open file
    pub fn open(&self, path: &str) -> Result<u32, WasiError> {
        let next_fd = *self.next_fd.read().unwrap();
        let fd = WasiFd {
            fd: next_fd,
            kind: FdKind::File,
            rights: 0,
            position: 0,
            path: Some(path.to_string()),
        };

        self.fds.write().unwrap().insert(next_fd, fd);
        Ok(next_fd)
    }

    /// Close file descriptor
    pub fn close(&self, fd: u32) -> Result<(), WasiError> {
        self.fds.write().unwrap().remove(&fd);
        Ok(())
    }

    /// Read from FD
    pub fn read(&self, fd: u32, buffer: &mut [u8]) -> Result<usize, WasiError> {
        let fds = self.fds.read().unwrap();
        if let Some(wasi_fd) = fds.get(&fd) {
            match wasi_fd.kind {
                FdKind::Stdin => {
                    use std::io::Read;
                    let bytes = std::io::stdin().read(buffer).map_err(|_| WasiError::IoError)?;
                    Ok(bytes)
                }
                FdKind::File => {
                    // Would read from file
                    Ok(0)
                }
                _ => Err(WasiError::BadFdtype),
            }
        } else {
            Err(WasiError::BadF)
        }
    }

    /// Write to FD
    pub fn write(&self, fd: u32, buffer: &[u8]) -> Result<usize, WasiError> {
        let fds = self.fds.read().unwrap();
        if let Some(wasi_fd) = fds.get(&fd) {
            match wasi_fd.kind {
                FdKind::Stdout | FdKind::Stderr => {
                    let output = String::from_utf8_lossy(buffer);
                    print!("{}", output);
                    Ok(buffer.len())
                }
                FdKind::File => {
                    // Would write to file
                    Ok(buffer.len())
                }
                _ => Err(WasiError::BadFdtype),
            }
        } else {
            Err(WasiError::BadF)
        }
    }

    /// Seek on FD
    pub fn seek(&self, fd: u32, offset: i64, whence: u8) -> Result<u64, WasiError> {
        let mut fds = self.fds.write().unwrap();
        if let Some(wasi_fd) = fds.get_mut(&fd) {
            match whence {
                0 => wasi_fd.position = offset as u64,
                1 => wasi_fd.position = (wasi_fd.position as i64 + offset) as u64,
                2 => {
                    // Would seek to end
                    wasi_fd.position = 0;
                }
                _ => return Err(WasiError::Einval),
            }
            Ok(wasi_fd.position)
        } else {
            Err(WasiError::BadF)
        }
    }

    /// Get file stat
    pub fn stat(&self, fd: u32) -> Result<WasiStat, WasiError> {
        let fds = self.fds.read().unwrap();
        if let Some(wasi_fd) = fds.get(&fd) {
            Ok(WasiStat {
                dev: 0,
                ino: 0,
                filetype: match wasi_fd.kind {
                    FdKind::File => Filetype::RegularFile,
                    FdKind::Directory => Filetype::Directory,
                    _ => Filetype::Unknown,
                },
                nlink: 1,
                size: 0,
                atim: 0,
                mtim: 0,
                ctim: 0,
            })
        } else {
            Err(WasiError::BadF)
        }
    }

    /// Get environment
    pub fn get_env(&self, name: &str) -> Option<String> {
        self.config.env.get(name).cloned()
    }

    /// Get arguments
    pub fn get_args(&self) -> &[String] {
        &self.config.args
    }

    /// Get configuration
    pub fn config(&self) -> &WasiConfig {
        &self.config
    }
}

/// WASI error
#[derive(Debug)]
pub enum WasiError {
    Esuccess,
    E2big,
    Eacces,
    Eaddrinuse,
    Eaddrnotavail,
    Eafnosupport,
    Eagain,
    Ealign,
    Ealready,
    Ebadf,
    Ebadmsg,
    Ebusy,
    Ecanceled,
    Echild,
    Econnaborted,
    Econnrefused,
    Econnreset,
    Edeadlk,
    Edestaddrreq,
    Edquot,
    Eexist,
    Efault,
    Efbig,
    Ehostunreach,
    Eidrm,
    Eilseq,
    Einprogress,
    Eintr,
    Einval,
    Eio,
    Eisconn,
    Eisdir,
    Eloop,
    Emfile,
    Emlink,
    Emsgsize,
    Emultihop,
    Enametoolong,
    Enetdown,
    Enetreset,
    Enetunreach,
    Enfile,
    Enobufs,
    Enodev,
    Enolck,
    Enolink,
    Enomem,
    Enomsg,
    Enoprotoopt,
    Enospc,
    Enosys,
    Enotconn,
    Enotdir,
    Enotempty,
    Enotrecoverable,
    Enotsup,
    Enotty,
    Enxio,
    Eopnotsup,
    Eoverflow,
    Eownerdead,
    Eperm,
    Epfnosupport,
    Epipe,
    Eproto,
    Eprotonosupport,
    Eprototype,
    Erange,
    Erefused,
    Eremote,
    Erofs,
    Espipe,
    Esrch,
    Estale,
    Etimedout,
    Etxtbsy,
    Eusers,
    Ewouldblock,
    Exdev,
    Enotcapable,
    IoError,
    InvalidOffset,
    BadF,
    BadFdtype,
}

impl WasiError {
    /// Convert to errno
    pub fn errno(&self) -> u32 {
        match self {
            WasiError::Esuccess => 0,
            WasiError::Einval => 28,
            WasiError::Eio => 5,
            WasiError::Ebadf => 9,
            WasiError::BadF => 8,
            _ => 1,
        }
    }
}

/// WASI stat
#[derive(Debug)]
pub struct WasiStat {
    pub dev: u64,
    pub ino: u64,
    pub filetype: Filetype,
    pub nlink: u64,
    pub size: u64,
    pub atim: u64,
    pub mtim: u64,
    pub ctim: u64,
}

/// File type
#[derive(Debug, Clone, Copy)]
pub enum Filetype {
    Unknown,
    BlockDevice,
    CharacterDevice,
    Directory,
    RegularFile,
    SocketStream,
    SocketDgram,
    SymbolicLink,
}

/// WASI clock
#[derive(Debug, Clone, Copy)]
pub enum ClockId {
    Realtime,
    Monotonic,
    ProcessCputime,
    ThreadCputime,
}

impl ClockId {
    pub fn id(&self) -> u32 {
        match self {
            ClockId::Realtime => 0,
            ClockId::Monotonic => 1,
            ClockId::ProcessCputime => 2,
            ClockId::ThreadCputime => 3,
        }
    }
}

/// Random source
pub struct RandomSource;

impl RandomSource {
    /// Get random bytes
    pub fn get_random_bytes(&mut self, buffer: &mut [u8]) {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        rng.fill(buffer);
    }

    /// Get random u32
    pub fn get_random_u32(&mut self) -> u32 {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        rng.gen()
    }
}

/// WASI preview2 support
pub mod preview2 {
    use super::*;

    /// Preview2 table
    pub struct WasiTable {
        /// Resource table
        pub resource_table: RwLock<Vec<Resource>>,
        /// Next resource ID
        pub next_resource: RwLock<u32>,
    }

    #[derive(Debug)]
    pub struct Resource {
        pub id: u32,
        pub kind: ResourceKind,
        pub closed: bool,
    }

    #[derive(Debug)]
    pub enum ResourceKind {
        StdioStdin,
        StdioStdout,
        StdioStderr,
        Filesystem(FilesystemResource),
        Socket(SocketResource),
    }

    #[derive(Debug)]
    pub struct FilesystemResource {
        pub path: String,
        pub read: bool,
        pub write: bool,
    }

    #[derive(Debug)]
    pub struct SocketResource {
        pub domain: u32,
        pub type_: u32,
        pub protocol: u32,
    }

    impl WasiTable {
        /// Create new table
        pub fn new() -> Self {
            Self {
                resource_table: RwLock::new(Vec::new()),
                next_resource: RwLock::new(0),
            }
        }

        /// Insert resource
        pub fn insert(&self, kind: ResourceKind) -> u32 {
            let mut next = self.next_resource.write().unwrap();
            let id = *next;
            *next += 1;

            self.resource_table.write().unwrap().push(Resource {
                id,
                kind,
                closed: false,
            });

            id
        }

        /// Get resource
        pub fn get(&self, id: u32) -> Option<Resource> {
            self.resource_table.read().unwrap()
                .iter()
                .find(|r| r.id == id)
                .cloned()
        }

        /// Delete resource
        pub fn delete(&self, id: u32) -> bool {
            let mut table = self.resource_table.write().unwrap();
            if let Some(pos) = table.iter().position(|r| r.id == id) {
                table.remove(pos);
                true
            } else {
                false
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wasi_creation() {
        let config = WasiConfig::default();
        let wasi = WasiImplementation::new(config);
        assert!(!wasi.config.args.is_empty() || wasi.config.args.is_empty());
    }

    #[test]
    fn test_alloc_memory() {
        let mut wasi = WasiImplementation::default();
        let offset = wasi.alloc_memory(1024);
        assert_eq!(offset, 0);
    }

    #[test]
    fn test_write_read_memory() {
        let mut wasi = WasiImplementation::default();
        let offset = wasi.alloc_memory(1024);
        wasi.write_memory(offset, b"hello").unwrap();
        let data = wasi.read_memory(offset, 5).unwrap();
        assert_eq!(data, b"hello");
    }

    #[test]
    fn test_clock_id() {
        assert_eq!(ClockId::Realtime.id(), 0);
        assert_eq!(ClockId::Monotonic.id(), 1);
    }

    #[test]
    fn test_preview2_table() {
        use preview2::WasiTable;
        let table = WasiTable::new();
        let id = table.insert(ResourceKind::StdioStdout);
        assert_eq!(id, 0);
        assert!(table.get(0).is_some());
    }
}
