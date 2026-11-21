use camino::Utf8Path;

pub struct ConfigName;

impl ConfigName {
    const BIOME_JSON: [&'static str; 4] =
        ["biome.json", "biome.jsonc", ".biome.json", ".biome.jsonc"];

    pub const fn biome_json() -> &'static str {
        Self::BIOME_JSON[0]
    }

    pub const fn biome_jsonc() -> &'static str {
        Self::BIOME_JSON[1]
    }

    pub const fn file_names() -> [&'static str; 4] {
        Self::BIOME_JSON
    }

    pub fn matches_file_name(file_name: &str) -> bool {
        Self::BIOME_JSON.contains(&file_name)
    }

    pub fn is_manifest_file(path: &Utf8Path) -> bool {
        path.file_name().is_some_and(Self::matches_file_name)
    }
}
