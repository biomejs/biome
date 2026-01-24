/* should not generate diagnostics */

declare module "react" {
  export interface ComponentMeta<T> {
    displayName?: string;
    version?: string;
  }

  export interface HookResult<T> {
    error?: Error | null;
    isLoading: boolean;
  }
}
