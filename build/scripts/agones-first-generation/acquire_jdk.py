#!/usr/bin/env python3
"""Recover the pinned JDK download using checked byte ranges and a final digest."""
from concurrent.futures import ThreadPoolExecutor
from datetime import datetime, timezone
import hashlib
import json
import os
from pathlib import Path
import urllib.request

ROOT = Path(__file__).resolve().parents[3]
PIN = ROOT / "verification/agones/first-generation-jdk-pin.json"
RECEIPT = ROOT / "verification/agones/first-generation-jdk-acquisition.json"


def save(record):
    temporary = RECEIPT.with_suffix(".tmp")
    temporary.write_text(json.dumps(record, indent=2, sort_keys=True) + "\n")
    temporary.replace(RECEIPT)


def main():
    pin = json.loads(PIN.read_text())
    prior = json.loads(RECEIPT.read_text())
    assert prior["status"] == "DOWNLOAD_FAILED", "recover an existing running or verified job first"
    target = Path(prior["archive"])
    partial = Path(str(target) + ".download")
    assert partial.is_file() and not target.exists()
    size = pin["asset"]["size"]
    prefix = partial.stat().st_size
    assert 0 < prefix < size
    cache = ROOT / ".git/agones-first-generation-jdk-ranges"
    cache.mkdir(exist_ok=True)
    url = f'https://api.github.com/repos/adoptium/temurin21-binaries/releases/assets/{pin["asset"]["id"]}'
    width = (size - prefix + 7) // 8
    ranges = [(start, min(start + width, size) - 1) for start in range(prefix, size, width)]
    record = dict(prior)
    record["attempts"] = prior.get("attempts", []) + [{k: v for k, v in prior.items() if k != "attempts"}]
    record.update(status="RUNNING_RANGED_RECOVERY", started_at=datetime.now(timezone.utc).isoformat(),
                  pid=os.getpid(), process_identity=Path(f"/proc/{os.getpid()}/stat").read_text(),
                  script=str(Path(__file__).relative_to(ROOT)), script_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
                  retained_prefix_bytes=prefix, ranges=ranges,
                  reason="Retain partial bytes after the bounded transfer timeout; use eight independent checked ranges.")
    save(record)

    def acquire(span):
        start, end = span
        destination = cache / f"{start}-{end}"
        marker = destination.with_suffix(".json")
        if destination.exists() and marker.exists():
            retained = json.loads(marker.read_text())
            assert retained["range"] == list(span)
            assert destination.stat().st_size == end - start + 1
            assert hashlib.sha256(destination.read_bytes()).hexdigest() == retained["sha256"]
            return
        temporary = destination.with_suffix(".download")
        request = urllib.request.Request(url, headers={"Accept": "application/octet-stream", "Range": f"bytes={start}-{end}"})
        with urllib.request.urlopen(request, timeout=45) as response:
            assert response.status == 206, "range request must not silently become a full download"
            assert response.headers["Content-Range"] == f"bytes {start}-{end}/{size}"
            with temporary.open("wb") as output:
                while data := response.read(1024 * 1024):
                    output.write(data)
        assert temporary.stat().st_size == end - start + 1
        checksum = hashlib.sha256(temporary.read_bytes()).hexdigest()
        temporary.replace(destination)
        marker.write_text(json.dumps({"range": span, "sha256": checksum}) + "\n")

    try:
        with ThreadPoolExecutor(max_workers=8) as executor:
            list(executor.map(acquire, ranges))
        assembled = target.with_suffix(".assembled")
        checksum = hashlib.sha256()
        with assembled.open("wb") as output:
            for path in [partial] + [cache / f"{start}-{end}" for start, end in ranges]:
                with path.open("rb") as source:
                    while data := source.read(1024 * 1024):
                        checksum.update(data)
                        output.write(data)
            output.flush()
            os.fsync(output.fileno())
        assert assembled.stat().st_size == size
        assert checksum.hexdigest() == pin["asset"]["sha256"], "complete JDK archive digest mismatch"
        assembled.replace(target)
        record.update(status="ARCHIVE_VERIFIED", observed_sha256=checksum.hexdigest())
    except Exception as error:
        record.update(status="RANGED_RECOVERY_FAILED", error=f"{type(error).__name__}: {error}")
        raise
    finally:
        record["finished_at"] = datetime.now(timezone.utc).isoformat()
        save(record)
        print(json.dumps({key: record.get(key) for key in ("status", "observed_sha256", "error")}), flush=True)


if __name__ == "__main__":
    main()
