set -eu

# Skip if wasm32-wasip2 target is not installed (CI may not have it)
if ! rustup target list --installed | grep -q wasm32-wasip2; then
    echo "SKIP: wasm32-wasip2 target not installed"
    exit 0
fi

# Build WASM plugins
cargo build --manifest-path "plugins/boolean-naming/Cargo.toml" --target wasm32-wasip2 --release
cargo build --manifest-path "plugins/css-style-conventions/Cargo.toml" --target wasm32-wasip2 --release
cargo build --manifest-path "plugins/json-naming/Cargo.toml" --target wasm32-wasip2 --release

# Run biome lint on JS and capture output
js_output=$(cargo run --bin biome --features wasm_plugin -- lint demo.js 2>&1) || true

# Verify the booleanNaming rule fires (check diagnostic message and header)
echo "$js_output" | grep -q "Boolean variable"
echo "OK: booleanNaming diagnostics found"
echo "$js_output" | grep -q "plugin/booleanNaming"
echo "OK: booleanNaming rule name in header"

# Run biome lint on CSS and capture output
css_output=$(cargo run --bin biome --features wasm_plugin -- lint demo.css 2>&1) || true

# Verify CSS rules fire (check diagnostic messages and headers)
echo "$css_output" | grep -q "does not match pattern"
echo "OK: customPropertyPattern diagnostics found"
echo "$css_output" | grep -q "plugin/customPropertyPattern"
echo "OK: customPropertyPattern rule name in header"
echo "$css_output" | grep -q "Avoid using.*!important.*on custom property"
echo "OK: noImportantInCustomProperties diagnostics found"
echo "$css_output" | grep -q "plugin/noImportantInCustomProperties"
echo "OK: noImportantInCustomProperties rule name in header"

# Run biome lint on JSON and capture output
json_output=$(cargo run --bin biome --features wasm_plugin -- lint demo.json 2>&1) || true

# Verify JSON rule fires (check diagnostic message and header)
echo "$json_output" | grep -q "does not follow camelCase convention"
echo "OK: keyNamingConvention diagnostics found"
echo "$json_output" | grep -q "plugin/keyNamingConvention"
echo "OK: keyNamingConvention rule name in header"
