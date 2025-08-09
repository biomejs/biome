#!/bin/bash

# Comprehensive script to promote and rename nursery rules to stable groups
# This handles all the complex file moves, renames, and updates needed

set -e

echo "Starting rule promotion and renaming process..."

# Array of rules to rename: "old_name:new_name:group"
declare -a RULES=(
    "noAwaitInLoop:noAwaitInLoops:performance"
    "noConstantBinaryExpression:noConstantBinaryExpressions:suspicious"
    "noDestructuredProps:noSolidDestructuredProps:correctness"
    "noImplicitCoercion:noImplicitCoercions:complexity"
    "noReactPropAssign:noReactPropAssignments:correctness"
    "noUnknownAtRule:noUnknownAtRules:suspicious"
    "noUselessBackrefInRegex:noUselessRegexBackrefs:suspicious"
    "noUselessEscapeInString:noUselessStringEscapes:suspicious"
    "useAdjacentGetterSetter:useGroupedAccessorPairs:style"
    "useConsistentObjectDefinition:useConsistentObjectDefinitions:style"
    "useConsistentResponse:useStaticResponseMethods:suspicious"
    "useForComponent:useSolidForComponent:performance"
    "useIterableCallbackReturn:useConsistentIterableCallbackReturnValues:suspicious"
    "useJsonImportAttribute:useJsonImportAttributes:correctness"
    "useNamedOperation:useNamedGraphqlOperations:correctness"
    "useUnifiedTypeSignature:useUnifiedTypeSignatures:style"
)

# Function to convert camelCase to snake_case
camel_to_snake() {
    echo "$1" | sed 's/\([A-Z]\)/_\1/g' | tr '[:upper:]' '[:lower:]' | sed 's/^_//'
}

# Function to convert snake_case to CamelCase
snake_to_camel() {
    echo "$1" | awk -F'_' '{for(i=1;i<=NF;i++) $i=toupper(substr($i,1,1)) tolower(substr($i,2))}1' | sed 's/ //g'
}

# Process each rule
for rule in "${RULES[@]}"; do
    IFS=':' read -r old_name new_name group <<< "$rule"
    
    echo "Processing rule: $old_name -> $new_name (group: $group)"
    
    old_snake=$(camel_to_snake "$old_name")
    new_snake=$(camel_to_snake "$new_name")
    old_camel=$(echo "${old_name:0:1}" | tr '[:lower:]' '[:upper:]')${old_name:1}
    new_camel=$(echo "${new_name:0:1}" | tr '[:lower:]' '[:upper:]')${new_name:1}
    
    echo "  Names: $old_name/$old_snake/$old_camel -> $new_name/$new_snake/$new_camel"
    
    # Define paths
    OLD_RULE_PATH="crates/biome_js_analyze/src/lint/nursery/${old_snake}.rs"
    NEW_RULE_PATH="crates/biome_js_analyze/src/lint/${group}/${new_snake}.rs"
    OLD_TEST_DIR="crates/biome_js_analyze/tests/specs/nursery/${old_name}"
    NEW_TEST_DIR="crates/biome_js_analyze/tests/specs/${group}/${new_name}"
    OLD_OPTIONS_PATH="crates/biome_rule_options/src/${old_snake}.rs"
    NEW_OPTIONS_PATH="crates/biome_rule_options/src/${new_snake}.rs"
    
    echo "  Moving rule file: $OLD_RULE_PATH -> $NEW_RULE_PATH"
    
    # Create target directory if it doesn't exist
    mkdir -p "crates/biome_js_analyze/src/lint/${group}"
    
    # Move and update the rule file
    if [ -f "$OLD_RULE_PATH" ]; then
        # Copy the file (we'll update content in place)
        cp "$OLD_RULE_PATH" "$NEW_RULE_PATH"
        
        # Update the rule file contents
        echo "  Updating rule file contents..."
        
        # Update the struct name in declare_lint_rule!
        sed -i.bak "s/pub ${old_camel} {/pub ${new_camel} {/g" "$NEW_RULE_PATH"
        
        # Update the rule name
        sed -i.bak "s/name: \"${old_name}\"/name: \"${new_name}\"/g" "$NEW_RULE_PATH"
        
        # Update any options imports if they exist
        sed -i.bak "s/use biome_rule_options::${old_snake}::/use biome_rule_options::${new_snake}::/g" "$NEW_RULE_PATH"
        sed -i.bak "s/${old_camel}Options/${new_camel}Options/g" "$NEW_RULE_PATH"
        
        # Clean up backup files
        rm -f "${NEW_RULE_PATH}.bak"
        
        # Remove old file
        rm "$OLD_RULE_PATH"
        echo "  Rule file moved and updated successfully"
    else
        echo "  WARNING: Rule file not found: $OLD_RULE_PATH"
        continue
    fi
    
    # Move and update test directory
    echo "  Moving test directory: $OLD_TEST_DIR -> $NEW_TEST_DIR"
    if [ -d "$OLD_TEST_DIR" ]; then
        mkdir -p "crates/biome_js_analyze/tests/specs/${group}"
        mv "$OLD_TEST_DIR" "$NEW_TEST_DIR"
        echo "  Test directory moved successfully"
    else
        echo "  WARNING: Test directory not found: $OLD_TEST_DIR"
    fi
    
    # Move and update rule options file if it exists
    if [ -f "$OLD_OPTIONS_PATH" ]; then
        echo "  Moving options file: $OLD_OPTIONS_PATH -> $NEW_OPTIONS_PATH"
        cp "$OLD_OPTIONS_PATH" "$NEW_OPTIONS_PATH"
        
        # Update struct name in options file
        sed -i.bak "s/pub struct ${old_camel}Options/pub struct ${new_camel}Options/g" "$NEW_OPTIONS_PATH"
        
        # Clean up backup files
        rm -f "${NEW_OPTIONS_PATH}.bak"
        
        # Remove old file
        rm "$OLD_OPTIONS_PATH"
        echo "  Options file moved and updated successfully"
    fi
    
    echo "  Rule $old_name -> $new_name completed"
    echo ""
