# AgenticTime Python SDK

Temporal reasoning for AI agents — deadlines, schedules, sequences, duration estimation (PERT), and decay models.

## Install

```bash
pip install agentic-time
```

## Usage

```python
from agentic_time import TimeGraph

tg = TimeGraph("project.atime")
tg.add_deadline("Ship v1.0", "2026-03-15T17:00:00Z", priority="high")
tg.add_duration_estimate("Auth refactor", hours=8, confidence=0.7)
tg.save()
```

Requires the `atime` CLI binary on PATH. Install via:

```bash
curl -fsSL https://agentralabs.tech/install/time | bash
```
