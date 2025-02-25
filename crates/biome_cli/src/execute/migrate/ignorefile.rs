use biome_fs::{FileSystem, OpenOptions};
use camino::Utf8Path;
use std::io;

/// Read an ignore file that follows gitignore pattern syntax,
/// and turn them into a list of UNIX glob patterns.
pub(crate) fn read_ignore_file(
    fs: &dyn FileSystem,
    ignore_filename: &str,
) -> io::Result<IgnorePatterns> {
    let mut file = fs.open_with_options(
        Utf8Path::new(ignore_filename),
        OpenOptions::default().read(true),
    )?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(IgnorePatterns::from(&content))
}

#[derive(Debug)]
pub(crate) struct IgnorePatterns {
    pub(crate) patterns: Box<[biome_glob::Glob]>,
}
impl IgnorePatterns {
    pub(crate) fn from(content: &str) -> Self {
        let mut patterns = Vec::new();
        if let Ok(glob) = "**".parse() {
            patterns.push(glob);
        }
        for line in content.lines() {
            // Trailing spaces are ignored
            let line = line.trim_end();
            // empty lines and comments are ignored
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Ok(glob) = convert_pattern(line).parse() {
                patterns.push(glob);
            }
        }
        IgnorePatterns {
            patterns: patterns.into_boxed_slice(),
        }
    }
}

pub(crate) fn convert_pattern(line: &str) -> String {
    let (negation, line) = if let Some(rest) = line.strip_prefix('!') {
        ("", rest)
    } else {
        ("!", line)
    };
    let result = if let Some(stripped_line) = line.strip_prefix('/') {
        // Patterns that starts with `/` are relative to the ignore file
        format!("{negation}./{stripped_line}")
    } else if line.find('/').is_some_and(|index| index < (line.len() - 1))
        || line == "**"
        || line == "**/"
    {
        // Patterns that includes at least one `/` in the middle are relatives paths
        format!("{negation}{line}")
    } else {
        format!("{negation}**/{line}")
    };
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        const IGNORE_FILE_CONTENT: &str = r#""#;
        let result = IgnorePatterns::from(IGNORE_FILE_CONTENT);

        assert_eq!(result.patterns.as_ref(), ["**".parse().unwrap(),]);
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

        assert_eq!(result.patterns.as_ref(), ["**".parse().unwrap(),]);
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

        assert_eq!(
            result.patterns.as_ref(),
            [
                "**".parse().unwrap(),
                "!**/file-or-dir".parse().unwrap(),
                "!**/dir/".parse().unwrap(),
                "!**".parse().unwrap(),
                "!**/".parse().unwrap(),
                "!**/*".parse().unwrap(),
                "!**/*/".parse().unwrap(),
            ]
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

        assert_eq!(
            result.patterns.as_ref(),
            [
                "**".parse().unwrap(),
                "!dir/dubfile-or-subdir".parse().unwrap(),
                "!dir/subdir/".parse().unwrap(),
                "!**/*".parse().unwrap(),
                "!**/*/".parse().unwrap(),
                "!**/a/b".parse().unwrap(),
                "!**/a/b/".parse().unwrap(),
            ]
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

        assert_eq!(
            result.patterns.as_ref(),
            [
                "**".parse().unwrap(),
                "!./dir/dubfile-or-subdir".parse().unwrap(),
                "!./dir/subdir/".parse().unwrap(),
                "!./**/*".parse().unwrap(),
                "!./**/*/".parse().unwrap(),
                "!./**/a/b".parse().unwrap(),
                "!./**/a/b/".parse().unwrap(),
            ]
        );
    }

    #[test]
    fn negated_pattern() {
        const IGNORE_FILE_CONTENT: &str = r#"!a"#;
        let result = IgnorePatterns::from(IGNORE_FILE_CONTENT);

        assert_eq!(
            result.patterns.as_ref(),
            ["**".parse().unwrap(), "**/a".parse().unwrap(),]
        );
    }

    #[test]
    fn take_leading_spaces_into_account() {
        const IGNORE_FILE_CONTENT: &str = r#"
    # This is not a comment because there is some leading spaces
        "#;
        let result = IgnorePatterns::from(IGNORE_FILE_CONTENT);

        assert_eq!(
            result.patterns.as_ref(),
            [
                "**".parse().unwrap(),
                "!**/    # This is not a comment because there is some leading spaces"
                    .parse()
                    .unwrap(),
            ]
        );
    }
}
