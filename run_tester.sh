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

# ./run_tester.sh                 every section below
# ./run_tester.sh all             same
# ./run_tester.sh navigation      one section
#
# Sections follow the CodeCrafters shell course. The first block has no
# heading on the site; here it is "basics".
# ei0 (The pwd builtin) is left out of navigation: that test uses sudo to
# rename /usr/bin/pwd.
SECTIONS=(basics navigation quoting redirection completion pipelines)

slugs_for() {
  case "$1" in
    basics) echo "oo8 cz2 ff0 pn5 iz3 ez5 mg5 ip1" ;;
    navigation) echo "ra6 gq9 gp4" ;;
    quoting) echo "ni6 tg6 yt5 le5 gu3 qj0" ;;
    redirection) echo "jv1 vz4 el9 un3" ;;
    completion) echo "qp2 gm9 qm8 gy5 wh6 wt6" ;;
    pipelines) echo "br6 ny9 xk3" ;;
  esac
}

title_for() {
  case "$1" in
    oo8) echo "Print a prompt" ;;
    cz2) echo "Handle invalid commands" ;;
    ff0) echo "Implement a REPL" ;;
    pn5) echo "Implement exit" ;;
    iz3) echo "Implement echo" ;;
    ez5) echo "Implement type" ;;
    mg5) echo "Locate executable files" ;;
    ip1) echo "Run a program" ;;
    ra6) echo "The cd builtin: Absolute paths" ;;
    gq9) echo "The cd builtin: Relative paths" ;;
    gp4) echo "The cd builtin: Home directory" ;;
    ni6) echo "Single quotes" ;;
    tg6) echo "Double quotes" ;;
    yt5) echo "Backslash outside quotes" ;;
    le5) echo "Backslash within single quotes" ;;
    gu3) echo "Backslash within double quotes" ;;
    qj0) echo "Executing a quoted executable" ;;
    jv1) echo "Redirect stdout" ;;
    vz4) echo "Redirect stderr" ;;
    el9) echo "Append stdout" ;;
    un3) echo "Append stderr" ;;
    qp2) echo "Builtin completion" ;;
    gm9) echo "Completion with arguments" ;;
    qm8) echo "Missing completions" ;;
    gy5) echo "Executable completion" ;;
    wh6) echo "Multiple completions" ;;
    wt6) echo "Partial completions" ;;
    br6) echo "Dual-command pipeline" ;;
    ny9) echo "Pipelines with built-ins" ;;
    xk3) echo "Multi-command pipelines" ;;
    *) echo "Stage $1" ;;
  esac
}

cases_json() {
  local first=1 slug title json="["
  for slug in "$@"; do
    title=$(title_for "$slug")
    [[ $first -eq 1 ]] || json+=","
    first=0
    json+=$(printf '{"slug":"%s","tester_log_prefix":"tester::#%s","title":"%s"}' "$slug" "$slug" "$title")
  done
  json+="]"
  printf '%s' "$json"
}

normalize_section() {
  local name
  name=$(printf '%s' "$1" | tr '[:upper:]' '[:lower:]')
  case "$name" in
    basics|basic) echo basics ;;
    navigation|navigations) echo navigation ;;
    quoting) echo quoting ;;
    redirection|redirections) echo redirection ;;
    completion|completions|"command completion") echo completion ;;
    pipeline|pipelines) echo pipelines ;;
    *) return 1 ;;
  esac
}

ARG="${1:-all}"
SELECTED=()
if [[ "$ARG" == "all" ]]; then
  SELECTED=("${SECTIONS[@]}")
else
  if ! section=$(normalize_section "$ARG"); then
    echo "Unknown section: $ARG" >&2
    echo "Sections: ${SECTIONS[*]}" >&2
    exit 1
  fi
  SELECTED=("$section")
fi

# Build once here (real $HOME) so your_program.sh can just exec the binary.
# oo8 sets HOME to a random empty dir; running cargo there breaks rustup.
SHELL_BIN="/tmp/codecrafters-build-shell-rust/release/codecrafters-shell"
( cd "$ROOT" && cargo build --release --target-dir=/tmp/codecrafters-build-shell-rust --manifest-path Cargo.toml >/dev/null 2>&1 )

WORK="$ROOT/.tester-run"
rm -rf "$WORK"
mkdir -p "$WORK"
rsync -a --exclude .tester-run "$ROOT/" "$WORK/"
# filter=1 limits PATH to /tmp for the shell under test. The shell that
# invoked this script is unchanged. Other stages keep the real PATH so
# commands such as cat and ls still resolve.
run_cases() {
  local filter="$1"
  shift
  [[ $# -eq 0 ]] && return 0
  if [[ "$filter" == "1" ]]; then
    cat > "$WORK/your_program.sh" << EOF
#!/bin/sh
kept=""
IFS=:
for dir in \$PATH; do
  case "\$dir" in
    /tmp/*) kept="\${kept:+\$kept:}\$dir" ;;
  esac
done
if [ -z "\$kept" ]; then
  kept="\$(mktemp -d)"
fi
export PATH="\$kept"
exec "$SHELL_BIN" "\$@"
EOF
  else
    cat > "$WORK/your_program.sh" << EOF
#!/bin/sh
exec "$SHELL_BIN" "\$@"
EOF
  fi
  chmod +x "$WORK/your_program.sh"
  TESTER_DIR="$TESTER_DIR" \
  CODECRAFTERS_REPOSITORY_DIR="$WORK" \
  CODECRAFTERS_TEST_CASES_JSON="$(cases_json "$@")" \
  "$TESTER_DIR/tester"
}

NORMAL_SLUGS=()
COMPLETION_SLUGS=()
for section in "${SELECTED[@]}"; do
  read -r -a slugs <<< "$(slugs_for "$section")"
  if [[ "$section" == "completion" ]]; then
    COMPLETION_SLUGS+=("${slugs[@]}")
  else
    NORMAL_SLUGS+=("${slugs[@]}")
  fi
done

run_cases 0 "${NORMAL_SLUGS[@]+"${NORMAL_SLUGS[@]}"}"
run_cases 1 "${COMPLETION_SLUGS[@]+"${COMPLETION_SLUGS[@]}"}"
