## Why

Praetor's adoption proved two rendered-output defects by execution: the CI lint job cannot find the pre-commit config, and the rendered releaser carries a trailing blank line that the template's own fixer gate strips, failing pre-commit on template-rendered output.

## What Changes

- Pass `--config .auxiliary/configuration/pre-commit.yaml` in the tester lint job.
- End the releaser template so the render terminates in a single newline.

## Capabilities

### New Capabilities

- None.

### Modified Capabilities

- `quality-gates`: lint job resolves the pre-commit config; rendered workflows are fixer-clean.

## Impact

- Rendered workflow fixes only. No downstream edits.
