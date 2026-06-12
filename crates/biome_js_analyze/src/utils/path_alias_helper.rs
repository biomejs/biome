use camino::Utf8PathBuf;
use std::fs;

/// Verifica se um prefixo (ex: "@components") é um path alias no tsconfig.json
pub fn is_path_alias_prefix(prefix: &str, file_path: &Utf8PathBuf) -> bool {
    // Tenta encontrar o tsconfig.json mais próximo
    let mut current_dir = file_path.parent();
    let mut max_depth = 10; // Evitar loops infinitos

    while let Some(dir) = current_dir {
        if max_depth == 0 {
            break;
        }
        max_depth -= 1;

        let tsconfig_path = dir.join("tsconfig.json");
        // Graceful degradation: se não conseguir ler, retorna false (trata como npm package)
        if let Ok(content) = fs::read_to_string(&tsconfig_path) {
            // Verificação simplificada: busca o prefixo no tsconfig.json
            // Padrões procurados:
            // - "@prefix*": [...]  (padrão exato)
            // - "@prefix/*": [...] (padrão com /*)
            if content.contains(&format!("\"{}*\"", prefix))
                || content.contains(&format!("\"{}/*\"", prefix))
            {
                return true;
            }
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
}