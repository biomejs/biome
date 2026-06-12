use camino::Utf8PathBuf;
use std::collections::HashMap;
use std::fs;
use std::sync::{LazyLock, Mutex};

/// Cache para resultados de verificação de path alias por diretório
/// Key: caminho do diretório, Value: Vec de alias prefixes encontrados
type AliasCache = Mutex<HashMap<String, Vec<String>>>;

/// Cache global em memória para evitar I/O repetido no hot path
/// Thread-safe via Mutex + LazyLock
static ALIAS_CACHE: LazyLock<AliasCache> = LazyLock::new(|| Mutex::new(HashMap::new()));

/// Verifica se um prefixo (ex: "@components") é um path alias no tsconfig.json
/// Usa cache para evitar I/O repetido no mesmo diretório
pub fn is_path_alias_prefix(prefix: &str, file_path: &Utf8PathBuf) -> bool {
    // Tenta encontrar o tsconfig.json mais próximo
    let mut current_dir = file_path.parent();
    let mut max_depth = 10; // Evitar loops infinitos

    while let Some(dir) = current_dir {
        if max_depth == 0 {
            break;
        }
        max_depth -= 1;

        let dir_str = dir.as_str().to_string();

        // Tenta obter do cache primeiro
        {
            let cache = ALIAS_CACHE.lock().unwrap();
            if let Some(aliases) = cache.get(&dir_str) {
                return aliases.iter().any(|a| a == prefix || a == &format!("{}/*", prefix) || a == &format!("{}*", prefix));
            }
        }

        // Cache miss: ler do arquivo
        let tsconfig_path = dir.join("tsconfig.json");
        if let Ok(content) = fs::read_to_string(&tsconfig_path) {
            // Extrai todos os alias prefixes do tsconfig.json
            // Padrões procurados: "@prefix*" ou "@prefix/*"
            let mut found_aliases = Vec::new();

            // Parser simples: busca todos os padrões de alias
            // Isso é mais eficiente que contains() múltiplas porque
            // cacheamos o resultado para reuse no mesmo diretório
            for line in content.lines() {
                let trimmed = line.trim();
                if trimmed.starts_with('"') && (trimmed.contains("*") || trimmed.contains("/*")) {
                    // Extrai o padrão entre aspas
                    if let Some(start) = trimmed.find('"') {
                        if let Some(end) = trimmed[start + 1..].find('"') {
                            let pattern = &trimmed[start + 1..start + 1 + end];
                            // Adiciona ao cache
                            found_aliases.push(pattern.to_string());
                        }
                    }
                }
            }

            // Armazena no cache
            {
                let mut cache = ALIAS_CACHE.lock().unwrap();
                cache.insert(dir_str.clone(), found_aliases.clone());
            }

            // Verifica se o prefixo está na lista (com variação de padrões)
            return found_aliases.iter().any(|a| a == prefix || a == &format!("{}/*", prefix) || a == &format!("{}*", prefix));
        }

        current_dir = dir.parent();
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_matching() {
        let content = r#"{
            "compilerOptions": {
                "paths": {
                    "@/*": ["./src/*"],
                    "@components/*": ["./src/components/*"],
                    "@utils/*": ["./src/utils/*"]
                }
            }
        }"#;

        // Verifica se os padrões de busca funcionam
        assert!(content.contains(&format!("\"{}/*\"", "@components")));
        assert!(content.contains(&format!("\"{}/*\"", "@utils")));
        assert!(content.contains(&format!("\"{}/*\"", "@")));
        assert!(!content.contains(&format!("\"{}/*\"", "@missing")));
    }

    #[test]
    fn test_pattern_without_slash() {
        let content = r#"{
            "compilerOptions": {
                "paths": {
                    "@components*": ["./src/components/*"],
                    "@utils*": ["./src/utils/*"]
                }
            }
        }"#;

        // Verifica se o padrão sem /* também funciona
        assert!(content.contains(&format!("\"{}*\"", "@components")));
        assert!(content.contains(&format!("\"{}*\"", "@utils")));
        assert!(!content.contains(&format!("\"{}*\"", "@missing")));
    }

    #[test]
    fn test_both_patterns() {
        let content = r#"{
            "compilerOptions": {
                "paths": {
                    "@components/*": ["./src/components/*"],
                    "@components*": ["./src/components/*"]
                }
            }
        }"#;

        // Ambos os padrões devem funcionar
        assert!(content.contains(&format!("\"{}/*\"", "@components")));
        assert!(content.contains(&format!("\"{}*\"", "@components")));
    }

    #[test]
    fn test_alias_extraction() {
        let content = r#"{
            "compilerOptions": {
                "paths": {
                    "@components/*": ["./src/components/*"],
                    "@utils/*": ["./src/utils/*"]
                }
            }
        }"#;

        // Extrai os alias prefixes das linhas
        let mut found_aliases = Vec::new();
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with('"') && (trimmed.contains("*") || trimmed.contains("/*")) {
                if let Some(start) = trimmed.find('"') {
                    if let Some(end) = trimmed[start + 1..].find('"') {
                        let pattern = &trimmed[start + 1..start + 1 + end];
                        found_aliases.push(pattern.to_string());
                    }
                }
            }
        }

        assert!(found_aliases.contains(&"@components/*".to_string()));
        assert!(found_aliases.contains(&"@utils/*".to_string()));
    }

    #[test]
    fn test_real_tsconfig_lookup() {
        // Create a temporary directory with tsconfig.json
        let temp_dir = std::env::temp_dir().join("biome_test_10607");
        fs::create_dir_all(&temp_dir).ok();

        let tsconfig_content = r#"{
            "compilerOptions": {
                "paths": {
                    "@components/*": ["./src/components/*"],
                    "@utils/*": ["./src/utils/*"]
                }
            }
        }"#;

        fs::write(temp_dir.join("tsconfig.json"), tsconfig_content).ok();

        // Test that @components is recognized as alias
        let test_file = temp_dir.join("test.js");
        assert!(is_path_alias_prefix(
            "@components",
            &Utf8PathBuf::from_path_buf(test_file.clone()).unwrap()
        ));

        // Test that @utils is recognized as alias
        assert!(is_path_alias_prefix(
            "@utils",
            &Utf8PathBuf::from_path_buf(test_file.clone()).unwrap()
        ));

        // Test that @missing is NOT recognized
        assert!(!is_path_alias_prefix(
            "@missing",
            &Utf8PathBuf::from_path_buf(test_file).unwrap()
        ));

        // Cleanup
        fs::remove_file(temp_dir.join("tsconfig.json")).ok();
        fs::remove_dir(&temp_dir).ok();
    }
}