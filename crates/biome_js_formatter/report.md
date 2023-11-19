# Overall Metrics

**Average compatibility**: 0.00

    <details>
    	<summary>Definition</summary>

    	$$average = \frac\{\sum_{file}^\{files}compatibility_\{file}}\{files}$$
    </details>

    **Compatible lines**: 0.00
    <details>
        <summary>Definition</summary>

        $$average = \frac{\sum_{file}^{files}matching\_lines_{file}}{max(lines_{rome}, lines_{prettier})}$$
    </details>

    [Metric definition discussion](https://github.com/rome/tools/issues/2555#issuecomment-1124787893)
                

# typescript/conformance/types/moduleDeclaration/kind-detection.ts
```diff
-declare namespace /* module */ A {}
+declare namespace A {} /* module */

```

**Prettier Similarity**: 0.00%


