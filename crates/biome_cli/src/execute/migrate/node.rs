use std::process::Command;

use crate::{diagnostics::MigrationDiagnostic, CliDiagnostic};

/// Imports `specifier` using Node's `import()` or node's `require()` and
/// returns the JSONified content of its default export.
pub(crate) fn load_config(specifier: &str) -> Result<Resolution, CliDiagnostic> {
    // JSON.stringify replacer to avoid serializing cyclic references
    let replacer = "(_key, val) => {
        if (val != null && typeof val == 'object') {
            if (seen.has(val)) { return; }
            seen.add(val);
        }
        return val;
    }";
    let content_output = Command::new("node")
        .arg("--eval")
        .arg(format!(
            "import('{specifier}').then((c) => {{
                const seen = new Set();
                console.log(JSON.stringify(c.default, {replacer}));
            }})"
        ))
        .output();
    match content_output {
        Err(_) => {
            Err(CliDiagnostic::MigrateError(MigrationDiagnostic {
                reason: "The `node` program doesn't exist or cannot be invoked by Biome.\n`node` is invoked to resolve ESLint configurations written in JavaScript.\nThis includes shared configurations and plugin configurations imported with ESLint's `extends`.".to_string()
            }))
        },
        Ok(output) => {
            let path_output = Command::new("node")
                .arg("--print")
                .arg(format!(
                    "require.resolve('{specifier}')"
                ))
                .output();
            let resolved_path = path_output.ok().map_or(String::new(), |path_output| String::from_utf8_lossy(&path_output.stdout).trim().to_string());
            if !output.stderr.is_empty() {
                // Try with `require` before giving up.
                let output2 = Command::new("node")
                    .arg("--eval")
                    .arg(format!(
                        "const seen = new Set(); console.log(JSON.stringify(require('{specifier}'), {replacer}))"
                    ))
                    .output();
                if let Ok(output2) = output2 {
                    if output2.stderr.is_empty() {
                        return Ok(Resolution {
                            content: String::from_utf8_lossy(&output2.stdout).to_string(),
                            resolved_path,
                        });
                    }
                }
                let stderr = String::from_utf8_lossy(&output.stderr);
                return Err(CliDiagnostic::MigrateError(MigrationDiagnostic {
                    reason: format!("`node` was invoked to resolve '{specifier}'. This invocation failed with the following error:\n{stderr}")
                }));
            }
            Ok(Resolution {
                content: String::from_utf8_lossy(&output.stdout).to_string(),
                resolved_path,
            })
        }
    }
}

#[derive(Debug)]
pub(crate) struct Resolution {
    /// Resolved path of the file.
    /// May be empty if the resolution failed.
    pub(crate) resolved_path: String,
    /// File content in JSON
    pub(crate) content: String,
}
