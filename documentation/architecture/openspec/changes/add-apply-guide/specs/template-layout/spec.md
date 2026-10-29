## ADDED Requirements

### Requirement: Copy and update banners are shown

After a successful copy or update, Copier MUST print a banner pointing at the template application guide, so the reconcile discipline is surfaced at the moment of use. Quiet runs are excepted.

#### Scenario: Copy finishes

- **WHEN** a consumer copies the template and the run is not quiet
- **THEN** the banner names the application guide with its repository URL

#### Scenario: Update finishes

- **WHEN** a consumer updates the template and the run is not quiet
- **THEN** the banner names the application guide with its repository URL
