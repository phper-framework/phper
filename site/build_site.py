#!/usr/bin/env python3

"""Build the PHPER website: the home page plus the versioned mdBook docs.

Subcommands:

    check-links  Offline link check of the mdBook sources.
    build        Build the whole site into an output directory.

The site layout is:

    index.html, style.css, home.js   home page, copied from site/home
    docs/index.html                  redirects to the default version
    docs/versions.json               versions shown by the switchers
    docs/master/                     docs of the working tree
    docs/vX.Y/                       docs of the newest `phper-vX.Y.Z` tag

Only the newest `MAX_RELEASES` minor series are built, and a series is
skipped when its tag has no `docs/book.toml`.

Only the standard library is used, Python 3.10 or later.
"""

from __future__ import annotations

import argparse
import html
import io
import json
import os
import re
import shutil
import subprocess
import sys
import tarfile
import tempfile
from dataclasses import dataclass
from pathlib import Path
from typing import Callable, Iterable
from urllib.parse import unquote

SITE_DIR = Path(__file__).resolve().parent
HOME_DIR = SITE_DIR / "home"
DEFAULT_REPO = SITE_DIR.parent

GITHUB_REPO_URL = "https://github.com/phper-framework/phper"
MAX_RELEASES = 3
MASTER = "master"
# Written into the output directory, so that a later build only clears
# directories created by this script.
OUT_MARKER = ".phper-site"

# ``` or ~~~ fences, possibly indented inside list items.
FENCE_RE = re.compile(r"^\s*(`{3,}|~{3,})")
INLINE_CODE_RE = re.compile(r"(`+)(.+?)\1")
# The target of an inline link: [text](target "title").
INLINE_LINK_RE = re.compile(r"\]\(\s*<?([^)\s>]+)>?(?:\s+\"[^\"]*\")?\s*\)")
# A reference definition: [label]: target
REF_DEF_RE = re.compile(r"^\s{0,3}\[[^\]]+\]:\s*<?([^\s>]+)>?")
# A rust path used as link target, such as `phper::values::ZVal`, or a module
# path of the old phper-doc crate, such as `_02_quick_start::...`.
RUST_PATH = r"(?:[A-Za-z_]\w*::)+[A-Za-z_]\w*!?"
RUSTDOC_TARGET_RE = re.compile(rf"^(?:{RUST_PATH}|_\d\w*)$")
# Shortcut intra-doc links, such as [`phper::values::ZVal`] or [str], which
# are not followed by a link target.
RUSTDOC_SHORTCUT_CODE_RE = re.compile(rf"\[`({RUST_PATH})`\](?![(\[])")
RUSTDOC_SHORTCUT_PLAIN_RE = re.compile(rf"\[(str|String|{RUST_PATH})\](?![(\[])")
GITHUB_MASTER_RE = re.compile(
    re.escape(GITHUB_REPO_URL) + r"/(?:blob|tree)/master/([^\s)>\"'#?]+)"
)
URL_SCHEME_RE = re.compile(r"^[A-Za-z][A-Za-z0-9+.-]*:")
TAG_RE = re.compile(r"^phper-v(\d+)\.(\d+)\.(\d+)$")
GITHUB_REF_RE = re.compile(re.escape(GITHUB_REPO_URL) + r"/(blob|tree)/master/")


def _iter_prose_lines(text: str):
    """Yields `(line_number, line)` for lines outside fenced code blocks."""
    fence = None
    for number, line in enumerate(text.splitlines(), start=1):
        match = FENCE_RE.match(line)
        if fence is None:
            if match:
                fence = match.group(1)
                continue
            yield number, line
        elif match and match.group(1)[0] == fence[0] and len(match.group(1)) >= len(fence):
            if not line.strip()[len(match.group(1)) :].strip():
                fence = None


def _display(path: Path) -> str:
    try:
        relative = os.path.relpath(path)
    except ValueError:
        return str(path)
    return str(path) if relative.startswith("..") else relative


