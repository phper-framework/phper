"""Tests for build_site.py, run with `python3 -m unittest discover -s site`.

The build tests need mdBook 0.5 or later, found through $MDBOOK or PATH, and
are skipped without it.
"""

import contextlib
import io
import json
import os
import subprocess
import tempfile
import textwrap
import unittest
from pathlib import Path

import build_site

GITHUB = "https://github.com/phper-framework/phper"
MDBOOK = os.environ.get("MDBOOK", "mdbook")


def mdbook_available():
    try:
        build_site.check_mdbook(MDBOOK)
    except (build_site.SiteError, OSError, subprocess.CalledProcessError):
        return False
    return True


class LinkCheckTest(unittest.TestCase):
    def setUp(self):
        self._tmp = tempfile.TemporaryDirectory()
        self.repo = Path(self._tmp.name)
        self.src = self.repo / "docs" / "src"
        (self.src / "guide").mkdir(parents=True)
        (self.repo / "phper" / "src").mkdir(parents=True)
        (self.repo / "phper" / "src" / "values.rs").write_text("")
        (self.repo / "examples" / "hello").mkdir(parents=True)
        (self.src / "intro.md").write_text("# Intro\n")

    def tearDown(self):
        self._tmp.cleanup()

    def write(self, name, text):
        path = self.src / name
        path.write_text(textwrap.dedent(text))
        return path

    def errors(self):
        return build_site.find_link_errors(self.src, self.repo)

    def test_valid_document(self):
        self.write(
            "guide/page.md",
            f"""\
            # Page

            See [`ZVal`]({GITHUB}/blob/master/phper/src/values.rs) and
            <{GITHUB}/tree/master/examples/hello>, also [intro](../intro.md#top),
            [anchor](#page), [str](https://doc.rust-lang.org/std/primitive.str.html)
            and `[str]` in inline code.

            [ref]: ../intro.md
            """,
        )
        self.assertEqual(self.errors(), [])

    def test_rustdoc_links(self):
        self.write(
            "page.md",
            """\
            [`ZVal`](phper::values::ZVal)
            [OsStr](std::ffi::OsStr)
            [first](_02_quick_start::_01_write_your_first_extension)
            The [`phper::values::ZVal`] wrapper.
            Pairs like [str] / [String].
            - [str]: dynamically sized type.
            """,
        )
        errors = self.errors()
        self.assertEqual(len(errors), 7, errors)
        for line in range(1, 7):
            self.assertTrue(
                any(f"page.md:{line}:" in error for error in errors),
                f"no error reported for line {line}: {errors}",
            )
        self.assertTrue(all("rustdoc" in error for error in errors), errors)

    def test_github_master_path_must_exist(self):
        self.write(
            "page.md",
            f"""\
            [ok]({GITHUB}/blob/master/phper/src/values.rs#L10)
            [missing]({GITHUB}/blob/master/phper/src/nope.rs)
            Full example is <{GITHUB}/tree/master/examples/gone>.
            [other repo](https://github.com/php/php-src/blob/master/Zend/zend_types.h)
            """,
        )
        errors = self.errors()
        self.assertEqual(len(errors), 2, errors)
        self.assertIn("page.md:2:", errors[0])
        self.assertIn("phper/src/nope.rs", errors[0])
        self.assertIn("page.md:3:", errors[1])
        self.assertIn("examples/gone", errors[1])

    def test_relative_markdown_link_must_exist(self):
        self.write(
            "guide/page.md",
            """\
            [ok](../intro.md)
            [missing](missing.md#section)

            [ref]: ./also-missing.md
            """,
        )
        errors = self.errors()
        self.assertEqual(len(errors), 2, errors)
        self.assertIn("page.md:2:", errors[0])
        self.assertIn("missing.md", errors[0])
        self.assertIn("page.md:4:", errors[1])

    def test_code_blocks_are_ignored(self):
        self.write(
            "page.md",
            """\
            1. Step:

               ```rust,no_run
               use phper::values::ZVal;
               let a = [str](phper::values::ZVal);
               // [String] [`phper::values::ZVal`] (missing.md)
               ```

            ~~~~text
            [x](phper::values::ZVal)
            ```
            still inside the tilde fence
            ~~~~
            """,
        )
        self.assertEqual(self.errors(), [])

    def test_cli_exit_code(self):
        self.assertEqual(
            build_site.main(["check-links", "--src", str(self.src), "--repo", str(self.repo)]),
            0,
        )
        self.write("page.md", "[`ZVal`](phper::values::ZVal)\n")
        self.assertEqual(
            build_site.main(["check-links", "--src", str(self.src), "--repo", str(self.repo)]),
            1,
        )


