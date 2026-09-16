/* should not generate diagnostics */

// Fixture for https://github.com/biomejs/biome/issues/10892. The `success()`
// function deliberately shares its name with the union's `success` member,
// which used to shadow it during type inference.
export namespace Types {
  export type Always = { truthy: true; data: string };

  export type Result<T> =
    | { success: true; data: T }
    | { success: false; error: string };

  export function success(): true {
    return true;
  }
}

export default Types;
