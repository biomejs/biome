set -eu

# Regression test for https://github.com/biomejs/biome/issues/10395
#
# `biome format --stdin-file-path` must echo formatted source back
# byte-for-byte, even when it contains single-codepoint non-ASCII
# characters (e.g. U+26A0 WARNING SIGN, U+2714 HEAVY CHECK MARK). These
# used to get silently rewritten to look-alike ASCII/Unicode characters
# (e.g. U+2714 -> U+221A) whenever stdout was not a TTY, because the
# formatted output was routed through the same sanitization pipeline
# used for human-facing diagnostic text.
#
# Piping through `cat` (rather than relying on the script's own stdout)
# ensures stdout is a pipe, not a TTY, which is what triggers the bug.

expected=$(printf 'const a = `\xe2\x9a\xa0`;\nconst b = `\xe2\x9c\x94`;\n')
actual=$(printf 'const a = `\xe2\x9a\xa0`;\nconst b = `\xe2\x9c\x94`;\n' | cargo run --bin biome -- format --stdin-file-path=probe.ts | cat)

if [ "$actual" != "$expected" ]; then
    echo "Formatted stdin output does not match input byte-for-byte:"
    echo "expected: $expected"
    echo "actual:   $actual"
    exit 1
fi
