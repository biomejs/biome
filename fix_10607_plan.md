# Fix para Issue #10607: tsconfig paths ignorados

## Problema

Imports usando path aliases TypeScript (ex: `import "@/lib/utils"`) são tratados como npm scopes e causam erro "Dependency @lib isn't specified in package.json".

## Causa Raiz

`parse_package_name` não distingue `@scope/pkg` de `@alias/path`. Ambos retornam um valor `Option<&str>`. A regra `noUndeclaredDependencies` assume que qualquer retorno válido é uma dependência npm.

## Solução

Antes de reportar erro, verificar se o prefixo `@xxx` existe nas configurações de paths do projeto (`tsconfig.json` ou biome.json extends).

### Algoritmo

```rust
// No método run(), após parse_package_name

let package_name = match parse_package_name(import_text.text()) {
    Some(name) => name,
    None => return None, // @/, ./, node:, etc - já ignorados
};

// NOVO: Se parece com um path alias (@alias), verificar tsconfig
if package_name.starts_with('@') {
    // 1. Tentar ler biome.json ou tsconfig.json
    // 2. Verificar se @alias* existe em paths
    // 3. Se existe -> é path alias, ignorar (return None)
    // 4. Se não existe -> prosseguir checagem de dependência
}
```

### Abordagem Técnica

Biome já expõe `biome_configuration` via `services`. Vou verificar se consigo acessar isso no `RuleContext`.

Caso não tenha acesso direto, vou implementar um parser simples de JSON para ler o arquivo `tsconfig.json` local.

## Test Cases

```javascript
// tsconfig.json: { "paths": { "@foo/*": ["./src/*"] } }

// ❌ Deve falhar (npm scope)
import "@bar/baz"; // @bar não está no package.json

// ✅ Deve passar (path alias)
import "@foo/component"; // @foo está no tsconfig.json paths
import "@/lib/utils"; // @/ retorna None em parse_package_name (já OK)
```