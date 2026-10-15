set -euo pipefail
case "$EVENT_NAME" in
  pull_request) base="$PR_BASE" ;;
  push) base="$PUSH_BEFORE" ;;
  *) echo "::error::unsupported semantic base event"; exit 2 ;;
esac
[[ "$base" =~ ^[0-9a-f]{40}$ && ! "$base" =~ ^0+$ ]] || {
  echo "::error::missing real semantic comparison base"; exit 2;
}
git cat-file -e "$base^{commit}"
git merge-base --is-ancestor "$base" HEAD
echo "revision=$base" >> "$GITHUB_OUTPUT"
