# ARIA data

This package provides a script that extracts data from an ARIA specification.
This is a best effort approach because the ARIA specifications are in a semi-structured representation.

Just call the script with the name an dversion of the ARIA specification and write to a given file:

```shell
node generate-aria-data.js wai-aria-1.2 >| wai-aria-1-2.json
```