class SelectVersionsTest(unittest.TestCase):
    def select(self, tags, without_docs=()):
        versions = build_site.select_versions(tags, lambda tag: tag not in without_docs)
        return [(version.name, version.tag, version.release) for version in versions]

    def test_newest_three_series(self):
        tags = [
            "phper-v0.15.0",
            "phper-v0.16.1",
            "phper-v0.17.0",
            "phper-v0.17.5",
            "phper-v0.18.0",
        ]
        self.assertEqual(
            self.select(tags),
            [
                ("v0.18", "phper-v0.18.0", "0.18.0"),
                ("v0.17", "phper-v0.17.5", "0.17.5"),
                ("v0.16", "phper-v0.16.1", "0.16.1"),
            ],
        )

    def test_patches_compare_numerically(self):
        tags = ["phper-v0.17.9", "phper-v0.17.10", "phper-v0.17.2", "phper-v1.0.0", "phper-v0.9.0"]
        self.assertEqual(
            self.select(tags),
            [
                ("v1.0", "phper-v1.0.0", "1.0.0"),
                ("v0.17", "phper-v0.17.10", "0.17.10"),
                ("v0.9", "phper-v0.9.0", "0.9.0"),
            ],
        )

    def test_other_tags_are_ignored(self):
        tags = [
            "phper-v0.19.0-alpha.1",
            "phper-v0.18.0",
            "phper-doc-v0.16.0",
            "phper-sys-v0.20.0",
            "0.15.1",
            "v1.0.0",
        ]
        self.assertEqual(self.select(tags), [("v0.18", "phper-v0.18.0", "0.18.0")])

    def test_series_without_docs_are_dropped_not_replaced(self):
        tags = ["phper-v0.15.0", "phper-v0.16.0", "phper-v0.17.0", "phper-v0.18.0"]
        self.assertEqual(
            self.select(tags, without_docs={"phper-v0.16.0", "phper-v0.17.0"}),
            [("v0.18", "phper-v0.18.0", "0.18.0")],
        )

    def test_no_tags(self):
        self.assertEqual(self.select([]), [])
        self.assertEqual(self.select(["phper-v0.17.2"], without_docs={"phper-v0.17.2"}), [])


class RewriteGithubLinksTest(unittest.TestCase):
    def test_rewrites_blob_and_tree(self):
        text = (
            f"[a]({GITHUB}/blob/master/phper/src/values.rs) "
            f"<{GITHUB}/tree/master/examples/hello>"
        )
        self.assertEqual(
            build_site.rewrite_github_links(text, "phper-v0.18.2"),
            f"[a]({GITHUB}/blob/phper-v0.18.2/phper/src/values.rs) "
            f"<{GITHUB}/tree/phper-v0.18.2/examples/hello>",
        )

    def test_leaves_other_links_alone(self):
        text = "\n".join(
            [
                "https://github.com/php/php-src/blob/master/Zend/zend_types.h",
                "https://github.com/phper-framework/phper-fork/blob/master/README.md",
                f"{GITHUB}/edit/master/docs/src/zval.md",
                f"{GITHUB}/blob/main/README.md",
            ]
        )
        self.assertEqual(build_site.rewrite_github_links(text, "phper-v0.18.2"), text)


class HelpersTest(unittest.TestCase):
    def test_normalize_base_url(self):
        self.assertEqual(build_site.normalize_base_url("phper"), "/phper/")
        self.assertEqual(build_site.normalize_base_url("/phper/"), "/phper/")
        self.assertEqual(build_site.normalize_base_url("/"), "/")
        self.assertEqual(build_site.normalize_base_url(""), "/")

    def test_versions_manifest(self):
        versions = [build_site.Version("v0.18", "phper-v0.18.2", "0.18.2")]
        self.assertEqual(
            build_site.versions_manifest(versions),
            {
                "default": "v0.18",
                "versions": [
                    {"name": "v0.18", "title": "v0.18 (0.18.2)", "tag": "phper-v0.18.2"},
                    {"name": "master", "title": "master (unreleased)", "tag": "master"},
                ],
            },
        )
        self.assertEqual(build_site.versions_manifest([])["default"], "master")

    def test_prepare_out_refuses_unrelated_directories(self):
        with tempfile.TemporaryDirectory() as tmp:
            repo = Path(tmp) / "repo"
            repo.mkdir()
            with self.assertRaises(build_site.SiteError):
                build_site.prepare_out(repo, repo)
            with self.assertRaises(build_site.SiteError):
                build_site.prepare_out(Path(tmp), repo)

            out = Path(tmp) / "out"
            out.mkdir()
            (out / "precious.txt").write_text("keep me")
            with self.assertRaises(build_site.SiteError):
                build_site.prepare_out(out, repo)
            self.assertTrue((out / "precious.txt").exists())

            (out / "precious.txt").unlink()
            build_site.prepare_out(out, repo)
            (out / "old.html").write_text("")
            build_site.prepare_out(out, repo)
            self.assertFalse((out / "old.html").exists())


