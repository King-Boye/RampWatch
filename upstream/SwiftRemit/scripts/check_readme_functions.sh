#!/usr/bin/env bash
# check_readme_functions.sh — Fail if a pub fn in the contract's #[contractimpl]
# block is missing from the README's "Contract Functions" section.
#
# Usage:
#   ./scripts/check_readme_functions.sh
#
# Extracts every `pub fn <name>` between the `#[contractimpl]` marker and the
# end of src/lib.rs (the whole file is a single impl block today), then
# checks each name appears at least once as `` `name( `` inside the
# "## Contract Functions" section of README.md.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
LIB_RS="${REPO_ROOT}/src/lib.rs"
README="${REPO_ROOT}/README.md"
ABI_DIR="${REPO_ROOT}/abi"

if [[ ! -f "${LIB_RS}" ]]; then
  echo "❌  ${LIB_RS} not found" >&2
  exit 1
fi

if [[ ! -f "${README}" ]]; then
  echo "❌  ${README} not found" >&2
  exit 1
fi

# Locate the ABI file: prefer abi/swiftremit-v1.json, else the first
# abi/*.json that looks like a Soroban contract spec.
ABI_FILE=""
if [[ -f "${ABI_DIR}/swiftremit-v1.json" ]]; then
  ABI_FILE="${ABI_DIR}/swiftremit-v1.json"
elif [[ -d "${ABI_DIR}" ]]; then
  ABI_FILE="$(find "${ABI_DIR}" -maxdepth 1 -name '*.json' -type f | sort | head -1)"
fi

if [[ -z "${ABI_FILE}" || ! -f "${ABI_FILE}" ]]; then
  echo "❌  Could not locate an ABI JSON file under ${ABI_DIR}" >&2
  exit 1
fi

if ! command -v jq >/dev/null 2>&1; then
  echo "❌  jq is required to compare README signatures against the ABI" >&2
  exit 1
fi

IMPL_LINE=$(grep -n "^impl SwiftRemitContract" "${LIB_RS}" | head -1 | cut -d: -f1)
if [[ -z "${IMPL_LINE}" ]]; then
  echo "❌  Could not locate 'impl SwiftRemitContract' in src/lib.rs" >&2
  exit 1
fi

# Isolate the README's "## Contract Functions" section (up to the next "## " heading).
README_SECTION=$(awk '
  /^## Contract Functions/ { capture=1 }
  capture && /^## / && !/^## Contract Functions/ { exit }
  capture { print }
' "${README}")

if [[ -z "${README_SECTION}" ]]; then
  echo "❌  Could not find a '## Contract Functions' section in README.md" >&2
  exit 1
fi

missing=()
while IFS=: read -r _ name; do
  name="$(echo "${name}" | sed -E 's/^ *pub fn *//; s/[(<].*//')"
  if ! grep -qF "\`${name}(" <<<"${README_SECTION}"; then
    missing+=("${name}")
  fi
done < <(tail -n +"${IMPL_LINE}" "${LIB_RS}" | grep -n "    pub fn ")

if [[ ${#missing[@]} -gt 0 ]]; then
  echo "❌  README.md's Contract Functions section is missing ${#missing[@]} function(s):" >&2
  for name in "${missing[@]}"; do
    echo "   - ${name}" >&2
  done
  echo "" >&2
  echo "   Add an entry (with description + auth) under '## Contract Functions' in README.md." >&2
  exit 1
fi

# --- Signature accuracy check ---------------------------------------------
# For each function in the ABI, build the expected parameter list
# (name: type, ...) and verify the README section contains a matching
# signature. This catches stale docs such as create_remittance missing
# the `integrator` parameter.

# Map ABI type descriptors to short Rust-ish type names used in the README.
abi_type_name() {
  local json="$1"
  local type="$2"
  case "${type}" in
    address|i128|u32|u64|i64|u128|bool|symbol|string|bytes|void|u32)
      echo "${type}"
      ;;
    *)
      # Fall back to the raw type string for anything unexpected.
      echo "${type}"
      ;;
  esac
}

mismatches=()
while IFS=$'\t' read -r fn_name params; do
  [[ -z "${fn_name}" ]] && continue

  # Build "name: type, name: type" from the ABI params.
  expected=""
  if [[ -n "${params}" && "${params}" != "null" ]]; then
    while IFS=$'\t' read -r p_name p_type; do
      [[ -z "${p_name}" ]] && continue
      if [[ -n "${expected}" ]]; then
        expected+=", "
      fi
      expected+="${p_name}: ${p_type}"
    done < <(jq -r '.[] | [.name, .type] | @tsv' <<<"${params}")
  fi

  # Extract the README signature line for this function, if present.
  readme_sig="$(grep -oE "\`${fn_name}\([^)]*\)\`" <<<"${README_SECTION}" | head -1 || true)"
  if [[ -z "${readme_sig}" ]]; then
    # Missing signature is already reported by the documentation check above.
    continue
  fi

  # Normalise whitespace inside the README signature for comparison.
  readme_params="$(sed -E 's/^`[^(]*\(//; s/\)`$//' <<<"${readme_sig}" | sed -E 's/[[:space:]]+/ /g; s/^ //; s/ $//')"
  expected_norm="$(sed -E 's/[[:space:]]+/ /g; s/^ //; s/ $//' <<<"${expected}")"

  if [[ "${readme_params}" != "${expected_norm}" ]]; then
    mismatches+=("${fn_name}: README has (${readme_params}) but ABI has (${expected_norm})")
  fi
done < <(jq -r '
  .functions[]
  | select(.name != null)
  | [.name, (.inputs // [])] | @tsv
' "${ABI_FILE}" 2>/dev/null | while IFS=$'\t' read -r name inputs; do
  # Re-emit inputs as JSON so the outer loop can parse them with jq.
  printf '%s\t%s\n' "${name}" "$(jq -c '.' <<<"${inputs}")"
done)

if [[ ${#mismatches[@]} -gt 0 ]]; then
  echo "❌  README.md signatures diverge from ${ABI_FILE}:" >&2
  for m in "${mismatches[@]}"; do
    echo "   - ${m}" >&2
  done
  echo "" >&2
  echo "   Update the '## Contract Functions' signatures in README.md to match the ABI." >&2
  exit 1
fi

echo "✅  README.md documents every pub fn in the SwiftRemitContract impl block and signatures match ${ABI_FILE}."
