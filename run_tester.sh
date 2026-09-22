#!/usr/bin/env bash
# Run the official CodeCrafters shell-tester against this repo locally.
# https://github.com/codecrafters-io/shell-tester
set -euo pipefail

ROOT="$(cd "$(dirname "$0")" && pwd)"
TESTER_DIR="${TESTER_DIR:-/tmp/shell-tester}"
ARCH="$(uname -m)"
case "$ARCH" in
  x86_64) ASSET="v133_linux_amd64.tar.gz" ;;
  aarch64|arm64) ASSET="v133_linux_arm64.tar.gz" ;;
  *) echo "Unsupported arch: $ARCH" >&2; exit 1 ;;
esac

if [[ ! -x "$TESTER_DIR/tester" ]]; then
  echo "Downloading shell-tester..." >&2
  mkdir -p "$TESTER_DIR"
  curl -fsSL -o "$TESTER_DIR/shell-tester.tar.gz" \
    "https://github.com/codecrafters-io/shell-tester/releases/download/v133/$ASSET"
  tar -xzf "$TESTER_DIR/shell-tester.tar.gz" -C "$TESTER_DIR"
fi

# Which stage group to run:
#   base, redirections, completions, all (through redirections), or a slug like el9
# Note: ei0 (PWD) is excluded from `all` — it uses sudo to rename /usr/bin/pwd.
GROUP="${1:-redirections}"

case "$GROUP" in
  base)
    JSON='[{"slug":"oo8","tester_log_prefix":"tester::#oo8","title":"Init"},{"slug":"cz2","tester_log_prefix":"tester::#cz2","title":"Invalid Command"},{"slug":"ff0","tester_log_prefix":"tester::#ff0","title":"REPL"},{"slug":"pn5","tester_log_prefix":"tester::#pn5","title":"Exit"},{"slug":"iz3","tester_log_prefix":"tester::#iz3","title":"Echo"},{"slug":"ez5","tester_log_prefix":"tester::#ez5","title":"Type built-in"},{"slug":"mg5","tester_log_prefix":"tester::#mg5","title":"Type for executables"},{"slug":"ip1","tester_log_prefix":"tester::#ip1","title":"Run a program"}]'
    ;;
  redirections)
    JSON='[{"slug":"jv1","tester_log_prefix":"tester::#jv1","title":"Redirect stdout"},{"slug":"vz4","tester_log_prefix":"tester::#vz4","title":"Redirect stderr"},{"slug":"el9","tester_log_prefix":"tester::#el9","title":"Append stdout"},{"slug":"un3","tester_log_prefix":"tester::#un3","title":"Append stderr"}]'
    ;;
  completions)
    JSON='[{"slug":"qp2","tester_log_prefix":"tester::#qp2","title":"Builtins completion"},{"slug":"gm9","tester_log_prefix":"tester::#gm9","title":"Completion with args"},{"slug":"qm8","tester_log_prefix":"tester::#qm8","title":"Completion with invalid command"},{"slug":"gy5","tester_log_prefix":"tester::#gy5","title":"Valid command completion"},{"slug":"wh6","tester_log_prefix":"tester::#wh6","title":"Completion with multiple executables"},{"slug":"wt6","tester_log_prefix":"tester::#wt6","title":"Partial completions"}]'
    ;;
  all)
    JSON='[{"slug":"oo8","tester_log_prefix":"tester::#oo8","title":"Init"},{"slug":"cz2","tester_log_prefix":"tester::#cz2","title":"Invalid Command"},{"slug":"ff0","tester_log_prefix":"tester::#ff0","title":"REPL"},{"slug":"pn5","tester_log_prefix":"tester::#pn5","title":"Exit"},{"slug":"iz3","tester_log_prefix":"tester::#iz3","title":"Echo"},{"slug":"ez5","tester_log_prefix":"tester::#ez5","title":"Type built-in"},{"slug":"mg5","tester_log_prefix":"tester::#mg5","title":"Type for executables"},{"slug":"ip1","tester_log_prefix":"tester::#ip1","title":"Run a program"},{"slug":"ra6","tester_log_prefix":"tester::#ra6","title":"CD-1"},{"slug":"gq9","tester_log_prefix":"tester::#gq9","title":"CD-2"},{"slug":"gp4","tester_log_prefix":"tester::#gp4","title":"CD-3"},{"slug":"ni6","tester_log_prefix":"tester::#ni6","title":"Quoting with single quotes"},{"slug":"tg6","tester_log_prefix":"tester::#tg6","title":"Quoting with double quotes"},{"slug":"yt5","tester_log_prefix":"tester::#yt5","title":"Quoting with backslashes"},{"slug":"le5","tester_log_prefix":"tester::#le5","title":"Quoting with single and double quotes"},{"slug":"gu3","tester_log_prefix":"tester::#gu3","title":"Quoting with mixed quotes"},{"slug":"qj0","tester_log_prefix":"tester::#qj0","title":"Quoting program names"},{"slug":"jv1","tester_log_prefix":"tester::#jv1","title":"Redirect stdout"},{"slug":"vz4","tester_log_prefix":"tester::#vz4","title":"Redirect stderr"},{"slug":"el9","tester_log_prefix":"tester::#el9","title":"Append stdout"},{"slug":"un3","tester_log_prefix":"tester::#un3","title":"Append stderr"}]'
    ;;
  ei0)
    echo "Skipping ei0 (PWD): requires sudo to rename /usr/bin/pwd. Use codecrafters submit instead." >&2
    exit 1
    ;;
  *)
    JSON="[{\"slug\":\"$GROUP\",\"tester_log_prefix\":\"tester::#$GROUP\",\"title\":\"Stage $GROUP\"}]"
    ;;
esac

# Build once here (real $HOME) so your_program.sh can just exec the binary.
# oo8 sets HOME to a random empty dir; running cargo there breaks rustup.
SHELL_BIN="/tmp/codecrafters-build-shell-rust/release/codecrafters-shell"
( cd "$ROOT" && cargo build --release --target-dir=/tmp/codecrafters-build-shell-rust --manifest-path Cargo.toml >/dev/null 2>&1 )

WORK="$ROOT/.tester-run"
rm -rf "$WORK"
mkdir -p "$WORK"
rsync -a --exclude .tester-run "$ROOT/" "$WORK/"
cat > "$WORK/your_program.sh" << EOF
#!/bin/sh
exec "$SHELL_BIN" "\$@"
EOF
chmod +x "$WORK/your_program.sh"

TESTER_DIR="$TESTER_DIR" \
CODECRAFTERS_REPOSITORY_DIR="$WORK" \
CODECRAFTERS_TEST_CASES_JSON="$JSON" \
"$TESTER_DIR/tester"
