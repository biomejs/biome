---
"@biomejs/biome": minor
---

Added 2 options from `typescript-eslint` (`ignoreDifferentlyNamedParameters` and `ignoreDifferentJsDoc`) to [`useUnifiedTypeSignatures`](https://biomejs.dev/linter/rules/use-unified-type-signatures/).

Each of them will conditionally cause the rule to ignore function overloads with either differently named parameters or different JSDoc comments.

Example with `ignoreDifferentlyNamedParameters` set to `true`:

```ts
enum FoodType {
  FILET_MIGNON,
  APPLE_COBBLER,
  BLACK_FOREST_CAKE,
  SHRIMP_COCKTAIL,
}

// These overloads would normally error due to being mergeable into 1 signature, but
// keeping separate parameter names is often useful when discriminating on an enum member or string literal.
// (An alternative would be creating a type map from discriminant to parameter list.)

function cook(type: FoodType.FILET_MIGNON, numSheep: number): void;
function cook(
  type: FoodType.APPLE_COBBLER,
  pieType: string,
  numPies: number,
): void;
function cook(
  type: FoodType.BLACK_FOREST_CAKE,
  addCherriesToCake: boolean,
): void;
function cook(type: FoodType.SHRIMP_COCKTAIL, sauces: string[]): void;
function cook(type: FoodType, ...params: unknown): void {
  switch (type) {
    case FoodType.FILET_MIGNON:
    // ...
  }
}
```

Example for `ignoreDifferentJsDoc` set to `true`:

```ts
// With the option disabled, this fails due to the "no starters" case being mergeable into the "has starters" case.
// With it enabled, both are kept alone.

/**
 * Generate a random battle with the specified starters.
 */
export async function startBattle(starters: SpeciesId[]): Promise<void>;
/**
 * Generate a random battle with 3 randomly picked starters.
 * @deprecated - specify the starters to avoid RNG and be more explicit
 */
// TODO: Remove
export async function startBattle(): Promise<void>;
```