def find_link_errors(src_dir: Path, repo_root: Path) -> list[str]:
    """Checks the links of every markdown file under `src_dir` offline.

    Reports rustdoc style links left over from the old phper-doc crate,
    GitHub `blob/master` and `tree/master` links to paths missing in
    `repo_root`, and relative `.md` links to missing files.
    """
    src_dir = Path(src_dir)
    repo_root = Path(repo_root)
    errors = []
    for path in sorted(src_dir.rglob("*.md")):
        name = _display(path)
        for number, line in _iter_prose_lines(path.read_text(encoding="utf-8")):
            where = f"{name}:{number}"
            ref_def = REF_DEF_RE.match(line)
            # The label of a reference definition is not a link by itself.
            prose = line[ref_def.end() :] if ref_def else line
            code_stripped = INLINE_CODE_RE.sub("", prose)

            for match in RUSTDOC_SHORTCUT_CODE_RE.finditer(prose):
                errors.append(f"{where}: rustdoc link [`{match.group(1)}`] needs a URL")
            for match in RUSTDOC_SHORTCUT_PLAIN_RE.finditer(code_stripped):
                errors.append(f"{where}: rustdoc link [{match.group(1)}] needs a URL")

            targets = INLINE_LINK_RE.findall(line)
            if ref_def:
                targets.append(ref_def.group(1))
            for target in targets:
                if RUSTDOC_TARGET_RE.match(target):
                    errors.append(f"{where}: rustdoc link target ({target}) needs a URL")
                elif not URL_SCHEME_RE.match(target) and not target.startswith(("#", "/")):
                    relative = unquote(target.split("#", 1)[0].split("?", 1)[0])
                    if relative.endswith(".md") and not (path.parent / relative).is_file():
                        errors.append(f"{where}: relative link ({target}) points to a missing file")

            for match in GITHUB_MASTER_RE.finditer(line):
                repo_path = unquote(match.group(1)).rstrip(".,;:")
                if not (repo_root / repo_path).exists():
                    errors.append(
                        f"{where}: GitHub link {match.group(0)} points to `{repo_path}`, "
                        "which does not exist in the repository"
                    )
    return errors


class SiteError(Exception):
    """A user facing build error."""


@dataclass(frozen=True)
class Version:
    """The newest release of a documented minor series."""

    name: str  # directory name, such as `v0.18`
    tag: str  # git tag, such as `phper-v0.18.2`
    release: str  # such as `0.18.2`


def parse_tag(tag: str) -> tuple[int, int, int] | None:
    """Parses a `phper-vX.Y.Z` tag; other tags, pre-releases included, give None."""
    match = TAG_RE.match(tag)
    if match is None:
        return None
    major, minor, patch = (int(part) for part in match.groups())
    return major, minor, patch


def select_versions(
    tags: Iterable[str],
    has_docs: Callable[[str], bool],
    limit: int = MAX_RELEASES,
) -> list[Version]:
    """Picks the newest tag of each of the newest `limit` minor series.

    Series whose tag has no docs are dropped, not replaced by older ones, so
    the site never shows more than the newest `limit` series. The result is
    sorted from newest to oldest.
    """
    newest: dict[tuple[int, int], tuple[tuple[int, int, int], str]] = {}
    for tag in tags:
        parsed = parse_tag(tag)
        if parsed is None:
            continue
        series = parsed[:2]
        if series not in newest or parsed > newest[series][0]:
            newest[series] = (parsed, tag)
    selected = sorted(newest.values(), reverse=True)[:limit]
    return [
        Version(name=f"v{major}.{minor}", tag=tag, release=f"{major}.{minor}.{patch}")
        for (major, minor, patch), tag in selected
        if has_docs(tag)
    ]


def rewrite_github_links(text: str, ref: str) -> str:
    """Points the `blob/master` and `tree/master` links of this repository to `ref`."""
    return GITHUB_REF_RE.sub(lambda match: f"{GITHUB_REPO_URL}/{match.group(1)}/{ref}/", text)


