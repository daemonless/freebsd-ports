"""Unit tests for dbuild.config."""

from __future__ import annotations

import unittest
import unittest.mock
from pathlib import Path
from unittest.mock import patch

from dbuild.config import (
    Config,
    _auto_detect_variants,
    _find_config_file,
    _git_remote_org,
    _parse_test_config,
    _parse_variants,
    load,
)


class TestFindConfigFile(unittest.TestCase):
    """Tests for _find_config_file()."""

    def test_no_config(self, tmp_path=None):
        import tempfile
        with tempfile.TemporaryDirectory() as d:
            self.assertIsNone(_find_config_file(Path(d)))

    def test_dbuild_yaml(self):
        import tempfile
        with tempfile.TemporaryDirectory() as d:
            p = Path(d) / ".dbuild.yaml"
            p.write_text("build: {}\n")
            self.assertEqual(_find_config_file(Path(d)), p)

    def test_daemonless_config(self):
        import tempfile
        with tempfile.TemporaryDirectory() as d:
            config_dir = Path(d) / ".daemonless"
            config_dir.mkdir()
            p = config_dir / "config.yaml"
            p.write_text("build: {}\n")
            self.assertEqual(_find_config_file(Path(d)), p)

    def test_dbuild_yaml_preferred(self):
        import tempfile
        with tempfile.TemporaryDirectory() as d:
            p1 = Path(d) / ".dbuild.yaml"
            p1.write_text("build: {}\n")
            config_dir = Path(d) / ".daemonless"
            config_dir.mkdir()
            p2 = config_dir / "config.yaml"
            p2.write_text("build: {}\n")
            self.assertEqual(_find_config_file(Path(d)), p1)


class TestAutoDetectVariants(unittest.TestCase):
    """Tests for _auto_detect_variants()."""

    def test_no_containerfiles(self):
        import tempfile
        with tempfile.TemporaryDirectory() as d:
            result = _auto_detect_variants(Path(d))
            self.assertEqual(result, [])

    def test_only_containerfile(self):
        import tempfile
        with tempfile.TemporaryDirectory() as d:
            (Path(d) / "Containerfile").touch()
            result = _auto_detect_variants(Path(d))
            self.assertEqual(len(result), 1)
            self.assertEqual(result[0].tag, "latest")
            self.assertTrue(result[0].default)

    def test_only_pkg(self):
        import tempfile
        with tempfile.TemporaryDirectory() as d:
            (Path(d) / "Containerfile.pkg").touch()
            result = _auto_detect_variants(Path(d))
            self.assertEqual(len(result), 1)
            self.assertEqual(result[0].tag, "pkg")
            self.assertEqual(result[0].containerfile, "Containerfile.pkg")

    def test_both_containerfiles(self):
        import tempfile
        with tempfile.TemporaryDirectory() as d:
            (Path(d) / "Containerfile").touch()
            (Path(d) / "Containerfile.pkg").touch()
            result = _auto_detect_variants(Path(d))
            self.assertEqual(len(result), 2)
            tags = [v.tag for v in result]
            self.assertEqual(tags, ["latest", "pkg"])

    def test_multiple_suffixes(self):
        import tempfile
        with tempfile.TemporaryDirectory() as d:
            (Path(d) / "Containerfile").touch()
            (Path(d) / "Containerfile.pkg").touch()
            (Path(d) / "Containerfile.dev").touch()
            result = _auto_detect_variants(Path(d))
            self.assertEqual(len(result), 3)
            tags = [v.tag for v in result]
            self.assertEqual(tags, ["latest", "dev", "pkg"])

    def test_no_hardcoded_args(self):
        import tempfile
        with tempfile.TemporaryDirectory() as d:
            (Path(d) / "Containerfile.pkg").touch()
            result = _auto_detect_variants(Path(d))
            self.assertEqual(result[0].args, {})

    def test_pkg_name_propagated(self):
        import tempfile
        with tempfile.TemporaryDirectory() as d:
            (Path(d) / "Containerfile").touch()
            result = _auto_detect_variants(Path(d), pkg_name="myapp")
            self.assertEqual(result[0].pkg_name, "myapp")

    def test_auto_version_propagated(self):
        import tempfile
        with tempfile.TemporaryDirectory() as d:
            (Path(d) / "Containerfile").touch()
            result = _auto_detect_variants(Path(d), auto_version=True)
            self.assertTrue(result[0].auto_version)


