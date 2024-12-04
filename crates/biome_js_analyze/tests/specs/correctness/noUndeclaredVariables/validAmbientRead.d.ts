import type { TransformStream as TransformWebStream } from "node:stream/web";

export class TextLineStream extends TransformWebStream<string, string> {}
