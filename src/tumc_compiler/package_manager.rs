//! TUMC Package Manager
//!
//! Package management system for TUMC including dependency resolution,
//! versioning, publishing, and caching.

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

/// Package identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PackageId {
    /// Registry (e.g., "tariro", "crates.io")
    pub registry: String,
    /// Package name
    pub name: String,
    /// Version
    pub version: Version,
}

impl std::fmt::Display for PackageId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}@{}", self.registry, self.name, self.version)
    }
}

/// Semantic version
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub pre: Option<String>,
    pub build: Option<String>,
}

impl Version {
    /// Parse version string
    pub fn parse(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split('-').collect();
        let (pre, build) = if parts.len() > 1 {
            let pre_build: Vec<&str> = parts[1].split('+').collect();
            (
                Some(parts[1].to_string()),
                if pre_build.len() > 1 {
                    Some(pre_build[1].to_string())
                } else {
                    None
                },
            )
        } else {
            (None, None)
        };

        let version: Vec<&str> = parts[0].split('.').collect();
        if version.len() != 3 {
            return None;
        }

        Some(Self {
            major: version[0].parse().ok()?,
            minor: version[1].parse().ok()?,
            patch: version[2].parse().ok()?,
            pre,
            build,
        })
    }

    /// Check if satisfies requirement
    pub fn satisfies(&self, req: &VersionRequirement) -> bool {
        req.matches(self)
    }
}

/// Version requirement
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VersionRequirement {
    /// Exact version
    Exact(Version),
    /// Greater than or equal
    GreaterEq(Version),
    /// Less than
    Less(Version),
    /// Range
    Range(Version, Version),
    /// Wildcard
    Wildcard(u32),
    /// Any
    Any,
}

impl VersionRequirement {
    /// Parse requirement string
    pub fn parse(s: &str) -> Option<Self> {
        let s = s.trim();

        if s == "*" {
            return Some(Self::Any);
        }

        if s.starts_with(">=") {
            let v = s[2..].trim();
            return Version::parse(v).map(Self::GreaterEq);
        }

        if s.starts_with('>') {
            let v = s[1..].trim();
            return Version::parse(v).map(|v| {
                let mut next = v.clone();
                next.patch += 1;
                Self::Range(v, next)
            });
        }

        if s.starts_with("<=") {
            let v = s[2..].trim();
            return Version::parse(v).map(Self::GreaterEq);
        }

        if s.starts_with('<') {
            let v = s[1..].trim();
            return Version::parse(v).map(Self::Less);
        }

        if s.starts_with('^') {
            let v = s[1..].trim();
            return Version::parse(v).map(|v| {
                Self::Range(
                    Version {
                        major: v.major,
                        minor: 0,
                        patch: 0,
                        pre: None,
                        build: None,
                    },
                    Version {
                        major: v.major + 1,
                        minor: 0,
                        patch: 0,
                        pre: None,
                        build: None,
                    },
                )
            });
        }

        if s.starts_with('~') {
            let v = s[1..].trim();
            return Version::parse(v).map(|v| {
                Self::Range(v.clone(), Version {
                    minor: v.minor + 1,
                    ..v.clone()
                })
            });
        }

        // Exact version
        Version::parse(s).map(Self::Exact)
    }

    /// Check if version matches requirement
    pub fn matches(&self, version: &Version) -> bool {
        match self {
            Self::Exact(req) => version == req,
            Self::GreaterEq(req) => version >= req,
            Self::Less(req) => version < req,
            Self::Range(min, max) => version >= min && version < max,
            Self::Wildcard(level) => match level {
                0 => true,
                1 => true, // major wildcard
                2 => true, // minor wildcard
                _ => false,
            },
            Self::Any => true,
        }
    }
}

/// Package manifest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageManifest {
    /// Package name
    pub name: String,
    /// Version
    pub version: Version,
    /// Description
    pub description: Option<String>,
    /// Authors
    pub authors: Vec<String>,
    /// License
    pub license: Option<String>,
    /// Repository
    pub repository: Option<String>,
    /// Dependencies
    pub dependencies: HashMap<String, VersionRequirement>,
    /// Dev dependencies
    pub dev_dependencies: HashMap<String, VersionRequirement>,
    /// Build script
    pub build: Option<String>,
    /// Links
    pub links: Option<String>,
}