class TestParseTestConfig(unittest.TestCase):
    """Tests for _parse_test_config()."""

    def test_no_cit(self):
        self.assertIsNone(_parse_test_config({}))

    def test_empty_cit(self):
        self.assertIsNone(_parse_test_config({"cit": {}}))

    def test_full_cit(self):
        data = {
            "cit": {
                "mode": "health",
                "port": 8080,
                "health": "/api",
                "wait": 60,
                "ready": "started",
                "screenshot_wait": 5,
                "screenshot": "/path/to/screenshot",
                "https": True,
                "compose": True,
                "annotations": ["org.freebsd.jail.allow.mlock=true"],
            }
        }
        result = _parse_test_config(data)
        self.assertIsNotNone(result)
        self.assertEqual(result.mode, "health")
        self.assertEqual(result.port, 8080)
        self.assertEqual(result.health, "/api")
        self.assertEqual(result.wait, 60)
        self.assertEqual(result.ready, "started")
        self.assertEqual(result.screenshot_wait, 5)
        self.assertEqual(result.screenshot_path, "/path/to/screenshot")
        self.assertTrue(result.https)
        self.assertTrue(result.compose)
        self.assertEqual(result.annotations, ["org.freebsd.jail.allow.mlock=true"])

    def test_defaults(self):
        data = {"cit": {"mode": "port"}}
        result = _parse_test_config(data)
        self.assertEqual(result.wait, 120)
        self.assertFalse(result.https)
        self.assertFalse(result.compose)
        self.assertEqual(result.annotations, [])


class TestParseVariants(unittest.TestCase):
    """Tests for _parse_variants()."""

    def test_empty_data(self):
        self.assertEqual(_parse_variants({}), [])

    def test_no_variants(self):
        self.assertEqual(_parse_variants({"build": {}}), [])

    def test_single_variant(self):
        data = {
            "build": {
                "variants": [
                    {"tag": "latest", "containerfile": "Containerfile", "default": True}
                ]
            }
        }
        result = _parse_variants(data)
        self.assertEqual(len(result), 1)
        self.assertEqual(result[0].tag, "latest")
        self.assertTrue(result[0].default)

    def test_variant_with_args(self):
        data = {
            "build": {
                "variants": [
                    {
                        "tag": "pkg",
                        "containerfile": "Containerfile.pkg",
                        "args": {"BASE_VERSION": "15-quarterly"},
                    }
                ]
            }
        }
        result = _parse_variants(data)
        self.assertEqual(result[0].args, {"BASE_VERSION": "15-quarterly"})

    def test_variant_with_aliases(self):
        data = {
            "build": {
                "variants": [
                    {"tag": "latest", "aliases": ["stable", "15"]}
                ]
            }
        }
        result = _parse_variants(data)
        self.assertEqual(result[0].aliases, ["stable", "15"])

    def test_build_auto_version_propagated(self):
        data = {
            "build": {
                "auto_version": True,
                "variants": [
                    {"tag": "latest"},
                    {"tag": "pkg", "auto_version": False},
                ]
            }
        }
        result = _parse_variants(data)
        self.assertTrue(result[0].auto_version)
        self.assertFalse(result[1].auto_version)


