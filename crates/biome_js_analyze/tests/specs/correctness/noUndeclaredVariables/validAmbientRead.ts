import type { TransformStream as TransformWebStream } from "node:stream/web";

export declare class TextLineStream extends TransformWebStream<string, string> {}