done

echo "All rules processed. Now updating diagnostic categories..."

# Update diagnostic categories
CATEGORIES_FILE="crates/biome_diagnostics_categories/src/categories.rs"

for rule in "${RULES[@]}"; do
    IFS=':' read -r old_name new_name group <<< "$rule"
    
    echo "Updating diagnostic category: lint/nursery/$old_name -> lint/$group/$new_name"
    
    # Update the category mapping
    sed -i.bak "s/\"lint\/nursery\/${old_name}\":/\"lint\/${group}\/${new_name}\":/g" "$CATEGORIES_FILE"
    
    # Update the URL mapping if it exists
    sed -i.bak "s/rules\/${old_name/rules\/${new_name/g" "$CATEGORIES_FILE"
done

# Clean up backup file
rm -f "${CATEGORIES_FILE}.bak"

echo "Diagnostic categories updated successfully"

# Update rule options mod.rs if it exists
OPTIONS_MOD_FILE="crates/biome_rule_options/src/lib.rs"
if [ -f "$OPTIONS_MOD_FILE" ]; then
    echo "Updating rule options module declarations..."
    
    for rule in "${RULES[@]}"; do
        IFS=':' read -r old_name new_name group <<< "$rule"
        old_snake=$(camel_to_snake "$old_name")
        new_snake=$(camel_to_snake "$new_name")
        
        # Update module declarations
        sed -i.bak "s/pub mod ${old_snake};/pub mod ${new_snake};/g" "$OPTIONS_MOD_FILE"
    done
    
    # Clean up backup file
    rm -f "${OPTIONS_MOD_FILE}.bak"
    echo "Rule options module declarations updated"
fi

echo ""
echo "=== PROMOTION AND RENAMING COMPLETE ==="
echo "Next steps:"
echo "1. Run 'just gen-all' to regenerate analyzer code"
echo "2. Run 'just test' to update test snapshots and verify everything works"
echo "3. Check git status to see all changes"
echo ""
echo "Rules that were promoted and renamed:"
for rule in "${RULES[@]}"; do
    IFS=':' read -r old_name new_name group <<< "$rule"
    echo "  - $old_name -> $group/$new_name"
done