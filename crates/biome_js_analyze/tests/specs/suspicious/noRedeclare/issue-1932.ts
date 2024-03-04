export interface EventMap {
  'article-viewed': { id: string };
  'chat-started': { name: string; email: string; subject: string };
}

export interface SomeFunc {
  <K extends keyof EventMap>(cmd: 'on', event: K, cb: (data: EventMap[K]) => void): void;
  <K extends keyof EventMap>(cmd: 'once', event: K, cb: (data: EventMap[K]) => void): void;
}
