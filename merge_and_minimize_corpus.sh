#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 5 ]]; then
  echo "Usage: $0 <afl-out-dir> <merged-dir> <minimized-dir> <aflpp-path> <nyx-sharedir>"
  exit 1
fi

AFL_OUT_DIR="$1"
MERGED_DIR="$2"
MINIMIZED_DIR="$3"
AFLPP_PATH="$4"
NYX_SHAREDIR="$5"

AFL_CMIN="$AFLPP_PATH/afl-cmin"

if [[ ! -x "$AFL_CMIN" ]]; then
  echo "Error: afl-cmin not found at $AFL_CMIN"
  exit 1
fi

if [[ ! -d "$AFL_OUT_DIR" ]]; then
  echo "Error: AFL output dir does not exist: $AFL_OUT_DIR"
  exit 1
fi

if [[ ! -d "$NYX_SHAREDIR" ]]; then
  echo "Error: Nyx sharedir does not exist: $NYX_SHAREDIR"
  exit 1
fi

rm -rf "$MERGED_DIR" "$MINIMIZED_DIR"
mkdir -p "$MERGED_DIR" "$MINIMIZED_DIR"

echo "[1/3] Collecting queue files from multi-runner AFL output..."
count=0
while IFS= read -r -d '' f; do
  count=$((count + 1))
  cp "$f" "$MERGED_DIR/input-$(printf '%06d' "$count")"
done < <(find "$AFL_OUT_DIR" -type f -path '*/queue/*' ! -name 'README.txt' -print0)

if [[ "$count" -eq 0 ]]; then
  echo "Error: No queue inputs found under $AFL_OUT_DIR"
  exit 1
fi

echo "Collected $count inputs into $MERGED_DIR"

echo "[2/3] Running afl-cmin in Nyx mode..."
"$AFL_CMIN" -X -i "$MERGED_DIR" -o "$MINIMIZED_DIR" -- "$NYX_SHAREDIR"

echo "[3/3] Done"
min_count=$(find "$MINIMIZED_DIR" -maxdepth 1 -type f | wc -l)
echo "Minimized corpus size: $min_count"
echo "Merged corpus:    $MERGED_DIR"
echo "Minimized corpus: $MINIMIZED_DIR"
