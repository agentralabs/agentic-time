"""AgenticTime — Temporal reasoning for AI agents.

Pure-Python SDK that wraps the ``atime`` CLI binary via subprocess.
Zero required dependencies; only stdlib: subprocess, json, pathlib, dataclasses.
"""

from __future__ import annotations

import json
import logging
import subprocess
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any, Optional

__version__ = "0.1.0"

logger = logging.getLogger(__name__)


class TimeError(Exception):
    """Raised when an atime CLI command fails."""


@dataclass
class TimeGraph:
    """Interface to an ``.atime`` temporal graph file.

    Parameters
    ----------
    path : str | Path
        Path to the ``.atime`` file. Created automatically on first write
        if it does not exist.
    binary : str
        Name or path of the ``atime`` CLI binary.
    """

    path: str | Path
    binary: str = "atime"
    _resolved_binary: Optional[str] = field(default=None, repr=False, init=False)

    def __post_init__(self) -> None:
        self.path = Path(self.path)

    # ------------------------------------------------------------------
    # Internal helpers
    # ------------------------------------------------------------------

    def _find_binary(self) -> str:
        if self._resolved_binary is not None:
            return self._resolved_binary

        import shutil

        found = shutil.which(self.binary)
        if found is None:
            raise TimeError(
                f"Cannot find '{self.binary}' on PATH. "
                "Install AgenticTime: curl -fsSL https://agentralabs.tech/install/time | bash"
            )
        self._resolved_binary = found
        return found

    def _run(self, *args: str, check: bool = True) -> str:
        """Execute an atime CLI command and return stdout."""
        cmd = [self._find_binary(), "--file", str(self.path), *args]
        logger.debug("Running: %s", " ".join(cmd))
        result = subprocess.run(cmd, capture_output=True, text=True)
        if check and result.returncode != 0:
            raise TimeError(
                f"atime command failed (exit {result.returncode}): {result.stderr.strip()}"
            )
        return result.stdout.strip()

    def _run_json(self, *args: str) -> Any:
        """Execute a command and parse JSON output."""
        raw = self._run(*args, "--format", "json")
        return json.loads(raw) if raw else {}

    # ------------------------------------------------------------------
    # Deadline operations
    # ------------------------------------------------------------------

    def add_deadline(
        self,
        label: str,
        due_at: str,
        *,
        priority: str = "medium",
        consequence: Optional[str] = None,
    ) -> str:
        """Add a deadline. Returns the deadline ID."""
        args = ["deadline", "add", label, due_at, "--priority", priority]
        if consequence:
            args.extend(["--consequence", consequence])
        return self._run(*args)

    def list_deadlines(self, *, status: Optional[str] = None) -> list[dict[str, Any]]:
        """List deadlines, optionally filtered by status."""
        args = ["deadline", "list", "--format", "json"]
        if status:
            args.extend(["--status", status])
        raw = self._run(*args)
        return json.loads(raw) if raw else []

    # ------------------------------------------------------------------
    # Schedule operations
    # ------------------------------------------------------------------

    def add_schedule(
        self,
        label: str,
        start_at: str,
        duration_minutes: int,
        *,
        priority: str = "medium",
    ) -> str:
        """Add a scheduled event. Returns the schedule ID."""
        return self._run(
            "schedule",
            "add",
            label,
            start_at,
            str(duration_minutes),
            "--priority",
            priority,
        )

    # ------------------------------------------------------------------
    # Duration estimation
    # ------------------------------------------------------------------

    def add_duration_estimate(
        self,
        label: str,
        *,
        hours: float = 0,
        confidence: float = 0.7,
    ) -> str:
        """Add a duration estimate. Returns the estimate ID."""
        minutes = int(hours * 60) if hours else 60
        return self._run(
            "duration",
            "estimate",
            label,
            str(minutes),
            "--confidence",
            str(confidence),
        )

    # ------------------------------------------------------------------
    # Stats
    # ------------------------------------------------------------------

    def stats(self) -> dict[str, Any]:
        """Get temporal statistics."""
        raw = self._run("stats", "--format", "json")
        return json.loads(raw) if raw else {}

    # ------------------------------------------------------------------
    # File operations
    # ------------------------------------------------------------------

    def save(self) -> None:
        """Explicit save (most operations auto-save)."""
        # CLI auto-saves; this is a no-op for API compatibility
        pass

    @property
    def exists(self) -> bool:
        """Whether the .atime file exists on disk."""
        return self.path.exists()
