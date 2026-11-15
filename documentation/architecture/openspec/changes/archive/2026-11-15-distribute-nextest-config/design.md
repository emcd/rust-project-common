## Context

`_skip_if_exists` already protects the linecheck config under the same directory with the same ownership shape (template-seeded, consumer-tuned). The nextest config follows it exactly.

## Goals / Non-Goals

**Goals:**

- One canonical config path; one invocation shape across all nextest call sites.
- Zero behavior change on day one: the default file carries no tuning.

**Non-Goals:**

- No slow-timeout, retries, or profile defaults in the template file — per-repo tuning stays project-owned.
- No doctest changes (`cargo test` does not take a nextest config).

## Decisions

### 1. Static file, skip-if-exists, like linecheck.yaml

Same mechanism, same rationale, one line in the same list. Consumers with a file at that path (cistella, agentmux) keep theirs byte-identical and delete their flag grafts.

### 2. Always pass the flag, everywhere nextest runs

Pre-commit, pre-merge-commit, pre-push release, and CI release-gates entries all carry it. No call site depends on autoload behavior anymore.

### 3. Stale default-path files are the consumer's move

Because `--config-file` replaces rather than augments the autoload path, a `.config/nextest.toml` left behind goes Christmas-dark silently. The proposal calls it out and the announcement names it; no template migration code attempts to move consumer tuning around.

## Risks / Trade-offs

- [A consumer tuned the default autoload path] → Mitigation: announced one-time move to the canonical path. No known consumer does (all three known configs live at the canonical path already).
- [Empty-ish default file confuses] → Mitigation: the header comment states the path convention and points at the reference docs; copiertv asserts the file exists and every entry names it.
