#!/bin/sh
set -eu

base='4deda08abeffdc188c269360229cf47e12a77a9f'
base_tree='8a2bd1c856a33135fd9738e632a27c4d9b9af6ef'
verus_sha256='59659856b1cc28264a8d21dc11707feb435f801ff386949d60c7ab5a995e7365'
disabled_push='disabled://quick-xml-local-only'

if [ "$#" -ne 1 ]; then
    printf 'usage: %s /path/to/verus\n' "$0" >&2
    exit 2
fi

verus_bin=$1
script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
repo_root=$(git -C "$script_dir" rev-parse --show-toplevel)
scratch=$(mktemp -d)
trap 'rm -r -- "$scratch"' EXIT

test "$(git -C "$repo_root" show -s --format=%P HEAD)" = "$base"
test "$(git -C "$repo_root" rev-list --count "$base"..HEAD)" = '1'
test "$(git -C "$repo_root" rev-parse "$base^{tree}")" = "$base_tree"
test -z "$(git -C "$repo_root" status --porcelain=v1)"
git -C "$repo_root" diff --check "$base" HEAD
git -C "$repo_root" diff --quiet "$base" HEAD -- . \
    ':(exclude)verification/quick-xml-single-quote-boundary-verifier'
test "$(git -C "$repo_root" rev-parse "$base:src/parser/element.rs")" = \
    'db99111f500b04177ef8dfdbb3ca8b0ca0c67832'
test "$(git -C "$repo_root" rev-parse "$base:src/reader/mod.rs")" = \
    'c359c5890870c324953385ddc8f5e0bd0e14f553'
test "$(git -C "$repo_root" rev-parse "$base:src/reader/slice_reader.rs")" = \
    'ffc8708e26b30890ac07ffe338d40fdfcbf4bbd9'
test "$(git -C "$repo_root" rev-parse "$base:src/reader/state.rs")" = \
    'a0ee3b963394bd6f0ebd8a2e3f8904a72f9ee8d2'

(cd "$script_dir" && sha256sum -c reviewed-source.sha256)
test -x "$verus_bin"
test "$(sha256sum "$verus_bin" | awk '{print $1}')" = "$verus_sha256"

layout="$scratch/layout"
mkdir -p "$layout"
cp -R "$script_dir/search" "$layout/counterexample-search"
git clone --quiet --no-checkout --origin origin \
    -c "remote.origin.pushurl=$disabled_push" "$repo_root" "$layout/checkout"
git -C "$layout/checkout" checkout --quiet --detach "$base"

target_dir=${QUICK_XML_BOUNDARY_CARGO_TARGET_DIR:-$scratch/cargo-target}
export CARGO_TARGET_DIR="$target_dir"
search_dir="$layout/counterexample-search"

(cd "$search_dir" && cargo fmt --check)
(cd "$search_dir" && cargo run --quiet --locked --release) \
    >"$scratch/search.txt"
grep -Fx 'status=no-counterexample' "$scratch/search.txt"
grep -Fx 'complete_cases=55987' "$scratch/search.txt"
grep -Fx 'embedded_gt_cases=36456' "$scratch/search.txt"
grep -Fx 'unclosed_single_quote_cases=55987' "$scratch/search.txt"
grep -Fx 'unclosed_tag_cases=55987' "$scratch/search.txt"
(cd "$search_dir" && cargo test --quiet --locked)
(cd "$search_dir" && cargo clippy --quiet --locked --no-deps --all-targets \
    -- -D warnings)

mkdir "$scratch/proof"
cp "$script_dir/proof/single_quote_boundary.rs" "$scratch/proof/"
(cd "$scratch/proof" && "$verus_bin" --no-cheating --compile \
    single_quote_boundary.rs >verus.txt && ./single_quote_boundary)
grep -Fx 'verification results:: 11 verified, 0 errors' \
    "$scratch/proof/verus.txt"
if grep -Eq '\b(assume|admit|external_body|assume_specification)\b' \
    "$script_dir/proof/single_quote_boundary.rs"; then
    exit 1
fi

printf '%s\n' 'checks=pass'
