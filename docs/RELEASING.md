# Release process and versioning

`Cargo.toml` → `[package].version` is the application version source of truth. Windows packaging and both release workflows read it directly. `packaging/build-deb.sh` derives the Debian version from it.

## Version formats

For application version `0.2.0-alpha.7`:

| Purpose | Value | Reason |
| --- | --- | --- |
| Cargo, application, release tag, Windows ZIP | `0.2.0-alpha.7` (tag prefixed with `v`) | Semantic version of the application |
| Debian package metadata | `0.2.0~alpha.7-1` | The tilde sorts a prerelease before `0.2.0`; `-1` is the package revision |
| Debian download filename | `0.2.0.alpha.7-1` | The packaging script replaces the tilde with a dot to keep the GitHub asset name stable |

The Debian filename is not the package metadata version. Do not replace the tilde in the package metadata with a dot. These derived forms are intentional, not independent versions to bump.

## Preparing a release

1. Update the package version in `Cargo.toml` and the root package entry in `Cargo.lock` (for example with `cargo check`), then update the private web manifest and lockfile to the same version.
2. Move the relevant `Unreleased` changelog entries into a dated release section. Preserve earlier entries. Update `packaging/RELEASE.md`, which supplies the current GitHub release notes.
3. Update version labels and download filenames in the root READMEs, current Markdown/HTML user guides, `docs/WINDOWS.md`, `docs/VALIDATION.md` and the examples on this page. Historical changelog entries and already-published binaries/PDFs are snapshots and must not be silently relabeled.
4. Run `python3 scripts/check-version.py`. CI checks manifest and current text-document consistency. Review illustrated PDF guides separately if the instructions or screenshots changed.
5. Run the regression procedure in [TESTING.md](TESTING.md) and record the evidence in [VALIDATION.md](VALIDATION.md). Require successful Linux, Debian and Windows pipelines for the same commit.
6. The release workflow derives asset names from Cargo and publishes only a version that does not already exist. The metadata workflow can add PDF guides and refresh notes/checksums without replacing binaries.

## Private vulnerability reports

Private vulnerability reporting is enabled for this public repository. Keep it enabled under **Settings → Advanced Security → Private vulnerability reporting**, and retain the working report link in [SECURITY.md](../SECURITY.md). Verify **Security → Advisories → Report a vulnerability** after repository configuration changes. Never use a GitHub `noreply` address for reports.

GitHub documentation: https://docs.github.com/en/code-security/how-tos/report-and-fix-vulnerabilities/configure-vulnerability-reporting/configure-for-a-repository