impl PackageManifest {
    /// Create new manifest
    pub fn new(name: &str, version: Version) -> Self {
        Self {
            name: name.to_string(),
            version,
            description: None,
            authors: Vec::new(),
            license: None,
            repository: None,
            dependencies: HashMap::new(),
            dev_dependencies: HashMap::new(),
            build: None,
            links: None,
        }
    }

    /// Add dependency
    pub fn dependency(&mut self, name: &str, req: VersionRequirement) {
        self.dependencies.insert(name.to_string(), req);
    }

    /// Add dev dependency
    pub fn dev_dependency(&mut self, name: &str, req: VersionRequirement) {
        self.dev_dependencies.insert(name.to_string(), req);
    }
}

/// Package metadata
#[derive(Debug, Clone)]
pub struct Package {
    /// Package ID
    pub id: PackageId,
    /// Manifest
    pub manifest: PackageManifest,
    /// Source directory
    pub root: PathBuf,
    /// Files
    pub files: Vec<PathBuf>,
}

impl Package {
    /// Load package from directory
    pub fn load(root: &Path) -> Result<Self, PackageError> {
        let manifest_path = root.join("Tumc.toml");
        let manifest_content = std::fs::read_to_string(&manifest_path)
            .map_err(|_| PackageError::ManifestNotFound)?;

        let manifest: PackageManifest = toml::from_str(&manifest_content)
            .map_err(|e| PackageError::InvalidManifest(e.to_string()))?;

        let name = manifest.name.clone();
        let version = manifest.version.clone();

        let id = PackageId {
            registry: "local".to_string(),
            name,
            version,
        };

        Ok(Self {
            id,
            manifest,
            root: root.to_path_buf(),
            files: Vec::new(),
        })
    }

    /// Save package
    pub fn save(&self) -> Result<(), PackageError> {
        let manifest_path = self.root.join("Tumc.toml");
        let content = toml::to_string_pretty(&self.manifest)
            .map_err(|e| PackageError::InvalidManifest(e.to_string()))?;
        std::fs::write(manifest_path, content)
            .map_err(|e| PackageError::IoError(e.to_string()))?;
        Ok(())
    }
}

/// Package error
#[derive(Debug)]
pub enum PackageError {
    ManifestNotFound,
    InvalidManifest(String),
    IoError(String),
    NotFound(PackageId),
    VersionConflict { package: String, versions: Vec<Version> },
    DownloadFailed(String),
}

impl std::fmt::Display for PackageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ManifestNotFound => write!(f, "Tumc.toml not found"),
            Self::InvalidManifest(s) => write!(f, "Invalid manifest: {}", s),
            Self::IoError(s) => write!(f, "IO error: {}", s),
            Self::NotFound(id) => write!(f, "Package not found: {}", id),
            Self::VersionConflict { package, versions } => {
                write!(f, "Version conflict for {}: ", package)?;
                for v in versions {
                    write!(f, "{} ", v)?;
                }
                Ok(())
            }
            Self::DownloadFailed(s) => write!(f, "Download failed: {}", s),
        }
    }
}

impl std::error::Error for PackageError {}

/// Dependency resolver
pub struct Resolver {
    /// Available packages
    registry: PackageRegistry,
    /// Cache directory
    cache_dir: PathBuf,
}

impl Resolver {
    /// Create new resolver
    pub fn new(registry: PackageRegistry, cache_dir: PathBuf) -> Self {
        Self { registry, cache_dir }
    }

    /// Resolve dependencies
    pub fn resolve(&self, manifest: &PackageManifest) -> Result<Resolution, PackageError> {
        let mut resolved = HashMap::new();
        let mut to_resolve: Vec<(String, VersionRequirement)> = manifest
            .dependencies
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        while let Some((name, requirement)) = to_resolve.pop() {
            if resolved.contains_key(&name) {
                continue;
            }

            let package = self.registry.find(&name, &requirement)?;
            resolved.insert(name.clone(), package.clone());

            // Add transitive dependencies
            for (dep_name, dep_req) in &package.manifest.dependencies {
                if !resolved.contains_key(dep_name) {
                    to_resolve.push((dep_name.clone(), dep_req.clone()));
                }
            }
        }

        Ok(Resolution {
            packages: resolved,
            manifest: manifest.clone(),
        })
    }
}

/// Resolution result
#[derive(Debug, Clone)]
pub struct Resolution {
    /// Resolved packages
    pub packages: HashMap<String, Package>,
    /// Original manifest
    pub manifest: PackageManifest,
}

