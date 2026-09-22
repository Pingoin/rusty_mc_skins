#!/usr/bin/env python3
"""
Sync Cargo.toml versions from `git describe` (tag-aware) to a Cargo-valid semver.

- Prefers `git describe --tags --always --abbrev=7 --dirty=-modified`
- Falls back to `git rev-parse --short=7 HEAD`
- Converts `0.6.2-1-1-g352f9eb` (tag 0.6.2-1, 1 commit) -> `0.6.2-1+1.g352f9eb`
- Writes to web/Cargo.toml and api/Cargo.toml [package] version
- Called by `just sync-version` and by `.githooks/pre-commit`
"""

import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
WEB_TOML = ROOT / "web" / "Cargo.toml"
API_TOML = ROOT / "api" / "Cargo.toml"

def run(cmd):
    try:
        out = subprocess.check_output(cmd, cwd=ROOT, stderr=subprocess.DEVNULL)
        return out.decode().strip()
    except Exception:
        return ""

def get_base_version():
    try:
        text = WEB_TOML.read_text()
        for line in text.splitlines():
            t = line.strip()
            if t.startswith("version") and "=" in t:
                # only within [package] – naive: first version line is package version
                # (dependencies use `version = { workspace = true }` or quoted)
                if '"' in line:
                    v = line.split('"')[1]
                    return v.split("+")[0].split("-")[0] if False else v.split("+")[0]
        return "0.1.0"
    except Exception:
        return "0.1.0"

def sanitize_for_cargo(v: str):
    if not v or v == "unknown":
        return None
    # only allow alphanum + . + - in cargo version (build metadata + allowed)
    s = "".join(c if c.isalnum() or c in ".-+" else "." for c in v)
    if not s or not s[0].isdigit():
        s = f"0.1.0+{s}"
    # ensure major.minor.patch
    base = s.split("+")[0].split("-")[0]
    dots = base.count(".")
    if dots == 0:
        s = s.replace(base, f"{base}.0.0", 1)
    elif dots == 1:
        s = s.replace(base, f"{base}.0", 1)
    return s

def sanitize_describe(raw: str):
    dirty = raw.endswith("-modified")
    clean = raw[:-9] if dirty else raw
    clean = clean.rstrip("-")

    # try -g split (tag-distance-g<hash> or tag-g<hash>)
    if "-g" in clean:
        left, h = clean.rsplit("-g", 1)
        # left = "0.6.2-1-1" or "0.6.2-1" or "0.6.2"
        if "-" in left:
            # split last '-' as distance
            tag, dist = left.rsplit("-", 1)
            # dist should be numeric
            if dist.isdigit():
                v = f"{tag}+{dist}.g{h}"
            else:
                # tag itself contained dash but dist not numeric -> treat as tag+g
                v = f"{left}+g{h}"
        else:
            v = f"{left}+g{h}"
        if dirty:
            v += ".modified"
        return sanitize_for_cargo(v) or v

    # tag-distance without g, e.g. "0.6.2-1"
    if "-" in clean and "." in clean.split("-")[0]:
        parts = clean.split("-")
        if len(parts) >= 2 and parts[0].count(".") >= 1:
            tag = parts[0]
            # handle tag like 0.6.2-1 already is tag; if we have 0.6.2-1 this is actually tag itself
            # but describe --long always includes -g, so this branch is for `git describe --tags --abbrev=0` style
            # treat as tag+dist
            if parts[-1].isdigit() and len(parts) > 2:
                # 0.6.2-1-1 case? already handled above
                pass
            v = f"{clean.replace('-', '+', 1)}"
            if dirty:
                v += ".modified"
            return sanitize_for_cargo(v) or v

    # pure hash
    if len(clean) <= 12 and all(c in "0123456789abcdef" for c in clean.lower()):
        base = get_base_version().split("+")[0]
        # strip pre-release for base
        base = base.split("-")[0]
        # ensure base is semver
        if base.count(".") < 2:
            base = "0.1.0"
        v = f"{base}+g{clean}"
        if dirty:
            v += ".modified"
        return sanitize_for_cargo(v) or v

    v = clean
    if dirty:
        v += ".modified"
    return sanitize_for_cargo(v) or v

def update_cargo_version(path: Path, new_version: str):
    if not path.exists():
        return False
    text = path.read_text()
    lines = text.splitlines()
    out = []
    in_package = False
    updated = False
    for line in lines:
        trimmed = line.strip()
        if trimmed == "[package]":
            in_package = True
            out.append(line)
            continue
        if in_package and trimmed.startswith("["):
            in_package = False
        if in_package and not updated and trimmed.startswith("version") and "=" in trimmed and '"' in line:
            # replace first quoted version in [package]
            start = line.find('"')
            end = line.find('"', start + 1)
            if start != -1 and end != -1:
                out.append(f'{line[:start+1]}{new_version}{line[end:]}')
                updated = True
                continue
        out.append(line)
    new_text = "\n".join(out) + "\n"
    if new_text != text:
        path.write_text(new_text)
        print(f"updated {path.relative_to(ROOT)} -> {new_version}")
        return True
    else:
        print(f"unchanged {path.relative_to(ROOT)} ({new_version})")
        return False

def main():
    raw = run(["git", "describe", "--tags", "--always", "--abbrev=7", "--dirty=-modified"])
    if not raw:
        raw = run(["git", "rev-parse", "--short=7", "HEAD"])
        if run(["git", "status", "--porcelain"]):
            raw += "-modified"
    if not raw:
        print("no git version found, keeping Cargo.toml", file=sys.stderr)
        sys.exit(0)

    version = sanitize_describe(raw)
    if not version:
        print(f"cannot sanitize {raw}", file=sys.stderr)
        sys.exit(1)

    print(f"git describe raw: {raw} -> cargo: {version}")
    changed = False
    changed |= update_cargo_version(WEB_TOML, version)
    changed |= update_cargo_version(API_TOML, version)
    # also try workspace root if it ever gets a version
    root_toml = ROOT / "Cargo.toml"
    if root_toml.exists():
        # only if it has [package] version (workspace root currently doesn't)
        try:
            t = root_toml.read_text()
            if "[package]" in t and 'version =' in t:
                changed |= update_cargo_version(root_toml, version)
        except Exception:
            pass
    # exit code 0, git hook will `git add` if changed
    sys.exit(0)

if __name__ == "__main__":
    main()
