#!/usr/bin/env python3
"""Vendor pinned first-generation inputs, preserving upstream identity."""
import argparse
from datetime import datetime, timezone
import hashlib
import json
import os
from pathlib import Path, PurePosixPath
import shutil
import stat
import subprocess
import tarfile
import tomllib
import urllib.request

ROOT = Path(__file__).resolve().parents[3]
OUT = ROOT / "verification/agones"
CACHE = ROOT / ".git/agones-first-generation-sources"


def digest(data):
    return hashlib.sha256(data).hexdigest()


def save(path, value):
    tmp = path.with_suffix(".tmp")
    tmp.write_text(json.dumps(value, indent=2, sort_keys=True)+"\n")
    tmp.replace(path)


def vendor(name, cached_git_archive=False, retained_archive_receipt=None, recover_staging=False):
    receipt = ROOT / f".git/agones-first-generation-source-{name}.json"
    source = json.loads(receipt.read_text())
    assert source["status"] == "FETCHED" and source["exit_code"] == 0
    repo = Path(source["cache"])
    commit, tree = source["commit"], source["tree"]
    listing = subprocess.check_output(["git", "-C", str(repo), "ls-tree", "-rz", commit]).split(b"\0")
    expected = {}
    for row in listing:
        if not row:
            continue
        header, path = row.split(b"\t", 1)
        mode, kind, sha = header.decode().split()
        assert kind == "blob", "required submodule must be vendored explicitly"
        expected[path.decode()] = (mode, sha)
    url = source["source"]["url"].removesuffix(".git")
    archive_url = url.replace("https://github.com/", "https://codeload.github.com/")+"/tar.gz/"+commit
    archive = CACHE / f"{name}-{commit}.tar.gz"
    job = OUT / f"first-generation-vendor-{name}.json"
    destination = ROOT / "third_party" / "first-generation" / name
    if job.exists():
        prior = json.loads(job.read_text())
        if prior["status"] == "VENDORED":
            assert prior["commit"] == commit and prior["tree"] == tree
            for path, record in prior["upstream_files"].items():
                target = destination / path
                data = os.readlink(target).encode() if target.is_symlink() else target.read_bytes()
                assert digest(data) == record["sha256"], "changed vendor input: "+path
            print(json.dumps({"name": name, "status": "REUSED_VERIFIED_VENDOR", "files": len(prior["upstream_files"])}))
            return
        allowed_recovery = cached_git_archive and prior["status"] == "SOURCE_ARCHIVE_CONNECTION_TIMEOUT" and archive.is_file()
        allowed_recovery |= recover_staging and prior["status"] == "ARCHIVE_RETAINED" and archive.is_file()
        if not allowed_recovery:
            raise SystemExit("Recover the existing vendor attempt before retry: "+str(job))
        assert prior["commit"] == commit and prior["tree"] == tree
    assert not destination.exists(), "never overwrite an existing source tree"
    intent = {"schema_version": 1, "name": name, "status": "INTENT_RECORDED", "repository": url,
              "commit": commit, "tree": tree, "archive_url": archive_url,
              "destination": str(destination.relative_to(ROOT)), "started_at": datetime.now(timezone.utc).isoformat()}
    if job.exists():
        intent["attempts"] = prior.get("attempts", [])
        if recover_staging:
            intent["recovery"] = "Revalidate the retained archive and every staged blob; retain the upstream Cargo SPDX license declaration when no license file exists."
    if cached_git_archive:
        assert archive.is_file(), "cached Git archive must already be present"
        intent["archive_provenance"] = {"transport": "LOCAL_PINNED_GIT_OBJECTS", "command": [
            "git", "archive", "--format=tar.gz", "--prefix="+name+"/", commit],
            "validation": "All archived paths, modes and content must match the pinned Git tree."}
    if retained_archive_receipt:
        retained = ROOT / retained_archive_receipt
        acquisition = json.loads(retained.read_text())
        assert acquisition["status"] == "ARCHIVE_RETAINED_UNVALIDATED"
        assert acquisition["source_commit"] == commit and acquisition["tree"] == tree
        assert Path(acquisition["archive"]).resolve() == archive.resolve()
        assert digest(archive.read_bytes()) == acquisition["sha256"]
        intent["archive_provenance"] = {"transport": acquisition["transport"],
            "receipt": str(retained.relative_to(ROOT)), "receipt_sha256": digest(retained.read_bytes()),
            "validation": "All archived paths, modes and content must match the pinned Git tree."}
    save(job, intent)
    if not archive.exists():
        tmp = archive.with_suffix(".download")
        request = urllib.request.Request(archive_url, headers={"User-Agent": "Agones-first-generation-source-vendor"})
        with urllib.request.urlopen(request, timeout=60) as response, tmp.open("wb") as output:
            while data := response.read(1024*1024):
                output.write(data)
        tmp.replace(archive)
    intent.update(status="ARCHIVE_RETAINED", archive_sha256=digest(archive.read_bytes()))
    save(job, intent)
    staging = CACHE / (name+"-vendor-staging")
    assert not staging.exists() or recover_staging, "recover existing source staging"
    staging.mkdir(exist_ok=recover_staging)
    files = {}
    with tarfile.open(archive, "r:gz") as tar:
        members = tar.getmembers()
        prefix = PurePosixPath(members[0].name).parts[0]
        for member in members:
            path = PurePosixPath(member.name)
            assert path.parts[0] == prefix and not path.is_absolute() and ".." not in path.parts
            if len(path.parts) == 1:
                continue
            relative = str(PurePosixPath(*path.parts[1:]))
            target = staging / relative
            if member.isdir():
                target.mkdir(parents=True, exist_ok=True)
                continue
            assert relative in expected and relative not in files, "unexpected/duplicate archive input"
            mode, blob = expected[relative]
            target.parent.mkdir(parents=True, exist_ok=True)
            if member.issym():
                assert mode == "120000" and not PurePosixPath(member.linkname).is_absolute()
                link_target = (target.parent / member.linkname).resolve()
                assert link_target.is_relative_to(staging.resolve()), "escaping upstream symlink"
                data = member.linkname.encode()
                if target.is_symlink():
                    assert os.readlink(target) == member.linkname, "changed retained source symlink"
                else:
                    assert not target.exists(), "changed retained source path"
                    target.symlink_to(member.linkname)
            else:
                assert member.isfile() and mode in ("100644", "100755")
                data = tar.extractfile(member).read()
                if target.exists():
                    assert target.read_bytes() == data, "changed retained source file"
                else:
                    target.write_bytes(data)
                target.chmod(0o755 if mode == "100755" else 0o644)
            assert hashlib.sha1(b"blob "+str(len(data)).encode()+b"\0"+data).hexdigest() == blob, "archive/Git blob mismatch: "+relative
            assert not data.startswith(b"version https://git-lfs.github.com/spec/v1"), "LFS payload must be resolved: "+relative
            files[relative] = {"git_blob": blob, "sha256": digest(data), "mode": mode, "bytes": len(data)}
    assert set(files) == set(expected), "source archive omitted required tracked paths"
    licenses = [p for p in files if Path(p).name.upper().startswith(("LICENSE", "COPYING", "NOTICE"))]
    declarations = {}
    cargo = staging / "Cargo.toml"
    if cargo.is_file():
        license_expression = tomllib.loads(cargo.read_text()).get("package", {}).get("license")
        if isinstance(license_expression, str) and license_expression:
            declarations["Cargo.toml"] = license_expression
    assert licenses or declarations, "upstream license files or explicit package license declaration missing"
    destination.parent.mkdir(parents=True, exist_ok=True)
    staging.rename(destination)
    overlays = []
    directories = [destination]+sorted(p for p in destination.rglob("*") if p.is_dir() and not p.is_symlink())
    for directory in directories:
        guidance = directory / "AGENTS.MD"
        if not guidance.exists():
            link = os.path.relpath(ROOT / "AGENTS.MD", directory)
            text = "# AGENTS.MD\n\nProject guidance is inherited from [the repository root]("+link+").\nThis is an owned guidance overlay, not upstream CPG/Genco/frontend source.\n"
            guidance.write_text(text)
            overlays.append({"path": str(guidance.relative_to(destination)), "sha256": digest(text.encode()),
                             "reason": "User requires AGENTS.MD in every project directory.", "classification": "OWNED_GUIDANCE_OVERLAY"})
    for directory in [ROOT / "third_party", ROOT / "third_party/first-generation"]:
        guidance = directory / "AGENTS.MD"
        if not guidance.exists():
            guidance.write_text("# AGENTS.MD\n\nProject guidance is inherited from [the repository root]("+os.path.relpath(ROOT / "AGENTS.MD", directory)+").\n")
    intent.update(status="VENDORED", upstream_files=files, license_files=licenses,
                  upstream_file_count=len(files), upstream_bytes=sum(r["bytes"] for r in files.values()),
                  upstream_content_manifest_sha256=digest(json.dumps(files, sort_keys=True, separators=(",", ":")).encode()),
                  patches=[], guidance_overlays=overlays, license_declarations=declarations,
                  upstream_license_file_absent=not bool(licenses),
                  pristine_identity="Recorded Git commit/tree and every upstream blob are unchanged; separately inventoried guidance additions are not presented as upstream source.",
                  finished_at=datetime.now(timezone.utc).isoformat())
    save(job, intent)
    print(json.dumps({"name": name, "status": "VENDORED", "commit": commit, "tree": tree,
                      "files": len(files), "bytes": intent["upstream_bytes"], "guidance_overlays": len(overlays)}), flush=True)


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("source", choices=["genco", "cpg", "joern", "goastgen", "flatgraph", "comprende", "better-comprehension", "grafeo"])
    transport = parser.add_mutually_exclusive_group()
    transport.add_argument("--cached-git-archive", action="store_true")
    transport.add_argument("--retained-archive-receipt")
    parser.add_argument("--recover-staging", action="store_true")
    args = parser.parse_args()
    vendor(args.source, args.cached_git_archive, args.retained_archive_receipt, args.recover_staging)