def git(repo, *args):
    subprocess.run(
        [
            "git",
            "-c", "user.name=Test",
            "-c", "user.email=test@example.com",
            "-c", "commit.gpgsign=false",
            "-c", "tag.gpgsign=false",
            "-C", str(repo),
            *args,
        ],
        check=True,
        capture_output=True,
    )


@unittest.skipUnless(mdbook_available(), "mdBook 0.5 or later is not installed")
class BuildTest(unittest.TestCase):
    def setUp(self):
        self._tmp = tempfile.TemporaryDirectory()
        self.tmp = Path(self._tmp.name)
        self.out = self.tmp / "out"

    def tearDown(self):
        self._tmp.cleanup()

    def build(self, repo):
        with contextlib.redirect_stdout(io.StringIO()):
            return build_site.main(
                [
                    "build",
                    "--repo", str(repo),
                    "--out", str(self.out),
                    "--base-url", "phper",
                    "--mdbook", MDBOOK,
                ]
            )

    def write_chapter(self, repo, text):
        (repo / "docs" / "src" / "chapter.md").write_text(
            f"# Chapter\n\n{text}\n\nSee [README]({GITHUB}/blob/master/README.md).\n"
        )

    def test_versions_from_tags(self):
        repo = self.tmp / "repo"
        repo.mkdir()
        git(repo, "init", "-q")
        (repo / "README.md").write_text("# test\n")
        git(repo, "add", ".")
        git(repo, "commit", "-q", "-m", "before docs")
        git(repo, "tag", "phper-v0.0.1")

        (repo / "docs" / "src").mkdir(parents=True)
        (repo / "docs" / "book.toml").write_text('[book]\ntitle = "Test"\nsrc = "src"\n')
        (repo / "docs" / "src" / "SUMMARY.md").write_text("# Summary\n\n- [Chapter](chapter.md)\n")
        self.write_chapter(repo, "Text of 0.1.0.")
        git(repo, "add", ".")
        git(repo, "commit", "-q", "-m", "add docs")
        git(repo, "tag", "phper-v0.1.0")
        self.write_chapter(repo, "Text of 0.1.1.")
        git(repo, "commit", "-q", "-am", "update docs")
        git(repo, "tag", "-a", "-m", "annotated", "phper-v0.1.1")
        # Uncommitted changes show up in master.
        self.write_chapter(repo, "Text of the working tree.")

        self.assertEqual(self.build(repo), 0)

        docs = self.out / "docs"
        master = (docs / "master" / "chapter.html").read_text()
        self.assertIn("Text of the working tree.", master)
        self.assertIn(f"{GITHUB}/blob/master/README.md", master)

        release = (docs / "v0.1" / "chapter.html").read_text()
        self.assertIn("Text of 0.1.1.", release)
        self.assertIn(f"{GITHUB}/blob/phper-v0.1.1/README.md", release)
        self.assertIn('<base href="/phper/docs/v0.1/"', (docs / "v0.1" / "404.html").read_text())
        self.assertFalse((docs / "v0.0").exists())

        manifest = json.loads((docs / "versions.json").read_text())
        self.assertEqual(manifest["default"], "v0.1")
        self.assertEqual(
            [(entry["name"], entry["title"]) for entry in manifest["versions"]],
            [("v0.1", "v0.1 (0.1.1)"), ("master", "master (unreleased)")],
        )
        self.assertIn("url=./v0.1/", (docs / "index.html").read_text())

        home = (self.out / "index.html").read_text()
        self.assertIn('href="docs/"', home)

        # Building again replaces the previous output.
        (self.out / "stale.html").write_text("")
        self.assertEqual(self.build(repo), 0)
        self.assertFalse((self.out / "stale.html").exists())

    def test_this_repository(self):
        self.assertEqual(self.build(build_site.DEFAULT_REPO), 0)
        page = (self.out / "docs" / "master" / "module" / "register-class.html").read_text()
        self.assertIn("version-switcher", page)
        self.assertTrue((self.out / "index.html").is_file())
        manifest = json.loads((self.out / "docs" / "versions.json").read_text())
        self.assertEqual(manifest["versions"][-1]["name"], "master")
        for version in manifest["versions"]:
            self.assertTrue((self.out / "docs" / version["name"] / "index.html").is_file())


if __name__ == "__main__":
    unittest.main()
