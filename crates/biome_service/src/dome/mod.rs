//! [Dome] handles all the files that should be handled

use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::path::PathBuf;

pub struct DomePath(pub PathBuf);

pub struct Dome {
    paths: BTreeSet<DomePath>,
}

impl DomePath {
    fn get_file_name(&self) -> Option<&str> {
        self.0.file_name().and_then(|s| s.to_str())
    }

    fn has_priority(file_name: &str) -> bool {
        matches!(file_name, "package.json" | "tsconfig.json")
    }
}

impl Eq for DomePath {}

impl PartialEq<Self> for DomePath {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd<Self> for DomePath {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DomePath {
    fn cmp(&self, other: &Self) -> Ordering {
        let current_file_name = self.get_file_name();
        let other_file_name = other.get_file_name();
        match (current_file_name, other_file_name) {
            (Some(current_file_name), Some(other_file_name)) => {
                if Self::has_priority(current_file_name) && !Self::has_priority(other_file_name) {
                    Ordering::Less
                } else if !Self::has_priority(current_file_name)
                    && Self::has_priority(other_file_name)
                {
                    Ordering::Greater
                } else {
                    current_file_name.cmp(other_file_name)
                }
            }
            (Some(_), None) => Ordering::Less,
            (None, Some(_)) => Ordering::Greater,
            (None, None) => Ordering::Equal,
        }
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_dome_path() {
        use super::DomePath;
        use std::path::PathBuf;

        let path = PathBuf::from("package.json");
        let dome_path = DomePath(path);
        assert_eq!(dome_path.get_file_name(), Some("package.json"));
        assert_eq!(DomePath::has_priority("package.json"), true);
    }
}