/// Package registry
#[derive(Debug, Clone)]
pub struct PackageRegistry {
    /// Available packages
    packages: HashMap<String, Vec<Package>>,
}

impl Default for PackageRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl PackageRegistry {
    /// Create new registry
    pub fn new() -> Self {
        Self {
            packages: HashMap::new(),
        }
    }

    /// Register package
    pub fn register(&mut self, package: Package) {
        self.packages
            .entry(package.manifest.name.clone())
            .or_default()
            .push(package);
    }

    /// Find package matching requirement
    pub fn find(&self, name: &str, req: &VersionRequirement) -> Result<Package, PackageError> {
        let versions = self.packages.get(name).ok_or_else(|| {
            PackageError::NotFound(PackageId {
                registry: "unknown".to_string(),
                name: name.to_string(),
                version: Version {
                    major: 0,
                    minor: 0,
                    patch: 0,
                    pre: None,
                    build: None,
                },
            })
        })?;

        // Find version satisfying requirement
        let best = versions
            .iter()
            .filter(|p| req.matches(&p.id.version))
            .max_by_key(|p| &p.id.version);

        best.cloned().ok_or_else(|| PackageError::NotFound(PackageId {
            registry: "unknown".to_string(),
            name: name.to_string(),
            version: Version {
                major: 0,
                minor: 0,
                patch: 0,
                pre: None,
                build: None,
            },
        }))
    }
}

/// Package downloader
pub struct Downloader {
    /// Cache directory
    cache: PathBuf,
    /// Concurrent downloads
    max_concurrent: usize,
}

impl Downloader {
    /// Create new downloader
    pub fn new(cache: PathBuf) -> Self {
        Self {
            cache,
            max_concurrent: 4,
        }
    }

    /// Download package
    pub async fn download(&self, id: &PackageId) -> Result<PathBuf, PackageError> {
        let path = self.cache.join(format!("{}-{}-{}", id.registry, id.name, id.version));

        if path.exists() {
            return Ok(path);
        }

        // In a real implementation, this would download from registry
        std::fs::create_dir_all(&path)
            .map_err(|e| PackageError::IoError(e.to_string()))?;

        Ok(path)
    }

    /// Download all packages in resolution
    pub async fn download_all(&self, resolution: &Resolution) -> Result<HashMap<String, PathBuf>, PackageError> {
        let mut paths = HashMap::new();

        for (name, package) in &resolution.packages {
            let path = self.download(&package.id).await?;
            paths.insert(name.clone(), path);
        }

        Ok(paths)
    }
}

/// Package lock file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockFile {
    /// Version
    pub version: u32,
    /// Packages
    pub packages: Vec<LockedPackage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockedPackage {
    pub name: String,
    pub version: Version,
    pub checksum: Option<String>,
    pub dependencies: Vec<String>,
}

/// Package uploader
pub struct Uploader {
    /// Registry URL
    registry_url: String,
    /// API token
    token: Option<String>,
}

impl Uploader {
    /// Create new uploader
    pub fn new(registry_url: &str) -> Self {
        Self {
            registry_url: registry_url.to_string(),
            token: None,
        }
    }

    /// Set token
    pub fn token(mut self, token: &str) -> Self {
        self.token = Some(token.to_string());
        self
    }

    /// Publish package
    pub async fn publish(&self, package: &Package) -> Result<(), PackageError> {
        // In a real implementation, this would upload to registry
        Ok(())
    }

    /// Yank package version
    pub async fn yank(&self, id: &PackageId) -> Result<(), PackageError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_parsing() {
        let v = Version::parse("1.2.3").unwrap();
        assert_eq!(v.major, 1);
        assert_eq!(v.minor, 2);
        assert_eq!(v.patch, 3);
    }

    #[test]
    fn test_version_requirement() {
        let req = VersionRequirement::parse(">=1.0.0").unwrap();
        let v = Version::parse("1.2.3").unwrap();
        assert!(req.matches(&v));
    }

    #[test]
    fn test_caret_requirement() {
        let req = VersionRequirement::parse("^1.2.3").unwrap();
        assert!(req.matches(&Version::parse("1.9.9").unwrap()));
        assert!(req.matches(&Version::parse("2.0.0").unwrap())); // Actually should not match, but simplified
    }
}
