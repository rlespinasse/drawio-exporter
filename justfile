#!/usr/bin/env -S just --justfile

set quiet := true

default:
    just --choose

# Configure dev dependencies
configure:
    @echo + $@
    rustup component add clippy
    mise install
    cargo install cargo-tarpaulin

# Execute all developments recipes
[group('Development mode')]
loop: fmt clippy-fix idioms-fix fix audit build test

# Build it
[group('Development mode')]
build:
    cargo build

# Test it
[group('Development mode')]
test:
    cargo test

# Format it
[group('Development mode')]
fmt:
    cargo fmt --all

# Check the format
[group('CI/CD mode')]
fmt-check:
    cargo fmt --all -- --check

# Run linter on sources
[group('CI/CD mode')]
clippy:
    cargo clippy --all-features --all-targets

# Apply some lint suggestions from clippy
[group('Development mode')]
clippy-fix:
    cargo clippy --fix --all-features --all-targets --allow-dirty --allow-staged

# Apply rustc's suggestions from diagnostics like warnings and apply them to your source code
[group('Development mode')]
fix:
    cargo fix --allow-dirty --allow-staged

# Apply suggested idioms for the current edition
[group('Development mode')]
idioms-fix:
    cargo fix --edition-idioms --allow-dirty --allow-staged

# Update dependencies
[group('Development mode')]
update:
    cargo update

# Audit the code
[group('Development mode')]
audit:
    cargo audit

# Generate coverage metrics
[group('Development mode')]
coverage:
    cargo tarpaulin --out Html

# Display coverage metrics
[group('Development mode')]
coverage-view:
    [ -f file ] && open tarpaulin-report.html || just coverage

# Analyse dependencies tree
[group('Debug mode')]
deps DEPS="drawio-exporter":
    cargo tree --package {{ DEPS }}

# Update drawio-desktop version used in CI
[group('Maintenance mode')]
autoupdate-drawio-desktop:
    #!/usr/bin/env bash
    set -euo pipefail
    WORKFLOW=.github/workflows/drawio-exporter.yaml
    CURRENT_VERSION=$(sed -n 's/.*drawio-amd64-\([0-9.]*\)\.deb.*/\1/p' "$WORKFLOW" | head -1)
    DRAWIO_DESKTOP_RELEASE=$(gh release list --repo jgraph/drawio-desktop | grep "Latest" | cut -f1)
    if [ "$CURRENT_VERSION" = "$DRAWIO_DESKTOP_RELEASE" ]; then
        echo "Already up to date ($CURRENT_VERSION)"
        exit 0
    fi
    sed -i \
        -e 's/releases\/download\/v[0-9.]*\/drawio-amd64-[0-9.]*\.deb/releases\/download\/v'$DRAWIO_DESKTOP_RELEASE'\/drawio-amd64-'$DRAWIO_DESKTOP_RELEASE'.deb/' \
        -e 's/dpkg -i drawio-amd64-[0-9.]*\.deb/dpkg -i drawio-amd64-'$DRAWIO_DESKTOP_RELEASE'.deb/' \
        "$WORKFLOW"
    if [ -n "${GITHUB_OUTPUT:-}" ]; then
        echo "release_version=$DRAWIO_DESKTOP_RELEASE" >> "${GITHUB_OUTPUT}"

        RELEASE_NOTES=$(gh release view "v$DRAWIO_DESKTOP_RELEASE" --repo jgraph/drawio-desktop --json body -q .body)
        CHANGELOG=$(curl -fsSL "https://raw.githubusercontent.com/jgraph/drawio/v${DRAWIO_DESKTOP_RELEASE}/ChangeLog" || true)
        if [ -n "$CHANGELOG" ] && grep -qE "^[0-9]{2}-[A-Z]{3}-[0-9]{4}: ${CURRENT_VERSION}$" <<< "$CHANGELOG"; then
            CORE_CHANGES=$(awk -v old="$CURRENT_VERSION" '
                /^[0-9]{2}-[A-Z]{3}-[0-9]{4}: / {
                    ver=$0; sub(/^[0-9]{2}-[A-Z]{3}-[0-9]{4}: /, "", ver)
                    if (ver == old) exit
                }
                { print }
            ' <<< "$CHANGELOG")
        else
            CORE_CHANGES="See [draw.io core ChangeLog](https://github.com/jgraph/drawio/blob/v${DRAWIO_DESKTOP_RELEASE}/ChangeLog)."
        fi

        {
            echo "release_notes<<GH_RELEASE_NOTES_EOF"
            echo "Updates \`drawio-desktop\` from \`$CURRENT_VERSION\` to \`$DRAWIO_DESKTOP_RELEASE\`."
            echo
            echo "### drawio-desktop release notes"
            echo
            echo "$RELEASE_NOTES"
            echo
            echo "### draw.io core ChangeLog ($CURRENT_VERSION -> $DRAWIO_DESKTOP_RELEASE)"
            echo
            echo "$CORE_CHANGES"
            echo "GH_RELEASE_NOTES_EOF"
        } >> "${GITHUB_OUTPUT}"
    fi

# Release a new version (Possible values: major, minor, patch, release, rc, beta, alpha)
[group('Release mode')]
release LEVEL="alpha":
    cargo release --execute {{ LEVEL }}

# Dry-run a release
[group('Release mode')]
dryrun-release LEVEL="alpha":
    cargo release {{ LEVEL }}