def list_tags(repo: Path) -> list[str]:
    result = subprocess.run(
        ["git", "-C", str(repo), "tag", "--list", "phper-v*"],
        check=True,
        capture_output=True,
        text=True,
    )
    return result.stdout.split()


def has_docs(repo: Path, tag: str) -> bool:
    """Tells whether `tag` contains `docs/book.toml`."""
    result = subprocess.run(
        ["git", "-C", str(repo), "cat-file", "-e", f"refs/tags/{tag}:docs/book.toml"],
        capture_output=True,
    )
    return result.returncode == 0


def export_docs(repo: Path, version: Version | None, dest: Path) -> None:
    """Writes the mdBook root of `version` to `dest`.

    `None` stands for master and copies the working tree, so that pull
    requests and local changes can be previewed. A tag is exported with
    `git archive`, and its GitHub links are pointed to the tag.
    """
    if version is None:
        docs = repo / "docs"
        # Skip the output of a local `mdbook build docs`.
        shutil.copytree(
            docs,
            dest,
            ignore=lambda directory, names: ["book"] if Path(directory) == docs else [],
        )
        return

    archive = subprocess.run(
        ["git", "-C", str(repo), "archive", "--format=tar", f"refs/tags/{version.tag}", "docs"],
        check=True,
        capture_output=True,
    ).stdout
    with tempfile.TemporaryDirectory() as tmp:
        with tarfile.open(fileobj=io.BytesIO(archive)) as tar:
            if hasattr(tarfile, "data_filter"):
                tar.extractall(tmp, filter="data")
            else:  # Python versions without extraction filters
                tar.extractall(tmp)
        shutil.move(os.path.join(tmp, "docs"), dest)
    for path in dest.rglob("*.md"):
        text = path.read_text(encoding="utf-8")
        path.write_text(rewrite_github_links(text, version.tag), encoding="utf-8")


def check_mdbook(mdbook: str) -> str:
    """Resolves the mdbook executable and makes sure it is 0.5 or later."""
    resolved = shutil.which(mdbook)
    if resolved is None:
        raise SiteError(f"mdbook not found: {mdbook}")
    output = subprocess.run(
        [resolved, "--version"], check=True, capture_output=True, text=True
    ).stdout
    match = re.search(r"v?(\d+)\.(\d+)\.\d+", output)
    if match is None or (int(match.group(1)), int(match.group(2))) < (0, 5):
        raise SiteError(f"mdBook 0.5 or later is required, got: {output.strip()}")
    return resolved


def build_book(mdbook: str, book_dir: Path, out_dir: Path, site_url: str) -> None:
    env = dict(os.environ, MDBOOK_OUTPUT__HTML__SITE_URL=site_url)
    subprocess.run(
        [mdbook, "build", str(book_dir), "--dest-dir", str(out_dir.resolve())],
        check=True,
        env=env,
    )


def versions_manifest(versions: list[Version]) -> dict:
    """The content of `docs/versions.json`, read by the version switchers."""
    entries = [
        {"name": version.name, "title": f"{version.name} ({version.release})", "tag": version.tag}
        for version in versions
    ]
    entries.append({"name": MASTER, "title": "master (unreleased)", "tag": MASTER})
    return {"default": versions[0].name if versions else MASTER, "versions": entries}


def redirect_page(target: str) -> str:
    target = html.escape(target, quote=True)
    return f"""<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<title>PHPER documentation</title>
<meta http-equiv="refresh" content="0; url=./{target}/">
</head>
<body>
<p>Redirecting to <a href="./{target}/">the {target} documentation</a>.</p>
</body>
</html>
"""


def normalize_base_url(base_url: str) -> str:
    """Makes the site base URL start and end with `/`."""
    stripped = base_url.strip().strip("/")
    return f"/{stripped}/" if stripped else "/"


