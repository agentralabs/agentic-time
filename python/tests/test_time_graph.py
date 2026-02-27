"""Tests for the AgenticTime Python SDK."""

from __future__ import annotations

import pytest

from agentic_time import TimeGraph, TimeError, __version__


class TestPackageMetadata:
    """Basic package import and metadata tests."""

    def test_version_exists(self) -> None:
        assert __version__ is not None
        assert isinstance(__version__, str)
        assert len(__version__) > 0

    def test_version_semver(self) -> None:
        parts = __version__.split(".")
        assert len(parts) == 3
        assert all(p.isdigit() for p in parts)

    def test_import_time_graph(self) -> None:
        assert TimeGraph is not None

    def test_import_time_error(self) -> None:
        assert TimeError is not None
        assert issubclass(TimeError, Exception)


class TestTimeGraphInit:
    """TimeGraph initialization tests."""

    def test_create_with_string_path(self, tmp_path: pytest.TempPathFactory) -> None:
        path = str(tmp_path / "test.atime")
        tg = TimeGraph(path)
        assert str(tg.path) == path

    def test_create_with_path_object(self, tmp_path: pytest.TempPathFactory) -> None:
        from pathlib import Path

        path = tmp_path / "test.atime"
        tg = TimeGraph(path)
        assert tg.path == path

    def test_custom_binary_name(self, tmp_path: pytest.TempPathFactory) -> None:
        tg = TimeGraph(str(tmp_path / "test.atime"), binary="custom-atime")
        assert tg.binary == "custom-atime"

    def test_exists_false_for_new_file(self, tmp_path: pytest.TempPathFactory) -> None:
        tg = TimeGraph(str(tmp_path / "nonexistent.atime"))
        assert not tg.exists

    def test_save_is_noop(self, tmp_path: pytest.TempPathFactory) -> None:
        tg = TimeGraph(str(tmp_path / "test.atime"))
        # save() should not raise
        tg.save()


class TestTimeGraphBinaryResolution:
    """Tests for binary resolution behavior."""

    def test_missing_binary_raises(self, tmp_path: pytest.TempPathFactory) -> None:
        tg = TimeGraph(str(tmp_path / "test.atime"), binary="nonexistent-binary-xyz")
        with pytest.raises(TimeError, match="Cannot find"):
            tg._find_binary()

    def test_find_binary_caches_result(self, tmp_path: pytest.TempPathFactory) -> None:
        tg = TimeGraph(str(tmp_path / "test.atime"))
        tg._resolved_binary = "/fake/path/atime"
        assert tg._find_binary() == "/fake/path/atime"
