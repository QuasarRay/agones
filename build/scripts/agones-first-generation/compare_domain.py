#!/usr/bin/env python3
"""Regenerate the pinned Scala domain in isolation and compare every output identity."""
from datetime import datetime, timezone
import hashlib
import json
import os
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[3]
EVIDENCE = ROOT / "verification/agones"
CACHE = ROOT / ".git/agones-first-generation-sources/cpg"
BUILD = ROOT / ".git/agones-first-generation-cpg-build"
RECEIPT = EVIDENCE / "first-generation-cpg-regeneration.json"


def sha256(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def save(record):
    temporary = RECEIPT.with_suffix(".tmp")
    temporary.write_text(json.dumps(record, indent=2, sort_keys=True) + "\n")
    temporary.replace(RECEIPT)


def main():
    assert not RECEIPT.exists(), "recover the existing source-bound regeneration receipt before launching another job"
    vendor_file = EVIDENCE / "first-generation-vendor-cpg.json"
    generator_file = EVIDENCE / "first-generation-vendor-flatgraph.json"
    vendor = json.loads(vendor_file.read_text())
    generator = json.loads(generator_file.read_text())
    jdk_file = EVIDENCE / "first-generation-jdk-acquisition.json"
    jdk = json.loads(jdk_file.read_text())
    lfs_file = EVIDENCE / "first-generation-git-lfs-acquisition.json"
    lfs = json.loads(lfs_file.read_text())
    sbt_file = EVIDENCE / "first-generation-sbt-pins.json"
    sbt = next(row for row in json.loads(sbt_file.read_text())["launchers"] if row["version"] == "1.12.5")
    assert vendor["status"] == generator["status"] == "VENDORED"
    assert jdk["status"] == lfs["status"] == "INSTALLED"
    java = Path(jdk["java"]["path"])
    git_lfs = Path(lfs["binary"]["path"])
    assert sha256(java) == jdk["java"]["sha256"]
    assert sha256(git_lfs) == lfs["binary"]["sha256"]
    assert sha256(Path(sbt["path"])) == sbt["sha256"]
    for path, identity in vendor["upstream_files"].items():
        source = CACHE / path
        data = os.readlink(source).encode() if source.is_symlink() else source.read_bytes()
        assert hashlib.sha256(data).hexdigest() == identity["sha256"], f"changed isolated build input: {path}"
    assert subprocess.check_output(["git", "-C", str(CACHE), "rev-parse", "HEAD"], text=True).strip() == vendor["commit"]
    BUILD.mkdir(exist_ok=True)
    generated = BUILD / "regenerated-domain"
    assert not generated.exists(), "never overwrite an interrupted regeneration"
    repositories = BUILD / "repositories"
    repositories.write_text("[repositories]\nmaven-central: https://repo.maven.apache.org/maven2/\n")
    command = [str(java), "-Xmx3g", "-XX:ActiveProcessorCount=4", "-Dsbt.supershell=false", "-Dsbt.log.noformat=true",
               "-Dsbt.override.build.repos=true", f"-Dsbt.repository.config={repositories}",
               f"-Dsbt.boot.directory={BUILD / 'boot'}", f"-Dsbt.global.base={BUILD / 'global'}",
               f"-Dsbt.ivy.home={BUILD / 'ivy'}", "-jar", sbt["path"],
               "schema/Compile/runMain flatgraph.codegen.Main "
               "--classWithSchema=io.shiftleft.codepropertygraph.schema.CpgSchema$ --field=instance "
               f"--out={generated} --scala3 --scalafmtConfig={CACHE / '.scalafmt.conf'}",
               "show schema/Compile/fullClasspath"]
    environment = dict(os.environ, JAVA_HOME=str(java.parent.parent),
                       PATH=str(git_lfs.parent) + os.pathsep + str(java.parent) + os.pathsep + os.environ["PATH"],
                       COURSIER_CACHE=str(BUILD / "coursier"),
                       COURSIER_REPOSITORIES="https://repo.maven.apache.org/maven2/", LANG="C.UTF-8", TZ="UTC")
    for key in ("JAVA_TOOL_OPTIONS", "JDK_JAVA_OPTIONS", "_JAVA_OPTIONS"):
        environment.pop(key, None)
    log = BUILD / "regeneration.log"
    record = {"schema_version": 1, "status": "INTENT_RECORDED", "source_commit": vendor["commit"],
              "source_tree": vendor["tree"], "vendor_manifest_sha256": sha256(vendor_file),
              "generator_manifest_sha256": sha256(generator_file), "script_sha256": sha256(Path(__file__)),
              "tool_receipts": {str(path.relative_to(ROOT)): sha256(path) for path in (jdk_file, lfs_file, sbt_file)},
              "command": command, "working_directory": str(CACHE), "log": str(log.relative_to(ROOT)),
              "output_root": str(generated), "source_patches": [],
              "environment": {key: environment[key] for key in ("JAVA_HOME", "PATH", "COURSIER_CACHE", "COURSIER_REPOSITORIES", "LANG", "TZ")},
              "started_at": datetime.now(timezone.utc).isoformat()}
    save(record)
    try:
        with log.open("wb") as output:
            process = subprocess.Popen(command, cwd=CACHE, env=environment, stdout=output, stderr=subprocess.STDOUT)
            record.update(status="RUNNING", pid=process.pid,
                          process_identity=Path(f"/proc/{process.pid}/stat").read_text())
            save(record)
            code = process.wait()
        record["exit_code"] = code
        record["log_sha256"] = sha256(log)
        if code != 0:
            record["status"] = "GENERATOR_BUILD_OR_EXECUTION_FAILED"
            return code
        prefix = "domainClasses/src/main/generated/"
        expected = {path for path in vendor["upstream_files"] if path.startswith(prefix)}
        actual = {prefix + str(path.relative_to(generated)): path for path in generated.rglob("*") if path.is_file()}
        record["missing_paths"] = sorted(expected - set(actual))
        record["added_paths"] = sorted(set(actual) - expected)
        record["files"] = {path: {"regenerated_path": str(actual[path]), "regenerated_sha256": sha256(actual[path]),
                                  "committed_sha256": vendor["upstream_files"][path]["sha256"]}
                           for path in sorted(expected & set(actual))}
        dependencies = {str(path.relative_to(BUILD)): {"sha256": sha256(path), "bytes": path.stat().st_size}
                        for path in sorted(BUILD.rglob("*")) if path.is_file() and path.suffix in (".jar", ".pom")}
        lock = EVIDENCE / "first-generation-cpg-generator-maven-lock.json"
        lock.write_text(json.dumps({"schema_version": 1, "artifacts": dependencies,
                                    "repository": "https://repo.maven.apache.org/maven2/"}, indent=2, sort_keys=True) + "\n")
        record["dependency_manifest"] = str(lock.relative_to(ROOT))
        record["dependency_manifest_sha256"] = sha256(lock)
        record["status"] = "COMPARED" if not record["missing_paths"] and not record["added_paths"] else "DIFFERING_FILE_SET"
        record["different_content_paths"] = [path for path, row in record["files"].items()
                                             if row["regenerated_sha256"] != row["committed_sha256"]]
        return 0 if record["status"] == "COMPARED" else 1
    except Exception as error:
        record.update(status="FAILED", error=f"{type(error).__name__}: {error}")
        raise
    finally:
        record["finished_at"] = datetime.now(timezone.utc).isoformat()
        save(record)
        print(json.dumps({key: record.get(key) for key in ("status", "exit_code", "different_content_paths", "missing_paths", "added_paths", "error")}), flush=True)


if __name__ == "__main__":
    raise SystemExit(main())
