//! Native equivalents of source/package/stage build effects.
//! A package consumes an explicit payload manifest. It never scans unrelated
//! work or relabels copied implementation files as generated CPG bodies.
use std::{collections::BTreeSet, fs::{self, File}, io::{Read, Write}, path::{Component, Path, PathBuf}};
use sha2::{Digest, Sha256};
use serde::{Deserialize, Serialize};
use crate::{source_io::HasBuildIo, Error, Result};
use zip::{write::SimpleFileOptions, CompressionMethod, DateTime, ZipWriter};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ArchiveStamp { pub year: u16, pub month: u8, pub day: u8, pub hour: u8, pub minute: u8, pub second: u8 }
impl ArchiveStamp {
    pub fn zip_time(self) -> Result<DateTime> {
        DateTime::from_date_and_time(self.year, self.month, self.day, self.hour, self.minute, self.second)
            .map_err(|error| Error::IllegalArgument(error.to_string()))
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PayloadKind { File, Symlink }

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PackageEntry {
    pub source_id: String,
    pub source: PathBuf,
    pub destination: PathBuf,
    pub sha256: String,
    pub unix_permissions: u32,
    pub kind: PayloadKind,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PackageManifest {
    pub source_scope_identity: String,
    pub source_revision: String,
    pub source_version: String,
    pub native_version: String,
    pub native_revision: String,
    pub stamp: ArchiveStamp,
    pub required_destinations: BTreeSet<PathBuf>,
    pub entries: Vec<PackageEntry>,
    pub license_and_notice_paths: BTreeSet<PathBuf>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PackageReport {
    pub source_scope_identity: String,
    pub input_manifest_sha256: String,
    pub archive: PathBuf,
    pub archive_sha256: String,
    pub files: usize,
    pub compilation: String,
    pub correctness: String,
}

fn relative_name(path: &Path) -> Result<String> {
    if path.as_os_str().is_empty() || path.components().any(|part| !matches!(part, Component::Normal(_))) {
        return Err(Error::IllegalArgument(format!("package path must stay inside its root: {}", path.display())));
    }
    Ok(path.components().map(|part| part.as_os_str().to_str()
        .ok_or_else(|| Error::IllegalArgument("package destination is not UTF-8".into())))
        .collect::<Result<Vec<_>>>()?.join("/"))
}

impl PackageManifest {
    fn payload(&self) -> Result<Vec<(&PackageEntry, String)>> {
        if self.entries.is_empty() { return Err(Error::IllegalArgument("empty native source package".into())); }
        let mut destinations = BTreeSet::new();
        let mut source_ids = BTreeSet::new();
        let mut payload = Vec::new();
        for entry in &self.entries {
            let name = relative_name(&entry.destination)?;
            if !destinations.insert(entry.destination.clone()) {
                return Err(Error::IllegalArgument(format!("duplicate package destination {name}")));
            }
            source_ids.insert(entry.source_id.as_str());
            // Check existence before creating an output. This also prevents a
            // manifest from reading the archive while that archive is written.
            match entry.kind {
                PayloadKind::File => { if !entry.source.is_file() { return Err(Error::IllegalArgument(format!("package file is missing: {}", entry.source.display()))); } }
                PayloadKind::Symlink => { fs::read_link(&entry.source)?; }
            }
            payload.push((entry, name));
        }
        if destinations != self.required_destinations {
            return Err(Error::IllegalArgument("package required/emitted destination sets differ".into()));
        }
        if self.license_and_notice_paths.is_empty() || !self.license_and_notice_paths.is_subset(&destinations) {
            return Err(Error::IllegalArgument("package lacks its declared license/NOTICE closure".into()));
        }
        if source_ids.contains("") { return Err(Error::IllegalArgument("package input has no source identity".into())); }
        Ok(payload)
    }
}

fn copy_checked(entry: &PackageEntry, output: &mut impl Write) -> Result<()> {
    let mut input = File::open(&entry.source)?;
    let mut hash = Sha256::new();
    let mut buffer = [0; 65536];
    loop {
        let count = match input.read(&mut buffer) {
            Err(error) if error.kind() == std::io::ErrorKind::Interrupted => continue,
            result => result?,
        };
        if count == 0 { break; }
        hash.update(&buffer[..count]); output.write_all(&buffer[..count])?;
    }
    let digest: String = hash.finalize().iter().map(|byte| format!("{byte:02x}")).collect();
    if digest != entry.sha256 { return Err(Error::IllegalArgument(format!("changed package input {}", entry.source.display()))); }
    Ok(())
}

fn checked_symlink(entry: &PackageEntry) -> Result<String> {
    let target = fs::read_link(&entry.source)?;
    if cpg_rs::content_digest(target.as_os_str().as_encoded_bytes()) != entry.sha256 {
        return Err(Error::IllegalArgument(format!("changed package symlink {}", entry.source.display())));
    }
    target.to_str().map(str::to_owned).ok_or_else(|| Error::IllegalArgument("symlink target is not UTF-8".into()))
}

fn digest_file(path: &Path) -> Result<String> {
    let mut input = File::open(path)?;
    let mut hash = Sha256::new();
    let mut bytes = [0; 65536];
    loop {
        let count = match input.read(&mut bytes) {
            Err(error) if error.kind() == std::io::ErrorKind::Interrupted => continue,
            result => result?,
        };
        if count == 0 { break; }
        hash.update(&bytes[..count]);
    }
    Ok(hash.finalize().iter().map(|byte| format!("{byte:02x}")).collect())
}

pub fn prepare_publication_bundle<Context: HasBuildIo>(context: &Context, manifest_path: &Path, output: &Path) -> Result<PackageReport> {
    let bytes = fs::read(manifest_path)?;
    let manifest: PackageManifest = serde_json::from_slice(&bytes)?;
    let payload = manifest.payload()?;
    let time = manifest.stamp.zip_time()?;
    let metadata_name = "META-INF/cpg-native-package.json";
    if manifest.required_destinations.contains(Path::new(metadata_name)) {
        return Err(Error::IllegalArgument("package payload collides with its provenance entry".into()));
    }
    if let Some(parent) = output.parent() { fs::create_dir_all(parent)?; }
    let file = File::options().create_new(true).write(true).open(output)?;
    let mut archive = ZipWriter::new(file);
    for (entry, name) in &payload {
        let options = SimpleFileOptions::default().last_modified_time(time)
            .unix_permissions(entry.unix_permissions).compression_method(CompressionMethod::Deflated);
        match entry.kind {
            PayloadKind::File => { archive.start_file(name, options)?; copy_checked(entry, &mut archive)?; }
            PayloadKind::Symlink => {
                let target = checked_symlink(entry)?;
                archive.add_symlink(name, target, options)?;
            }
        }
    }
    let metadata = serde_json::json!({
        "sourceScopeIdentity": &manifest.source_scope_identity,
        "sourceRevision": &manifest.source_revision, "sourceVersion": &manifest.source_version,
        "nativeVersion": &manifest.native_version, "nativeRevision": &manifest.native_revision,
        "inputManifestSha256": cpg_rs::content_digest(&bytes),
        "files": manifest.entries.iter().map(|entry| serde_json::json!({
            "sourceId": &entry.source_id, "destination": &entry.destination,
            "sha256": &entry.sha256, "kind": entry.kind, "permissions": entry.unix_permissions,
        })).collect::<Vec<_>>(), "compilation": "UNVERIFIED", "correctness": "UNVERIFIED",
    });
    archive.start_file(metadata_name, SimpleFileOptions::default().last_modified_time(time).unix_permissions(0o644))?;
    archive.write_all(&serde_json::to_vec_pretty(&metadata)?)?;
    let mut file = archive.finish()?;
    file.flush()?;
    context.build_io().close_owned_file(file)?;
    let report = PackageReport {
        source_scope_identity: manifest.source_scope_identity.clone(),
        input_manifest_sha256: cpg_rs::content_digest(&bytes),
        archive: output.to_owned(), archive_sha256: digest_file(output)?,
        files: payload.len() + 1, compilation: "UNVERIFIED".into(), correctness: "UNVERIFIED".into(),
    };
    Ok(report)
}

/// Format an owned copy. Live generated bodies keep their emitter ownership.
/// The returned tree is a reviewable result; applying it still follows each
/// source owner's rule/adoption path.
pub fn copy_source_snapshot<Context: HasBuildIo>(context: &Context, manifest: &PackageManifest, output: &Path) -> Result<()> {
    if output.exists() { return Err(Error::IllegalArgument("source snapshot destination already exists".into())); }
    let payload = manifest.payload()?;
    if let Some(parent) = output.parent() { fs::create_dir_all(parent)?; }
    fs::create_dir(output)?;
    for (entry, _) in payload {
        let target = output.join(&entry.destination);
        if let Some(parent) = target.parent() { fs::create_dir_all(parent)?; }
        match entry.kind {
            PayloadKind::File => {
                let mut file = File::options().create_new(true).write(true).open(&target)?;
                copy_checked(entry, &mut file)?;
                #[cfg(unix)]
                { use std::os::unix::fs::PermissionsExt; file.set_permissions(fs::Permissions::from_mode(entry.unix_permissions & 0o777))?; }
                context.build_io().close_owned_file(file)?;
            }
            PayloadKind::Symlink => {
                let source_target = checked_symlink(entry)?;
                #[cfg(unix)]
                std::os::unix::fs::symlink(source_target, &target)?;
                #[cfg(windows)]
                std::os::windows::fs::symlink_file(source_target, &target)?;
            }
        }
    }
    Ok(())
}

pub fn stage_binary(binary: &Path, output: &Path, name: &str, resources: &[(PathBuf, PathBuf)]) -> Result<PathBuf> {
    let executable = output.join("bin").join(relative_name(Path::new(name))?);
    if executable.exists() { return Err(Error::IllegalArgument("staged executable already exists".into())); }
    fs::create_dir_all(executable.parent().expect("bin parent"))?;
    fs::copy(binary, &executable)?;
    for (source, relative) in resources {
        relative_name(relative)?;
        let target = output.join(relative);
        if target.exists() { return Err(Error::IllegalArgument(format!("staged resource already exists: {}", target.display()))); }
        if let Some(parent) = target.parent() { fs::create_dir_all(parent)?; }
        fs::copy(source, target)?;
    }
    Ok(executable)
}
