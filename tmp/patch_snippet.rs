        // Check if it's a TypeScript path alias configured in tsconfig.json
        // This fixes Issue #10607: @components/Button should not trigger error if @components is a path alias
        if package_name.starts_with('@') {
            if is_path_alias_prefix(package_name, &path.to_path_buf()) {
                return None; // Valid path alias, ignore this import
            }
        }