class TestLoad(unittest.TestCase):
    """Tests for load()."""

    @patch("dbuild.config._git_remote_org", return_value="myorg")
    def test_auto_detect(self, _mock_org):
        import tempfile
        with tempfile.TemporaryDirectory() as d:
            base = Path(d)
            (base / "Containerfile").touch()
            cfg = load(base)
            self.assertEqual(cfg.image, base.name)
            self.assertEqual(cfg.registry, "ghcr.io/myorg")
            self.assertEqual(len(cfg.variants), 1)
            self.assertEqual(cfg.variants[0].tag, "latest")

    @patch("dbuild.config._git_remote_org", return_value=None)
    def test_registry_fallback_localhost(self, _mock_org):
        import tempfile
        with tempfile.TemporaryDirectory() as d:
            base = Path(d)
            (base / "Containerfile").touch()
            cfg = load(base)
            self.assertEqual(cfg.registry, "localhost")

    @patch.dict("os.environ", {"DBUILD_REGISTRY": "myregistry.io/org"})
    def test_registry_env_override(self):
        import tempfile
        with tempfile.TemporaryDirectory() as d:
            base = Path(d)
            (base / "Containerfile").touch()
            cfg = load(base)
            self.assertEqual(cfg.registry, "myregistry.io/org")

    def test_from_yaml(self):
        import tempfile
        with tempfile.TemporaryDirectory() as d:
            base = Path(d)
            config_dir = base / ".daemonless"
            config_dir.mkdir()
            (config_dir / "config.yaml").write_text(
                "type: base\n"
                "build:\n"
                "  architectures: [amd64, aarch64]\n"
                "  variants:\n"
                "    - tag: latest\n"
                "      containerfile: Containerfile\n"
                "      default: true\n"
                "cit:\n"
                "  mode: health\n"
                "  port: 5432\n"
            )
            cfg = load(base)
            self.assertEqual(cfg.type, "base")
            self.assertEqual(cfg.architectures, ["amd64", "aarch64"])
            self.assertEqual(len(cfg.variants), 1)
            self.assertIsNotNone(cfg.test)
            self.assertEqual(cfg.test.mode, "health")
            self.assertEqual(cfg.test.port, 5432)

    def test_yaml_fallback_to_auto_detect(self):
        """When config exists but has no variants, auto-detect kicks in."""
        import tempfile
        with tempfile.TemporaryDirectory() as d:
            base = Path(d)
            (base / "Containerfile").touch()
            config_dir = base / ".daemonless"
            config_dir.mkdir()
            (config_dir / "config.yaml").write_text("build: {}\n")
            cfg = load(base)
            self.assertEqual(len(cfg.variants), 1)
            self.assertEqual(cfg.variants[0].tag, "latest")

    @patch("dbuild.config.yaml", None)
    def test_no_yaml_module_warns(self):
        """When PyYAML is missing, falls back to auto-detect with warning."""
        import tempfile
        with tempfile.TemporaryDirectory() as d:
            base = Path(d)
            (base / "Containerfile").touch()
            config_dir = base / ".daemonless"
            config_dir.mkdir()
            (config_dir / "config.yaml").write_text("build: {}\n")
            cfg = load(base)
            self.assertEqual(len(cfg.variants), 1)

    def test_full_image_property(self):
        cfg = Config(image="radarr", registry="ghcr.io/daemonless")
        self.assertEqual(cfg.full_image, "ghcr.io/daemonless/radarr")


class TestGitRemoteOrg(unittest.TestCase):
    """Tests for _git_remote_org()."""

    @patch("subprocess.run")
    def test_https_url(self, mock_run):
        mock_run.return_value = unittest.mock.MagicMock(
            returncode=0, stdout="https://github.com/myorg/myrepo.git\n"
        )
        self.assertEqual(_git_remote_org(), "myorg")

    @patch("subprocess.run")
    def test_ssh_url(self, mock_run):
        mock_run.return_value = unittest.mock.MagicMock(
            returncode=0, stdout="git@github.com:myorg/myrepo.git\n"
        )
        self.assertEqual(_git_remote_org(), "myorg")

    @patch("subprocess.run")
    def test_no_remote(self, mock_run):
        mock_run.return_value = unittest.mock.MagicMock(
            returncode=1, stdout=""
        )
        self.assertIsNone(_git_remote_org())

    @patch("subprocess.run", side_effect=FileNotFoundError)
    def test_no_git(self, _mock_run):
        self.assertIsNone(_git_remote_org())


if __name__ == "__main__":
    unittest.main()
