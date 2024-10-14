// See https://github.com/biomejs/biome/issues/3895
declare module "eslint-plugin-jsx-a11y" {
   import type { Linter } from "eslint";
 
   export const flatConfigs: {
     readonly recommended: { readonly rules: Readonly<Linter.RulesRecord> };
     readonly strict: { readonly rules: Readonly<Linter.RulesRecord> };
   };
 }