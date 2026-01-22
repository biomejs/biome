---
"@biomejs/biome": patch
---

Fixed [#8809](https://github.com/biomejs/biome/issues/8809), [#7985](https://github.com/biomejs/biome/issues/7985), and [#8136](https://github.com/biomejs/biome/issues/8136): the `noSecrets` rule no longer reports false positives on common CamelCase identifiers like `paddingBottom`, `backgroundColor`, `unhandledRejection`, `uncaughtException`, and `IngestGatewayLogGroup`.

The entropy calculation algorithm now uses "average run length" to distinguish between legitimate CamelCase patterns (which have longer runs of same-case letters) and suspicious alternating case patterns (which have short runs).
