use std::{io, path::Path};

use biome_fs::{FileSystem, OpenOptions};
use biome_service::DynRef;
use indexmap::IndexSet;

/// Read an ignore file that follows gitignore pattern syntax,
/// and turn them into a list of UNIX glob patterns.
pub(crate) fn read_ignore_file(
    fs: &DynRef<'_, dyn FileSystem>,
    ignore_filename: &str,
) -> io::Result<IgnorePatterns> {
    let mut file = fs.open_with_options(
        Path::new(ignore_filename),
        OpenOptions::default().read(true),
    )?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(IgnorePatterns::from(&content))
}

#[derive(Debug)]
pub(crate) struct IgnorePatterns {
    pub(crate) patterns: IndexSet<String>,
    pub(crate) has_negated_patterns: bool,
}
impl IgnorePatterns {
    pub(crate) fn from(content: &str) -> Self {
        let mut has_negated_patterns = false;
        let mut patterns = IndexSet::new();
        for line in content.lines() {
            // Trailing spaces are ignored
            let line = line.trim_end();
            // empty lines and comments are ignored
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            match convert_pattern(line) {
                Ok(pattern) => {
                    patterns.insert(pattern);
                }
                Err(_) => {
                    has_negated_patterns = true;
                    // Skip negated patterns because we don't support them.
                    continue;
                }
            }
        }
        IgnorePatterns {
            patterns,
            has_negated_patterns,
        }
    }
}

pub(crate) fn convert_pattern(line: &str) -> Result<String, &'static str> {
    if line.starts_with('!') {
        // Skip negated patterns because we don't support them.
        return Err("Negated patterns are not supported.");
    }
    let result = if let Some(stripped_line) = line.strip_prefix('/') {
        // Patterns tha tstarts with `/` are relative to the ignore file
        format!("./{stripped_line}")
    } else if line.find('/').is_some_and(|index| index < (line.len() - 1))
        || line == "**"
        || line == "**/"
    {
        // Patterns that includes at least one `/` in the middle are relatives paths
        line.to_string()
    } else {
        format!("**/{line}")
    };
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        const IGNORE_FILE_CONTENT: &str = r#""#;
        let result = IgnorePatterns::from(IGNORE_FILE_CONTENT);

        assert!(!result.has_negated_patterns);
        assert!(result.patterns.is_empty());
    }

    #[test]
    fn comments_and_empty_lines() {
        const IGNORE_FILE_CONTENT: &str = r#"
# Comment 1
# folloed by a blank line
    
# Comment 2
# folloed by a blank line (trailing space are ignored)

        "#;
        let result = IgnorePatterns::from(IGNORE_FILE_CONTENT);

        assert!(!result.has_negated_patterns);
        assert!(result.patterns.is_empty());
    }

    #[test]
    fn non_relative_patterns() {
        const IGNORE_FILE_CONTENT: &str = r#"
file-or-dir
dir/
**
**/
*
*/
"#;
        let result = IgnorePatterns::from(IGNORE_FILE_CONTENT);

        assert!(!result.has_negated_patterns);
        assert_eq!(
            result.patterns,
            [
                "**/file-or-dir".to_string(),
                "**/dir/".to_string(),
                "**".to_string(),
                "**/".to_string(),
                "**/*".to_string(),
                "**/*/".to_string(),
            ]
            .into()
        );
    }

    #[test]
    fn relative_patterns() {
        const IGNORE_FILE_CONTENT: &str = r#"
dir/dubfile-or-subdir
dir/subdir/
**/*
**/*/
**/a/b
**/a/b/
"#;
        let result = IgnorePatterns::from(IGNORE_FILE_CONTENT);

        assert!(!result.has_negated_patterns);
        assert_eq!(
            result.patterns,
            [
                "dir/dubfile-or-subdir".to_string(),
                "dir/subdir/".to_string(),
                "**/*".to_string(),
                "**/*/".to_string(),
                "**/a/b".to_string(),
                "**/a/b/".to_string(),
            ]
            .into()
        );
    }

    #[test]
    fn relative_patterns_starting_with_root_slash() {
        const IGNORE_FILE_CONTENT: &str = r#"
/dir/dubfile-or-subdir
/dir/subdir/
/**/*
/**/*/
/**/a/b
/**/a/b/
"#;
        let result = IgnorePatterns::from(IGNORE_FILE_CONTENT);

        assert!(!result.has_negated_patterns);
        assert_eq!(
            result.patterns,
            [
                "./dir/dubfile-or-subdir".to_string(),
                "./dir/subdir/".to_string(),
                "./**/*".to_string(),
                "./**/*/".to_string(),
                "./**/a/b".to_string(),
                "./**/a/b/".to_string(),
            ]
            .into()
        );
    }

    #[test]
    fn negated_pattern() {
        const IGNORE_FILE_CONTENT: &str = r#"!a"#;
        let result = IgnorePatterns::from(IGNORE_FILE_CONTENT);

        assert!(result.has_negated_patterns);
        assert!(result.patterns.is_empty());
    }

    #[test]
    fn take_leading_spaces_into_account() {
        const IGNORE_FILE_CONTENT: &str = r#"
    # This is not a comment because there is some leading spaces
        "#;
        let result = IgnorePatterns::from(IGNORE_FILE_CONTENT);

        assert!(!result.has_negated_patterns);
        assert_eq!(
            result.patterns,
            ["**/    # This is not a comment because there is some leading spaces".to_string()]
                .into()
        );
    }
}
