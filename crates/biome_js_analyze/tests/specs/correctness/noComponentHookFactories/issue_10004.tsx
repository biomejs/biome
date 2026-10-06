/* should not generate diagnostics */

// https://github.com/biomejs/biome/issues/10004
export function createClient(): MyHttpClient {
  return {
    async Request(): Promise<void> {},
  }
}

export interface MyHttpClient {
  Request(): Promise<void>
}

// PascalCase class method inside a factory function
function createService() {
  class InternalService {
    async Execute(): Promise<void> {}
  }
  return new InternalService();
}
