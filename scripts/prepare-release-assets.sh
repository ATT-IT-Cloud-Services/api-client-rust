#!/usr/bin/env bash
# Package api-client-rust release assets.
set -euo pipefail

ROOT="${1:?usage: prepare-release-assets.sh <project-dir> <output-dir>}"
OUT="${2:?usage: prepare-release-assets.sh <project-dir> <output-dir>}"

VERSION="$(sed -n 's/^version[[:space:]]*=[[:space:]]*"\([^"]*\)".*/\1/p' "$ROOT/Cargo.toml" | head -n 1 | sed -e 's/^[vV]//')"
PREFIX="ignite-api-client-${VERSION}"

mkdir -p "$OUT"

(
  cd "$ROOT"
  cargo package --allow-dirty --no-verify
)

CRATE="$(find "$ROOT/target/package" -maxdepth 1 -name '*.crate' -type f | head -n 1)"
if [[ -z "$CRATE" ]]; then
  echo "[release] error: cargo package produced no .crate" >&2
  exit 1
fi

CRATE_OUT="$OUT/${PREFIX}.crate"
cp "$CRATE" "$CRATE_OUT"
echo "[release] crate: $(basename "$CRATE_OUT")"

ZIP="$OUT/${PREFIX}-src.zip"
(
  cd "$ROOT"
  zip -qr "$ZIP" src examples Cargo.toml README.md LICENSE.md I18N_POLICY.md
  if [[ -f Cargo.lock ]]; then
    zip -q "$ZIP" Cargo.lock
  fi
)
echo "[release] source zip: $(basename "$ZIP")"

echo "[release] Upload set:"
find "$OUT" -maxdepth 1 -type f -printf '  %f (%s bytes)\n' | sort