def prepare_out(out: Path, repo: Path) -> Path:
    """Empties the output directory, refusing to clear unrelated directories."""
    out = out.resolve()
    repo = repo.resolve()
    if out == repo or out in repo.parents:
        raise SiteError(f"refusing to use {out} as output: it contains the repository")
    if out.exists():
        if not out.is_dir():
            raise SiteError(f"output {out} is not a directory")
        if any(out.iterdir()) and not (out / OUT_MARKER).exists():
            raise SiteError(
                f"refusing to clear {out}: it is not empty and was not created by this script"
            )
        shutil.rmtree(out)
    out.mkdir(parents=True)
    (out / OUT_MARKER).write_text("Generated by site/build_site.py.\n", encoding="utf-8")
    return out


def build_site(repo: Path, out: Path, base_url: str, mdbook: str) -> list[Version]:
    """Builds the home page and every docs version into `out`."""
    repo = repo.resolve()
    mdbook = check_mdbook(mdbook)
    errors = find_link_errors(repo / "docs" / "src", repo)
    if errors:
        raise SiteError("broken links:\n" + "\n".join(errors))
    base_url = normalize_base_url(base_url)
    out = prepare_out(out, repo)

    versions = select_versions(list_tags(repo), lambda tag: has_docs(repo, tag))
    docs_out = out / "docs"
    with tempfile.TemporaryDirectory(prefix="phper-site-") as tmp:
        for version in [*versions, None]:
            name = version.name if version else MASTER
            source = f"tag {version.tag}" if version else "the working tree"
            print(f"building docs/{name}/ from {source}", flush=True)
            book_dir = Path(tmp) / name
            export_docs(repo, version, book_dir)
            build_book(mdbook, book_dir, docs_out / name, f"{base_url}docs/{name}/")

    manifest = versions_manifest(versions)
    (docs_out / "versions.json").write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8")
    (docs_out / "index.html").write_text(redirect_page(manifest["default"]), encoding="utf-8")
    shutil.copytree(HOME_DIR, out, dirs_exist_ok=True)
    return versions


def cmd_check_links(args: argparse.Namespace) -> int:
    repo = Path(args.repo)
    src = Path(args.src) if args.src else repo / "docs" / "src"
    errors = find_link_errors(src, repo)
    for error in errors:
        print(error, file=sys.stderr)
    if errors:
        print(f"found {len(errors)} broken link(s)", file=sys.stderr)
        return 1
    print(f"links ok: {_display(src)}")
    return 0


def cmd_build(args: argparse.Namespace) -> int:
    versions = build_site(Path(args.repo), Path(args.out), args.base_url, args.mdbook)
    names = ", ".join([*(version.name for version in versions), MASTER])
    print(f"site written to {_display(Path(args.out))} with docs: {names}")
    return 0


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    commands = parser.add_subparsers(dest="command", required=True)

    check = commands.add_parser("check-links", help="check the links of the docs offline")
    check.add_argument("--repo", default=str(DEFAULT_REPO), help="repository root")
    check.add_argument("--src", help="mdBook source directory, defaults to <repo>/docs/src")
    check.set_defaults(func=cmd_check_links)

    build = commands.add_parser("build", help="build the whole site")
    build.add_argument("--repo", default=str(DEFAULT_REPO), help="repository root")
    build.add_argument("--out", required=True, help="output directory, cleared first")
    build.add_argument(
        "--base-url", default="/phper/", help="URL path the site is served under (default: /phper/)"
    )
    build.add_argument(
        "--mdbook",
        default=os.environ.get("MDBOOK", "mdbook"),
        help="mdbook executable, defaults to $MDBOOK or mdbook",
    )
    build.set_defaults(func=cmd_build)

    args = parser.parse_args(argv)
    try:
        return args.func(args)
    except SiteError as error:
        print(f"error: {error}", file=sys.stderr)
    except subprocess.CalledProcessError as error:
        print(f"error: command failed: {' '.join(map(str, error.cmd))}", file=sys.stderr)
        if error.stderr:
            stderr = error.stderr if isinstance(error.stderr, str) else error.stderr.decode()
            print(stderr, file=sys.stderr)
    return 1


if __name__ == "__main__":
    sys.exit(main())
