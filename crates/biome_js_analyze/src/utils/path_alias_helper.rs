//! Helpers para path aliases TypeScript

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
    fn test_contains_pattern() {
        let content = r#"{
            "compilerOptions": {
                "paths": {
                    "@/*": ["./src/*"],
                    "@components/*": ["./src/components/*"],
                    "@utils/*": ["./src/utils/*"]
                }
            }
        }"#;

        assert!(content.contains(&format!("\"{}*\"", "components")));
        assert!(content.contains(&format!("\"{}/*\"", "components")));
    }

    #[test]
    fn test_not_contains() {
        let content = r#"{
            "compilerOptions": {
                "paths": {
                    "@/*": ["./src/*"]
                }
            }
        }"#;

        assert!(!content.contains(&format!("\"{}*\"", "components")));
    }
}