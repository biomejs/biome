import { GenericSchema, GenericSchemaAsync, InferInput, InferOutput, SafeParseResult } from './valibot';
declare const GET_MATCH_RESULT: unique symbol;
declare const METHOD_NAME_ALL_LOWERCASE: "all";
declare const METHODS: readonly [
    "get",
    "post",
    "put",
    "delete",
    "options",
    "patch",
    "query"
];
interface Router<T> {
    name: string;
    add(method: string, path: string, handler: T): void;
    match(method: string, path: string): Result<T>;
}
type ParamIndexMap = Record<string, number>;
type ParamStash = string[];
type Params = Record<string, string>;
type Result<T> = [
    [
        T,
        ParamIndexMap
    ][],
    ParamStash
] | [
    [
        T,
        Params
    ][]
];
type RequestHeader = 'A-IM' | 'Accept' | 'Accept-Additions' | 'Accept-CH' | 'Accept-Charset' | 'Accept-Datetime' | 'Accept-Encoding' | 'Accept-Features' | 'Accept-Language' | 'Accept-Patch' | 'Accept-Post' | 'Accept-Query' | 'Accept-Ranges' | 'Accept-Signature' | 'Access-Control' | 'Access-Control-Allow-Credentials' | 'Access-Control-Allow-Headers' | 'Access-Control-Allow-Methods' | 'Access-Control-Allow-Origin' | 'Access-Control-Expose-Headers' | 'Access-Control-Max-Age' | 'Access-Control-Request-Headers' | 'Access-Control-Request-Method' | 'Activate-Storage-Access' | 'Age' | 'Allow' | 'ALPN' | 'Alt-Svc' | 'Alt-Used' | 'Alternates' | 'AMP-Cache-Transform' | 'Apply-To-Redirect-Ref' | 'Authentication-Control' | 'Authentication-Info' | 'Authorization' | 'Available-Dictionary' | 'C-Ext' | 'C-Man' | 'C-Opt' | 'C-PEP' | 'C-PEP-Info' | 'Cache-Control' | 'Cache-Group-Invalidation' | 'Cache-Groups' | 'Cache-Status' | 'Cal-Managed-ID' | 'CalDAV-Timezones' | 'Capsule-Protocol' | 'CDN-Cache-Control' | 'CDN-Loop' | 'Cert-Not-After' | 'Cert-Not-Before' | 'Clear-Site-Data' | 'Client-Cert' | 'Client-Cert-Chain' | 'Close' | 'CMCD-Object' | 'CMCD-Request' | 'CMCD-Session' | 'CMCD-Status' | 'CMSD-Dynamic' | 'CMSD-Static' | 'Concealed-Auth-Export' | 'Configuration-Context' | 'Connection' | 'Content-Base' | 'Content-Digest' | 'Content-Disposition' | 'Content-Encoding' | 'Content-ID' | 'Content-Language' | 'Content-Length' | 'Content-Location' | 'Content-MD5' | 'Content-Range' | 'Content-Script-Type' | 'Content-Security-Policy' | 'Content-Security-Policy-Report-Only' | 'Content-Style-Type' | 'Content-Type' | 'Content-Version' | 'Cookie' | 'Cookie2' | 'Cross-Origin-Embedder-Policy' | 'Cross-Origin-Embedder-Policy-Report-Only' | 'Cross-Origin-Opener-Policy' | 'Cross-Origin-Opener-Policy-Report-Only' | 'Cross-Origin-Resource-Policy' | 'CTA-Common-Access-Token' | 'DASL' | 'Date' | 'DAV' | 'Default-Style' | 'Delta-Base' | 'Deprecation' | 'Depth' | 'Derived-From' | 'Destination' | 'Detached-JWS' | 'Differential-ID' | 'Dictionary-ID' | 'Digest' | 'DPoP' | 'DPoP-Nonce' | 'Early-Data' | 'EDIINT-Features' | 'ETag' | 'Expect' | 'Expect-CT' | 'Expires' | 'Ext' | 'Forwarded' | 'From' | 'GetProfile' | 'Hobareg' | 'Host' | 'HTTP2-Settings' | 'If' | 'If-Match' | 'If-Modified-Since' | 'If-None-Match' | 'If-Range' | 'If-Schedule-Tag-Match' | 'If-Unmodified-Since' | 'IM' | 'Include-Referred-Token-Binding-ID' | 'Incremental' | 'Isolation' | 'Keep-Alive' | 'Label' | 'Last-Event-ID' | 'Last-Modified' | 'Link' | 'Link-Template' | 'Location' | 'Lock-Token' | 'Man' | 'Max-Forwards' | 'Memento-Datetime' | 'Meter' | 'Method-Check' | 'Method-Check-Expires' | 'MIME-Version' | 'Negotiate' | 'NEL' | 'OData-EntityId' | 'OData-Isolation' | 'OData-MaxVersion' | 'OData-Version' | 'Opt' | 'Optional-WWW-Authenticate' | 'Ordering-Type' | 'Origin' | 'Origin-Agent-Cluster' | 'OSCORE' | 'OSLC-Core-Version' | 'Overwrite' | 'P3P' | 'PEP' | 'PEP-Info' | 'Permissions-Policy' | 'PICS-Label' | 'Ping-From' | 'Ping-To' | 'Position' | 'Pragma' | 'Prefer' | 'Preference-Applied' | 'Priority' | 'ProfileObject' | 'Protocol' | 'Protocol-Info' | 'Protocol-Query' | 'Protocol-Request' | 'Proxy-Authenticate' | 'Proxy-Authentication-Info' | 'Proxy-Authorization' | 'Proxy-Features' | 'Proxy-Instruction' | 'Proxy-Status' | 'Public' | 'Public-Key-Pins' | 'Public-Key-Pins-Report-Only' | 'Range' | 'Redirect-Ref' | 'Referer' | 'Referer-Root' | 'Referrer-Policy' | 'Refresh' | 'Repeatability-Client-ID' | 'Repeatability-First-Sent' | 'Repeatability-Request-ID' | 'Repeatability-Result' | 'Replay-Nonce' | 'Reporting-Endpoints' | 'Repr-Digest' | 'Retry-After' | 'Safe' | 'Schedule-Reply' | 'Schedule-Tag' | 'Sec-Fetch-Dest' | 'Sec-Fetch-Mode' | 'Sec-Fetch-Site' | 'Sec-Fetch-Storage-Access' | 'Sec-Fetch-User' | 'Sec-GPC' | 'Sec-Purpose' | 'Sec-Token-Binding' | 'Sec-WebSocket-Accept' | 'Sec-WebSocket-Extensions' | 'Sec-WebSocket-Key' | 'Sec-WebSocket-Protocol' | 'Sec-WebSocket-Version' | 'Security-Scheme' | 'Server' | 'Server-Timing' | 'Set-Cookie' | 'Set-Cookie2' | 'Set-Txn' | 'SetProfile' | 'Signature' | 'Signature-Input' | 'SLUG' | 'SoapAction' | 'Status-URI' | 'Strict-Transport-Security' | 'Sunset' | 'Surrogate-Capability' | 'Surrogate-Control' | 'TCN' | 'TE' | 'Timeout' | 'Timing-Allow-Origin' | 'Topic' | 'Traceparent' | 'Tracestate' | 'Trailer' | 'Transfer-Encoding' | 'TTL' | 'Unencoded-Digest' | 'Upgrade' | 'Urgency' | 'URI' | 'Use-As-Dictionary' | 'User-Agent' | 'Variant-Vary' | 'Vary' | 'Via' | 'Want-Content-Digest' | 'Want-Digest' | 'Want-Repr-Digest' | 'Want-Unencoded-Digest' | 'Warning' | 'WWW-Authenticate' | 'X-Content-Type-Options' | 'X-Frame-Options';
type ResponseHeader = 'Accept-Query' | 'Access-Control-Allow-Credentials' | 'Access-Control-Allow-Headers' | 'Access-Control-Allow-Methods' | 'Access-Control-Allow-Origin' | 'Access-Control-Expose-Headers' | 'Access-Control-Max-Age' | 'Activate-Storage-Access' | 'Age' | 'Allow' | 'Cache-Control' | 'Cache-Group-Invalidation' | 'Cache-Groups' | 'Clear-Site-Data' | 'Content-Disposition' | 'Content-Encoding' | 'Content-Language' | 'Content-Length' | 'Content-Location' | 'Content-Range' | 'Content-Security-Policy' | 'Content-Security-Policy-Report-Only' | 'Content-Type' | 'Cookie' | 'Cross-Origin-Embedder-Policy' | 'Cross-Origin-Opener-Policy' | 'Cross-Origin-Resource-Policy' | 'Date' | 'ETag' | 'Expires' | 'Incremental' | 'Last-Modified' | 'Location' | 'Permissions-Policy' | 'Pragma' | 'Retry-After' | 'Save-Data' | 'Sec-CH-Prefers-Color-Scheme' | 'Sec-CH-Prefers-Reduced-Motion' | 'Sec-CH-UA' | 'Sec-CH-UA-Arch' | 'Sec-CH-UA-Bitness' | 'Sec-CH-UA-Form-Factor' | 'Sec-CH-UA-Full-Version' | 'Sec-CH-UA-Full-Version-List' | 'Sec-CH-UA-Mobile' | 'Sec-CH-UA-Model' | 'Sec-CH-UA-Platform' | 'Sec-CH-UA-Platform-Version' | 'Sec-CH-UA-WoW64' | 'Sec-Fetch-Dest' | 'Sec-Fetch-Mode' | 'Sec-Fetch-Site' | 'Sec-Fetch-User' | 'Sec-GPC' | 'Server' | 'Server-Timing' | 'Service-Worker-Navigation-Preload' | 'Set-Cookie' | 'Set-Txn' | 'Strict-Transport-Security' | 'Timing-Allow-Origin' | 'Trailer' | 'Transfer-Encoding' | 'Unencoded-Digest' | 'Upgrade' | 'Vary' | 'Want-Unencoded-Digest' | 'Warning' | 'WWW-Authenticate' | 'X-Content-Type-Options' | 'X-DNS-Prefetch-Control' | 'X-Frame-Options' | 'X-Permitted-Cross-Domain-Policies' | 'X-Powered-By' | 'X-Robots-Tag' | 'X-XSS-Protection';
type CustomHeader = string & {};
type InfoStatusCode = 100 | 101 | 102 | 103;
type SuccessStatusCode = 200 | 201 | 202 | 203 | 204 | 205 | 206 | 207 | 208 | 226;
type DeprecatedStatusCode = 305 | 306;
type RedirectStatusCode = 300 | 301 | 302 | 303 | 304 | DeprecatedStatusCode | 307 | 308;
type ClientErrorStatusCode = 400 | 401 | 402 | 403 | 404 | 405 | 406 | 407 | 408 | 409 | 410 | 411 | 412 | 413 | 414 | 415 | 416 | 417 | 418 | 421 | 422 | 423 | 424 | 425 | 426 | 428 | 429 | 431 | 451;
type ServerErrorStatusCode = 500 | 501 | 502 | 503 | 504 | 505 | 506 | 507 | 508 | 510 | 511;
type UnofficialStatusCode = -1;
type StatusCode = InfoStatusCode | SuccessStatusCode | RedirectStatusCode | ClientErrorStatusCode | ServerErrorStatusCode | UnofficialStatusCode;
type ContentlessStatusCode = 101 | 204 | 205 | 304;
type ContentfulStatusCode = Exclude<StatusCode, ContentlessStatusCode>;
type UnionToIntersection<U> = (U extends any ? (k: U) => void : never) extends (k: infer I) => void ? I : never;
type RemoveBlankRecord<T> = T extends Record<infer K, unknown> ? (K extends string ? T : never) : never;
type IfAnyThenEmptyObject<T> = 0 extends 1 & T ? {} : T;
type JSONPrimitive = string | boolean | number | null;
type JSONArray = (JSONPrimitive | JSONObject | JSONArray)[];
type JSONObject = {
    [key: string]: JSONPrimitive | JSONArray | JSONObject | object | InvalidJSONValue;
};
type InvalidJSONValue = undefined | symbol | ((...args: unknown[]) => unknown);
type InvalidToNull<T> = T extends InvalidJSONValue ? null : T;
type IsInvalid<T> = T extends InvalidJSONValue ? true : false;
type OmitSymbolKeys<T> = {
    [K in keyof T as K extends symbol ? never : K]: T[K];
};
type JSONValue = JSONObject | JSONArray | JSONPrimitive;
type JSONParsed<T, TError = bigint | ReadonlyArray<bigint>> = T extends {
    toJSON(): infer J;
} ? (() => J) extends () => JSONPrimitive ? J : (() => J) extends () => {
    toJSON(): unknown;
} ? {} : JSONParsed<J, TError> : T extends JSONPrimitive ? T : T extends InvalidJSONValue ? never : T extends ReadonlyArray<unknown> ? {
    [K in keyof T]: JSONParsed<InvalidToNull<T[K]>, TError>;
} extends infer A ? A extends ReadonlyArray<unknown> ? A : JSONParsed<InvalidToNull<T[number]>, TError>[] : never : T extends Set<unknown> | Map<unknown, unknown> | Record<string, never> ? {} : T extends object ? T[keyof T] extends TError ? never : {
    [K in keyof OmitSymbolKeys<T> as IsInvalid<T[K]> extends true ? never : K]: boolean extends IsInvalid<T[K]> ? JSONParsed<T[K], TError> | undefined : JSONParsed<T[K], TError>;
} : T extends unknown ? T extends TError ? never : JSONValue : never;
type Simplify<T> = {
    [KeyType in keyof T]: T[KeyType];
} & {};
type RequiredKeysOf<BaseType extends object> = Exclude<{
    [Key in keyof BaseType]: BaseType extends Record<Key, BaseType[Key]> ? Key : never;
}[keyof BaseType], undefined>;
type HasRequiredKeys<BaseType extends object> = RequiredKeysOf<BaseType> extends never ? false : true;
type IsAny<T> = boolean extends (T extends never ? true : false) ? true : false;
type Bindings = object;
type Variables = object;
type BlankEnv = {};
type Env = {
    Bindings?: Bindings;
    Variables?: Variables;
};
type Next = () => Promise<void>;
type ExtractInput<I extends Input | Input['in']> = I extends Input ? unknown extends I['in'] ? {} : I['in'] : I;
type Input = {
    in?: {};
    out?: {};
    outputFormat?: ResponseFormat;
};
type BlankSchema = {};
type BlankInput = {};
interface RouterRoute {
    basePath: string;
    path: string;
    method: string;
    handler: H;
}
type HandlerResponse<O> = Response | TypedResponse<O> | Promise<Response | TypedResponse<O>> | Promise<void>;
type Handler<E extends Env = any, P extends string = any, I extends Input = BlankInput, R extends HandlerResponse<any> = any> = (c: Context<E, P, I>, next: Next) => R;
type MiddlewareHandler<E extends Env = any, P extends string = string, I extends Input = {}, R extends HandlerResponse<any> = Response> = (c: Context<E, P, I>, next: Next) => Promise<R | void>;
type H<E extends Env = any, P extends string = any, I extends Input = BlankInput, R extends HandlerResponse<any> = any> = Handler<E, P, I, R> | MiddlewareHandler<E, P, I, R>;
interface NotFoundResponse {
}
type NotFoundHandler<E extends Env = any> = (c: Context<E>) => NotFoundResponse extends Response ? NotFoundResponse | Promise<NotFoundResponse> : Response | Promise<Response>;
interface HTTPResponseError extends Error {
    getResponse: () => Response;
}
type ErrorHandler<E extends Env = any> = (err: Error | HTTPResponseError, c: Context<E>) => Response | Promise<Response>;
interface HandlerInterface<E extends Env = Env, M extends string = string, S extends Schema = BlankSchema, BasePath extends string = '/', CurrentPath extends string = BasePath> {
    <P extends string = CurrentPath, I extends Input = BlankInput, R extends HandlerResponse<any> = any, E2 extends Env = E>(handler: H<E2, P, I, R>): Hono$1<IntersectNonAnyTypes<[
        E,
        E2
    ]>, S & ToSchema<M, P, I, MergeTypedResponse<R>>, BasePath, CurrentPath>;
    <P extends string = CurrentPath, I extends Input = BlankInput, I2 extends Input = I, R extends HandlerResponse<any> = any, E2 extends Env = E, E3 extends Env = IntersectNonAnyTypes<[
        E,
        E2
    ]>, M1 extends H<E2, P, I> = H<E2, P, I>>(...handlers: [
        H<E2, P, I> & M1,
        H<E3, P, I2, R>
    ]): Hono$1<IntersectNonAnyTypes<[
        E,
        E2,
        E3
    ]>, S & ToSchema<M, P, I2, MergeTypedResponse<R> | MergeMiddlewareResponse<M1>>, BasePath, CurrentPath>;
    <P extends string, MergedPath extends MergePath<BasePath, P>, R extends HandlerResponse<any> = any, I extends Input = BlankInput, E2 extends Env = E>(path: P, handler: H<E2, MergedPath, I, R>): Hono$1<E, AddSchemaIfHasResponse<MergeTypedResponse<R>, S, M, P, I, BasePath>, BasePath, MergePath<BasePath, P>>;
    <P extends string = CurrentPath, R extends HandlerResponse<any> = any, I extends Input = BlankInput, I2 extends Input = I, I3 extends Input = I & I2, E2 extends Env = E, E3 extends Env = IntersectNonAnyTypes<[
        E,
        E2
    ]>, E4 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3
    ]>, M1 extends H<E2, P, I> = H<E2, P, I>, M2 extends H<E3, P, I2> = H<E3, P, I2>>(...handlers: [
        H<E2, P, I> & M1,
        H<E3, P, I2> & M2,
        H<E4, P, I3, R>
    ]): Hono$1<IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4
    ]>, S & ToSchema<M, P, I3, MergeTypedResponse<R> | MergeMiddlewareResponse<M1> | MergeMiddlewareResponse<M2>>, BasePath, CurrentPath>;
    <P extends string, MergedPath extends MergePath<BasePath, P>, R extends HandlerResponse<any> = any, I extends Input = BlankInput, I2 extends Input = I, E2 extends Env = E, E3 extends Env = IntersectNonAnyTypes<[
        E,
        E2
    ]>, M1 extends H<E2, MergedPath, I> = H<E2, MergedPath, I>>(path: P, ...handlers: [
        H<E2, MergedPath, I> & M1,
        H<E3, MergedPath, I2, R>
    ]): Hono$1<E, AddSchemaIfHasResponse<MergeTypedResponse<R> | MergeMiddlewareResponse<M1>, S, M, P, I2, BasePath>, BasePath, MergePath<BasePath, P>>;
    <P extends string = CurrentPath, R extends HandlerResponse<any> = any, I extends Input = BlankInput, I2 extends Input = I, I3 extends Input = I & I2, I4 extends Input = I & I2 & I3, E2 extends Env = E, E3 extends Env = IntersectNonAnyTypes<[
        E,
        E2
    ]>, E4 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3
    ]>, E5 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4
    ]>, M1 extends H<E2, P, I> = H<E2, P, I>, M2 extends H<E3, P, I2> = H<E3, P, I2>, M3 extends H<E4, P, I3> = H<E4, P, I3>>(...handlers: [
        H<E2, P, I> & M1,
        H<E3, P, I2> & M2,
        H<E4, P, I3> & M3,
        H<E5, P, I4, R>
    ]): Hono$1<IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5
    ]>, S & ToSchema<M, P, I4, MergeTypedResponse<R> | MergeMiddlewareResponse<M1> | MergeMiddlewareResponse<M2> | MergeMiddlewareResponse<M3>>, BasePath, CurrentPath>;
    <P extends string, MergedPath extends MergePath<BasePath, P>, R extends HandlerResponse<any> = any, I extends Input = BlankInput, I2 extends Input = I, I3 extends Input = I & I2, E2 extends Env = E, E3 extends Env = IntersectNonAnyTypes<[
        E,
        E2
    ]>, E4 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3
    ]>, M1 extends H<E2, MergedPath, I> = H<E2, MergedPath, I>, M2 extends H<E3, MergedPath, I2> = H<E3, MergedPath, I2>>(path: P, ...handlers: [
        H<E2, MergedPath, I> & M1,
        H<E3, MergedPath, I2> & M2,
        H<E4, MergedPath, I3, R>
    ]): Hono$1<E, AddSchemaIfHasResponse<MergeTypedResponse<R> | MergeMiddlewareResponse<M1> | MergeMiddlewareResponse<M2>, S, M, P, I3, BasePath>, BasePath, MergePath<BasePath, P>>;
    <P extends string = CurrentPath, R extends HandlerResponse<any> = any, I extends Input = BlankInput, I2 extends Input = I, I3 extends Input = I & I2, I4 extends Input = I & I2 & I3, I5 extends Input = I & I2 & I3 & I4, E2 extends Env = E, E3 extends Env = IntersectNonAnyTypes<[
        E,
        E2
    ]>, E4 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3
    ]>, E5 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4
    ]>, E6 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5
    ]>, M1 extends H<E2, P, I> = H<E2, P, I>, M2 extends H<E3, P, I2> = H<E3, P, I2>, M3 extends H<E4, P, I3> = H<E4, P, I3>, M4 extends H<E5, P, I4> = H<E5, P, I4>>(...handlers: [
        H<E2, P, I> & M1,
        H<E3, P, I2> & M2,
        H<E4, P, I3> & M3,
        H<E5, P, I4> & M4,
        H<E6, P, I5, R>
    ]): Hono$1<IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6
    ]>, S & ToSchema<M, P, I5, MergeTypedResponse<R> | MergeMiddlewareResponse<M1> | MergeMiddlewareResponse<M2> | MergeMiddlewareResponse<M3> | MergeMiddlewareResponse<M4>>, BasePath, CurrentPath>;
    <P extends string, MergedPath extends MergePath<BasePath, P>, R extends HandlerResponse<any> = any, I extends Input = BlankInput, I2 extends Input = I, I3 extends Input = I & I2, I4 extends Input = I & I2 & I3, E2 extends Env = E, E3 extends Env = IntersectNonAnyTypes<[
        E,
        E2
    ]>, E4 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3
    ]>, E5 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4
    ]>, M1 extends H<E2, MergedPath, I> = H<E2, MergedPath, I>, M2 extends H<E3, MergedPath, I2> = H<E3, MergedPath, I2>, M3 extends H<E4, MergedPath, I3> = H<E4, MergedPath, I3>>(path: P, ...handlers: [
        H<E2, MergedPath, I> & M1,
        H<E3, MergedPath, I2> & M2,
        H<E4, MergedPath, I3> & M3,
        H<E5, MergedPath, I4, R>
    ]): Hono$1<E, AddSchemaIfHasResponse<MergeTypedResponse<R> | MergeMiddlewareResponse<M1> | MergeMiddlewareResponse<M2> | MergeMiddlewareResponse<M3>, S, M, P, I4, BasePath>, BasePath, MergePath<BasePath, P>>;
    <P extends string = CurrentPath, R extends HandlerResponse<any> = any, I extends Input = BlankInput, I2 extends Input = I, I3 extends Input = I & I2, I4 extends Input = I & I2 & I3, I5 extends Input = I & I2 & I3 & I4, I6 extends Input = I & I2 & I3 & I4 & I5, E2 extends Env = E, E3 extends Env = IntersectNonAnyTypes<[
        E,
        E2
    ]>, E4 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3
    ]>, E5 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4
    ]>, E6 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5
    ]>, E7 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6
    ]>, M1 extends H<E2, P, I> = H<E2, P, I>, M2 extends H<E3, P, I2> = H<E3, P, I2>, M3 extends H<E4, P, I3> = H<E4, P, I3>, M4 extends H<E5, P, I4> = H<E5, P, I4>, M5 extends H<E6, P, I5> = H<E6, P, I5>>(...handlers: [
        H<E2, P, I> & M1,
        H<E3, P, I2> & M2,
        H<E4, P, I3> & M3,
        H<E5, P, I4> & M4,
        H<E6, P, I5> & M5,
        H<E7, P, I6, R>
    ]): Hono$1<IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7
    ]>, S & ToSchema<M, P, I6, MergeTypedResponse<R> | MergeMiddlewareResponse<M1> | MergeMiddlewareResponse<M2> | MergeMiddlewareResponse<M3> | MergeMiddlewareResponse<M4> | MergeMiddlewareResponse<M5>>, BasePath, CurrentPath>;
    <P extends string, MergedPath extends MergePath<BasePath, P>, R extends HandlerResponse<any> = any, I extends Input = BlankInput, I2 extends Input = I, I3 extends Input = I & I2, I4 extends Input = I & I2 & I3, I5 extends Input = I & I2 & I3 & I4, E2 extends Env = E, E3 extends Env = IntersectNonAnyTypes<[
        E,
        E2
    ]>, E4 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3
    ]>, E5 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4
    ]>, E6 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5
    ]>, M1 extends H<E2, MergedPath, I> = H<E2, MergedPath, I>, M2 extends H<E3, MergedPath, I2> = H<E3, MergedPath, I2>, M3 extends H<E4, MergedPath, I3> = H<E4, MergedPath, I3>, M4 extends H<E5, MergedPath, I4> = H<E5, MergedPath, I4>>(path: P, ...handlers: [
        H<E2, MergedPath, I> & M1,
        H<E3, MergedPath, I2> & M2,
        H<E4, MergedPath, I3> & M3,
        H<E5, MergedPath, I4> & M4,
        H<E6, MergedPath, I5, R>
    ]): Hono$1<E, AddSchemaIfHasResponse<MergeTypedResponse<R> | MergeMiddlewareResponse<M1> | MergeMiddlewareResponse<M2> | MergeMiddlewareResponse<M3> | MergeMiddlewareResponse<M4>, S, M, P, I5, BasePath>, BasePath, MergePath<BasePath, P>>;
    <P extends string = CurrentPath, R extends HandlerResponse<any> = any, I extends Input = BlankInput, I2 extends Input = I, I3 extends Input = I & I2, I4 extends Input = I & I2 & I3, I5 extends Input = I & I2 & I3 & I4, I6 extends Input = I & I2 & I3 & I4 & I5, I7 extends Input = I & I2 & I3 & I4 & I5 & I6, E2 extends Env = E, E3 extends Env = IntersectNonAnyTypes<[
        E,
        E2
    ]>, E4 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3
    ]>, E5 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4
    ]>, E6 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5
    ]>, E7 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6
    ]>, E8 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7
    ]>, M1 extends H<E2, P, I> = H<E2, P, I>, M2 extends H<E3, P, I2> = H<E3, P, I2>, M3 extends H<E4, P, I3> = H<E4, P, I3>, M4 extends H<E5, P, I4> = H<E5, P, I4>, M5 extends H<E6, P, I5> = H<E6, P, I5>, M6 extends H<E7, P, I6> = H<E7, P, I6>>(...handlers: [
        H<E2, P, I> & M1,
        H<E3, P, I2> & M2,
        H<E4, P, I3> & M3,
        H<E5, P, I4> & M4,
        H<E6, P, I5> & M5,
        H<E7, P, I6> & M6,
        H<E8, P, I7, R>
    ]): Hono$1<IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7,
        E8
    ]>, S & ToSchema<M, P, I7, MergeTypedResponse<R> | MergeMiddlewareResponse<M1> | MergeMiddlewareResponse<M2> | MergeMiddlewareResponse<M3> | MergeMiddlewareResponse<M4> | MergeMiddlewareResponse<M5> | MergeMiddlewareResponse<M6>>, BasePath, CurrentPath>;
    <P extends string, MergedPath extends MergePath<BasePath, P>, R extends HandlerResponse<any> = any, I extends Input = BlankInput, I2 extends Input = I, I3 extends Input = I & I2, I4 extends Input = I & I2 & I3, I5 extends Input = I & I2 & I3 & I4, I6 extends Input = I & I2 & I3 & I4 & I5, E2 extends Env = E, E3 extends Env = IntersectNonAnyTypes<[
        E,
        E2
    ]>, E4 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3
    ]>, E5 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4
    ]>, E6 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5
    ]>, E7 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6
    ]>, M1 extends H<E2, MergedPath, I> = H<E2, MergedPath, I>, M2 extends H<E3, MergedPath, I2> = H<E3, MergedPath, I2>, M3 extends H<E4, MergedPath, I3> = H<E4, MergedPath, I3>, M4 extends H<E5, MergedPath, I4> = H<E5, MergedPath, I4>, M5 extends H<E6, MergedPath, I5> = H<E6, MergedPath, I5>>(path: P, ...handlers: [
        H<E2, MergedPath, I> & M1,
        H<E3, MergedPath, I2> & M2,
        H<E4, MergedPath, I3> & M3,
        H<E5, MergedPath, I4> & M4,
        H<E6, MergedPath, I5> & M5,
        H<E7, MergedPath, I6, R>
    ]): Hono$1<E, AddSchemaIfHasResponse<MergeTypedResponse<R> | MergeMiddlewareResponse<M1> | MergeMiddlewareResponse<M2> | MergeMiddlewareResponse<M3> | MergeMiddlewareResponse<M4> | MergeMiddlewareResponse<M5>, S, M, P, I6, BasePath>, BasePath, MergePath<BasePath, P>>;
    <P extends string = CurrentPath, R extends HandlerResponse<any> = any, I extends Input = BlankInput, I2 extends Input = I, I3 extends Input = I & I2, I4 extends Input = I & I2 & I3, I5 extends Input = I & I2 & I3 & I4, I6 extends Input = I & I2 & I3 & I4 & I5, I7 extends Input = I & I2 & I3 & I4 & I5 & I6, I8 extends Input = I & I2 & I3 & I4 & I5 & I6 & I7, E2 extends Env = E, E3 extends Env = IntersectNonAnyTypes<[
        E,
        E2
    ]>, E4 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3
    ]>, E5 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4
    ]>, E6 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5
    ]>, E7 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6
    ]>, E8 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7
    ]>, E9 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7,
        E8
    ]>, M1 extends H<E2, P, I> = H<E2, P, I>, M2 extends H<E3, P, I2> = H<E3, P, I2>, M3 extends H<E4, P, I3> = H<E4, P, I3>, M4 extends H<E5, P, I4> = H<E5, P, I4>, M5 extends H<E6, P, I5> = H<E6, P, I5>, M6 extends H<E7, P, I6> = H<E7, P, I6>, M7 extends H<E8, P, I7> = H<E8, P, I7>>(...handlers: [
        H<E2, P, I> & M1,
        H<E3, P, I2> & M2,
        H<E4, P, I3> & M3,
        H<E5, P, I4> & M4,
        H<E6, P, I5> & M5,
        H<E7, P, I6> & M6,
        H<E8, P, I7> & M7,
        H<E9, P, I8, R>
    ]): Hono$1<IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7,
        E8,
        E9
    ]>, S & ToSchema<M, P, I8, MergeTypedResponse<R> | MergeMiddlewareResponse<M1> | MergeMiddlewareResponse<M2> | MergeMiddlewareResponse<M3> | MergeMiddlewareResponse<M4> | MergeMiddlewareResponse<M5> | MergeMiddlewareResponse<M6> | MergeMiddlewareResponse<M7>>, BasePath, CurrentPath>;
    <P extends string, MergedPath extends MergePath<BasePath, P>, R extends HandlerResponse<any> = any, I extends Input = BlankInput, I2 extends Input = I, I3 extends Input = I & I2, I4 extends Input = I & I2 & I3, I5 extends Input = I & I2 & I3 & I4, I6 extends Input = I & I2 & I3 & I4 & I5, I7 extends Input = I & I2 & I3 & I4 & I5 & I6, E2 extends Env = E, E3 extends Env = IntersectNonAnyTypes<[
        E,
        E2
    ]>, E4 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3
    ]>, E5 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4
    ]>, E6 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5
    ]>, E7 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6
    ]>, E8 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7
    ]>, M1 extends H<E2, MergedPath, I> = H<E2, MergedPath, I>, M2 extends H<E3, MergedPath, I2> = H<E3, MergedPath, I2>, M3 extends H<E4, MergedPath, I3> = H<E4, MergedPath, I3>, M4 extends H<E5, MergedPath, I4> = H<E5, MergedPath, I4>, M5 extends H<E6, MergedPath, I5> = H<E6, MergedPath, I5>, M6 extends H<E7, MergedPath, I6> = H<E7, MergedPath, I6>>(path: P, ...handlers: [
        H<E2, MergedPath, I> & M1,
        H<E3, MergedPath, I2> & M2,
        H<E4, MergedPath, I3> & M3,
        H<E5, MergedPath, I4> & M4,
        H<E6, MergedPath, I5> & M5,
        H<E7, MergedPath, I6> & M6,
        H<E8, MergedPath, I7, R>
    ]): Hono$1<E, AddSchemaIfHasResponse<MergeTypedResponse<R> | MergeMiddlewareResponse<M1> | MergeMiddlewareResponse<M2> | MergeMiddlewareResponse<M3> | MergeMiddlewareResponse<M4> | MergeMiddlewareResponse<M5> | MergeMiddlewareResponse<M6>, S, M, P, I7, BasePath>, BasePath, MergePath<BasePath, P>>;
    <P extends string = CurrentPath, R extends HandlerResponse<any> = any, I extends Input = BlankInput, I2 extends Input = I, I3 extends Input = I & I2, I4 extends Input = I & I2 & I3, I5 extends Input = I & I2 & I3 & I4, I6 extends Input = I & I2 & I3 & I4 & I5, I7 extends Input = I & I2 & I3 & I4 & I5 & I6, I8 extends Input = I & I2 & I3 & I4 & I5 & I6 & I7, I9 extends Input = I & I2 & I3 & I4 & I5 & I6 & I7 & I8, E2 extends Env = E, E3 extends Env = IntersectNonAnyTypes<[
        E,
        E2
    ]>, E4 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3
    ]>, E5 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4
    ]>, E6 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5
    ]>, E7 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6
    ]>, E8 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7
    ]>, E9 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7,
        E8
    ]>, E10 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7,
        E8,
        E9
    ]>, M1 extends H<E2, P, I> = H<E2, P, I>, M2 extends H<E3, P, I2> = H<E3, P, I2>, M3 extends H<E4, P, I3> = H<E4, P, I3>, M4 extends H<E5, P, I4> = H<E5, P, I4>, M5 extends H<E6, P, I5> = H<E6, P, I5>, M6 extends H<E7, P, I6> = H<E7, P, I6>, M7 extends H<E8, P, I7> = H<E8, P, I7>, M8 extends H<E9, P, I8> = H<E9, P, I8>>(...handlers: [
        H<E2, P, I> & M1,
        H<E3, P, I2> & M2,
        H<E4, P, I3> & M3,
        H<E5, P, I4> & M4,
        H<E6, P, I5> & M5,
        H<E7, P, I6> & M6,
        H<E8, P, I7> & M7,
        H<E9, P, I8> & M8,
        H<E10, P, I9, R>
    ]): Hono$1<IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7,
        E8,
        E9,
        E10
    ]>, S & ToSchema<M, P, I9, MergeTypedResponse<R> | MergeMiddlewareResponse<M1> | MergeMiddlewareResponse<M2> | MergeMiddlewareResponse<M3> | MergeMiddlewareResponse<M4> | MergeMiddlewareResponse<M5> | MergeMiddlewareResponse<M6> | MergeMiddlewareResponse<M7> | MergeMiddlewareResponse<M8>>, BasePath, CurrentPath>;
    <P extends string, MergedPath extends MergePath<BasePath, P>, R extends HandlerResponse<any> = any, I extends Input = BlankInput, I2 extends Input = I, I3 extends Input = I & I2, I4 extends Input = I & I2 & I3, I5 extends Input = I & I2 & I3 & I4, I6 extends Input = I & I2 & I3 & I4 & I5, I7 extends Input = I & I2 & I3 & I4 & I5 & I6, I8 extends Input = I & I2 & I3 & I4 & I5 & I6 & I7, E2 extends Env = E, E3 extends Env = IntersectNonAnyTypes<[
        E,
        E2
    ]>, E4 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3
    ]>, E5 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4
    ]>, E6 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5
    ]>, E7 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6
    ]>, E8 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7
    ]>, E9 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7,
        E8
    ]>, M1 extends H<E2, MergedPath, I> = H<E2, MergedPath, I>, M2 extends H<E3, MergedPath, I2> = H<E3, MergedPath, I2>, M3 extends H<E4, MergedPath, I3> = H<E4, MergedPath, I3>, M4 extends H<E5, MergedPath, I4> = H<E5, MergedPath, I4>, M5 extends H<E6, MergedPath, I5> = H<E6, MergedPath, I5>, M6 extends H<E7, MergedPath, I6> = H<E7, MergedPath, I6>, M7 extends H<E8, MergedPath, I7> = H<E8, MergedPath, I7>>(path: P, ...handlers: [
        H<E2, MergedPath, I> & M1,
        H<E3, MergedPath, I2> & M2,
        H<E4, MergedPath, I3> & M3,
        H<E5, MergedPath, I4> & M4,
        H<E6, MergedPath, I5> & M5,
        H<E7, MergedPath, I6> & M6,
        H<E8, MergedPath, I7> & M7,
        H<E9, MergedPath, I8, R>
    ]): Hono$1<E, AddSchemaIfHasResponse<MergeTypedResponse<R> | MergeMiddlewareResponse<M1> | MergeMiddlewareResponse<M2> | MergeMiddlewareResponse<M3> | MergeMiddlewareResponse<M4> | MergeMiddlewareResponse<M5> | MergeMiddlewareResponse<M6> | MergeMiddlewareResponse<M7>, S, M, P, I8, BasePath>, BasePath, MergePath<BasePath, P>>;
    <P extends string = CurrentPath, R extends HandlerResponse<any> = any, I extends Input = BlankInput, I2 extends Input = I, I3 extends Input = I & I2, I4 extends Input = I & I2 & I3, I5 extends Input = I & I2 & I3 & I4, I6 extends Input = I & I2 & I3 & I4 & I5, I7 extends Input = I & I2 & I3 & I4 & I5 & I6, I8 extends Input = I & I2 & I3 & I4 & I5 & I6 & I7, I9 extends Input = I & I2 & I3 & I4 & I5 & I6 & I7 & I8, I10 extends Input = I & I2 & I3 & I4 & I5 & I6 & I7 & I8 & I9, E2 extends Env = E, E3 extends Env = IntersectNonAnyTypes<[
        E,
        E2
    ]>, E4 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3
    ]>, E5 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4
    ]>, E6 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5
    ]>, E7 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6
    ]>, E8 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7
    ]>, E9 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7,
        E8
    ]>, E10 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7,
        E8,
        E9
    ]>, E11 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7,
        E8,
        E9,
        E10
    ]>, M1 extends H<E2, P, I> = H<E2, P, I>, M2 extends H<E3, P, I2> = H<E3, P, I2>, M3 extends H<E4, P, I3> = H<E4, P, I3>, M4 extends H<E5, P, I4> = H<E5, P, I4>, M5 extends H<E6, P, I5> = H<E6, P, I5>, M6 extends H<E7, P, I6> = H<E7, P, I6>, M7 extends H<E8, P, I7> = H<E8, P, I7>, M8 extends H<E9, P, I8> = H<E9, P, I8>, M9 extends H<E10, P, I9> = H<E10, P, I9>>(...handlers: [
        H<E2, P, I> & M1,
        H<E3, P, I2> & M2,
        H<E4, P, I3> & M3,
        H<E5, P, I4> & M4,
        H<E6, P, I5> & M5,
        H<E7, P, I6> & M6,
        H<E8, P, I7> & M7,
        H<E9, P, I8> & M8,
        H<E10, P, I9> & M9,
        H<E11, P, I10, R>
    ]): Hono$1<IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7,
        E8,
        E9,
        E10,
        E11
    ]>, S & ToSchema<M, P, I10, MergeTypedResponse<R> | MergeMiddlewareResponse<M1> | MergeMiddlewareResponse<M2> | MergeMiddlewareResponse<M3> | MergeMiddlewareResponse<M4> | MergeMiddlewareResponse<M5> | MergeMiddlewareResponse<M6> | MergeMiddlewareResponse<M7> | MergeMiddlewareResponse<M8> | MergeMiddlewareResponse<M9>>, BasePath, CurrentPath>;
    <P extends string, MergedPath extends MergePath<BasePath, P>, R extends HandlerResponse<any> = any, I extends Input = BlankInput, I2 extends Input = I, I3 extends Input = I & I2, I4 extends Input = I & I2 & I3, I5 extends Input = I & I2 & I3 & I4, I6 extends Input = I & I2 & I3 & I4 & I5, I7 extends Input = I & I2 & I3 & I4 & I5 & I6, I8 extends Input = I & I2 & I3 & I4 & I5 & I6 & I7, I9 extends Input = I & I2 & I3 & I4 & I5 & I6 & I7 & I8, E2 extends Env = E, E3 extends Env = IntersectNonAnyTypes<[
        E,
        E2
    ]>, E4 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3
    ]>, E5 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4
    ]>, E6 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5
    ]>, E7 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6
    ]>, E8 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7
    ]>, E9 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7,
        E8
    ]>, E10 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7,
        E8,
        E9
    ]>, M1 extends H<E2, MergedPath, I> = H<E2, MergedPath, I>, M2 extends H<E3, MergedPath, I2> = H<E3, MergedPath, I2>, M3 extends H<E4, MergedPath, I3> = H<E4, MergedPath, I3>, M4 extends H<E5, MergedPath, I4> = H<E5, MergedPath, I4>, M5 extends H<E6, MergedPath, I5> = H<E6, MergedPath, I5>, M6 extends H<E7, MergedPath, I6> = H<E7, MergedPath, I6>, M7 extends H<E8, MergedPath, I7> = H<E8, MergedPath, I7>, M8 extends H<E9, MergedPath, I8> = H<E9, MergedPath, I8>>(path: P, ...handlers: [
        H<E2, MergedPath, I> & M1,
        H<E3, MergedPath, I2> & M2,
        H<E4, MergedPath, I3> & M3,
        H<E5, MergedPath, I4> & M4,
        H<E6, MergedPath, I5> & M5,
        H<E7, MergedPath, I6> & M6,
        H<E8, MergedPath, I7> & M7,
        H<E9, MergedPath, I8> & M8,
        H<E10, MergedPath, I9, R>
    ]): Hono$1<E, AddSchemaIfHasResponse<MergeTypedResponse<R> | MergeMiddlewareResponse<M1> | MergeMiddlewareResponse<M2> | MergeMiddlewareResponse<M3> | MergeMiddlewareResponse<M4> | MergeMiddlewareResponse<M5> | MergeMiddlewareResponse<M6> | MergeMiddlewareResponse<M7> | MergeMiddlewareResponse<M8>, S, M, P, I9, BasePath>, BasePath, MergePath<BasePath, P>>;
    <P extends string, MergedPath extends MergePath<BasePath, P>, R extends HandlerResponse<any> = any, I extends Input = BlankInput, I2 extends Input = I, I3 extends Input = I & I2, I4 extends Input = I & I2 & I3, I5 extends Input = I & I2 & I3 & I4, I6 extends Input = I & I2 & I3 & I4 & I5, I7 extends Input = I & I2 & I3 & I4 & I5 & I6, I8 extends Input = I & I2 & I3 & I4 & I5 & I6 & I7, I9 extends Input = I & I2 & I3 & I4 & I5 & I6 & I7 & I8, I10 extends Input = I & I2 & I3 & I4 & I5 & I6 & I7 & I8 & I9, E2 extends Env = E, E3 extends Env = IntersectNonAnyTypes<[
        E,
        E2
    ]>, E4 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3
    ]>, E5 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4
    ]>, E6 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5
    ]>, E7 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6
    ]>, E8 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7
    ]>, E9 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7,
        E8
    ]>, E10 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7,
        E8,
        E9
    ]>, E11 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7,
        E8,
        E9,
        E10
    ]>, M1 extends H<E2, MergedPath, I> = H<E2, MergedPath, I>, M2 extends H<E3, MergedPath, I2> = H<E3, MergedPath, I2>, M3 extends H<E4, MergedPath, I3> = H<E4, MergedPath, I3>, M4 extends H<E5, MergedPath, I4> = H<E5, MergedPath, I4>, M5 extends H<E6, MergedPath, I5> = H<E6, MergedPath, I5>, M6 extends H<E7, MergedPath, I6> = H<E7, MergedPath, I6>, M7 extends H<E8, MergedPath, I7> = H<E8, MergedPath, I7>, M8 extends H<E9, MergedPath, I8> = H<E9, MergedPath, I8>, M9 extends H<E10, MergedPath, I9> = H<E10, MergedPath, I9>>(path: P, ...handlers: [
        H<E2, MergedPath, I> & M1,
        H<E3, MergedPath, I2> & M2,
        H<E4, MergedPath, I3> & M3,
        H<E5, MergedPath, I4> & M4,
        H<E6, MergedPath, I5> & M5,
        H<E7, MergedPath, I6> & M6,
        H<E8, MergedPath, I7> & M7,
        H<E9, MergedPath, I8> & M8,
        H<E10, MergedPath, I9> & M9,
        H<E11, MergedPath, I10, R>
    ]): Hono$1<E, AddSchemaIfHasResponse<MergeTypedResponse<R> | MergeMiddlewareResponse<M1> | MergeMiddlewareResponse<M2> | MergeMiddlewareResponse<M3> | MergeMiddlewareResponse<M4> | MergeMiddlewareResponse<M5> | MergeMiddlewareResponse<M6> | MergeMiddlewareResponse<M7> | MergeMiddlewareResponse<M8> | MergeMiddlewareResponse<M9>, S, M, P, I10, BasePath>, BasePath, MergePath<BasePath, P>>;
    <P extends string = CurrentPath, I extends Input = BlankInput, R extends HandlerResponse<any> = any>(...handlers: H<E, P, I, R>[]): Hono$1<E, S & ToSchema<M, P, I, MergeTypedResponse<R>>, BasePath, CurrentPath>;
    <P extends string, I extends Input = BlankInput, R extends HandlerResponse<any> = any>(path: P, ...handlers: [
        H<E, MergePath<BasePath, P>, I, R>,
        ...H<E, MergePath<BasePath, P>, I, R>[]
    ]): Hono$1<E, S & ToSchema<M, MergePath<BasePath, P>, I, MergeTypedResponse<R>>, BasePath, MergePath<BasePath, P>>;
    <P extends string, R extends HandlerResponse<any> = any, I extends Input = BlankInput>(path: P): Hono$1<E, S & ToSchema<M, MergePath<BasePath, P>, I, MergeTypedResponse<R>>, BasePath, MergePath<BasePath, P>>;
}
interface MiddlewareHandlerInterface<E extends Env = Env, S extends Schema = BlankSchema, BasePath extends string = '/'> {
    <E2 extends Env = E>(...handlers: MiddlewareHandler<E2, MergePath<BasePath, '*'>>[]): Hono$1<IntersectNonAnyTypes<[
        E,
        E2
    ]>, S, BasePath, MergePath<BasePath, '*'>>;
    <E2 extends Env = E>(handler: MiddlewareHandler<E2, MergePath<BasePath, '*'>>): Hono$1<IntersectNonAnyTypes<[
        E,
        E2
    ]>, S, BasePath, MergePath<BasePath, '*'>>;
    <E2 extends Env = E, E3 extends Env = IntersectNonAnyTypes<[
        E,
        E2
    ]>, P extends string = MergePath<BasePath, '*'>>(...handlers: [
        MiddlewareHandler<E2, P>,
        MiddlewareHandler<E3, P>
    ]): Hono$1<IntersectNonAnyTypes<[
        E,
        E2,
        E3
    ]>, S, BasePath, P>;
    <P extends string, MergedPath extends MergePath<BasePath, P>, E2 extends Env = E>(path: P, handler: MiddlewareHandler<E2, MergedPath, any, any>): Hono$1<IntersectNonAnyTypes<[
        E,
        E2
    ]>, S, BasePath, MergedPath>;
    <E2 extends Env = E, E3 extends Env = IntersectNonAnyTypes<[
        E,
        E2
    ]>, E4 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3
    ]>, P extends string = MergePath<BasePath, '*'>>(...handlers: [
        MiddlewareHandler<E2, P, any, any>,
        MiddlewareHandler<E3, P, any, any>,
        MiddlewareHandler<E4, P, any, any>
    ]): Hono$1<IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4
    ]>, S, BasePath, P>;
    <P extends string, MergedPath extends MergePath<BasePath, P>, E2 extends Env = E, E3 extends Env = IntersectNonAnyTypes<[
        E,
        E2
    ]>>(path: P, ...handlers: [
        MiddlewareHandler<E2, P>,
        MiddlewareHandler<E3, P>
    ]): Hono$1<IntersectNonAnyTypes<[
        E,
        E2,
        E3
    ]>, S, BasePath, MergedPath>;
    <E2 extends Env = E, E3 extends Env = IntersectNonAnyTypes<[
        E,
        E2
    ]>, E4 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3
    ]>, E5 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4
    ]>, P extends string = MergePath<BasePath, '*'>>(...handlers: [
        MiddlewareHandler<E2, P>,
        MiddlewareHandler<E3, P>,
        MiddlewareHandler<E4, P>,
        MiddlewareHandler<E5, P>
    ]): Hono$1<IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5
    ]>, S, BasePath, P>;
    <P extends string, MergedPath extends MergePath<BasePath, P>, E2 extends Env = E, E3 extends Env = IntersectNonAnyTypes<[
        E,
        E2
    ]>, E4 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3
    ]>>(path: P, ...handlers: [
        MiddlewareHandler<E2, P>,
        MiddlewareHandler<E3, P>,
        MiddlewareHandler<E4, P>
    ]): Hono$1<IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4
    ]>, S, BasePath, MergedPath>;
    <E2 extends Env = E, E3 extends Env = IntersectNonAnyTypes<[
        E,
        E2
    ]>, E4 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3
    ]>, E5 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4
    ]>, E6 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5
    ]>, P extends string = MergePath<BasePath, '*'>>(...handlers: [
        MiddlewareHandler<E2, P>,
        MiddlewareHandler<E3, P>,
        MiddlewareHandler<E4, P>,
        MiddlewareHandler<E5, P>,
        MiddlewareHandler<E6, P>
    ]): Hono$1<IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6
    ]>, S, BasePath, P>;
    <P extends string, MergedPath extends MergePath<BasePath, P>, E2 extends Env = E, E3 extends Env = IntersectNonAnyTypes<[
        E,
        E2
    ]>, E4 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3
    ]>, E5 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4
    ]>>(path: P, ...handlers: [
        MiddlewareHandler<E2, P>,
        MiddlewareHandler<E3, P>,
        MiddlewareHandler<E4, P>,
        MiddlewareHandler<E5, P>
    ]): Hono$1<IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5
    ]>, S, BasePath, MergedPath>;
    <E2 extends Env = E, E3 extends Env = IntersectNonAnyTypes<[
        E,
        E2
    ]>, E4 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3
    ]>, E5 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4
    ]>, E6 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5
    ]>, E7 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6
    ]>, P extends string = MergePath<BasePath, '*'>>(...handlers: [
        MiddlewareHandler<E2, P>,
        MiddlewareHandler<E3, P>,
        MiddlewareHandler<E4, P>,
        MiddlewareHandler<E5, P>,
        MiddlewareHandler<E6, P>,
        MiddlewareHandler<E7, P>
    ]): Hono$1<IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7
    ]>, S, BasePath, P>;
    <P extends string, MergedPath extends MergePath<BasePath, P>, E2 extends Env = E, E3 extends Env = IntersectNonAnyTypes<[
        E,
        E2
    ]>, E4 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3
    ]>, E5 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4
    ]>, E6 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5
    ]>>(path: P, ...handlers: [
        MiddlewareHandler<E2, P>,
        MiddlewareHandler<E3, P>,
        MiddlewareHandler<E4, P>,
        MiddlewareHandler<E5, P>,
        MiddlewareHandler<E6, P>
    ]): Hono$1<IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6
    ]>, S, BasePath, MergedPath>;
    <E2 extends Env = E, E3 extends Env = IntersectNonAnyTypes<[
        E,
        E2
    ]>, E4 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3
    ]>, E5 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4
    ]>, E6 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5
    ]>, E7 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6
    ]>, E8 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7
    ]>, P extends string = MergePath<BasePath, '*'>>(...handlers: [
        MiddlewareHandler<E2, P>,
        MiddlewareHandler<E3, P>,
        MiddlewareHandler<E4, P>,
        MiddlewareHandler<E5, P>,
        MiddlewareHandler<E6, P>,
        MiddlewareHandler<E7, P>,
        MiddlewareHandler<E8, P>
    ]): Hono$1<IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7,
        E8
    ]>, S, BasePath, P>;
    <P extends string, MergedPath extends MergePath<BasePath, P>, E2 extends Env = E, E3 extends Env = IntersectNonAnyTypes<[
        E,
        E2
    ]>, E4 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3
    ]>, E5 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4
    ]>, E6 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5
    ]>, E7 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6
    ]>>(path: P, ...handlers: [
        MiddlewareHandler<E2, P>,
        MiddlewareHandler<E3, P>,
        MiddlewareHandler<E4, P>,
        MiddlewareHandler<E5, P>,
        MiddlewareHandler<E6, P>,
        MiddlewareHandler<E7, P>
    ]): Hono$1<IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7
    ]>, S, BasePath, MergedPath>;
    <E2 extends Env = E, E3 extends Env = IntersectNonAnyTypes<[
        E,
        E2
    ]>, E4 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3
    ]>, E5 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4
    ]>, E6 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5
    ]>, E7 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6
    ]>, E8 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7
    ]>, E9 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7,
        E8
    ]>, P extends string = MergePath<BasePath, '*'>>(...handlers: [
        MiddlewareHandler<E2, P>,
        MiddlewareHandler<E3, P>,
        MiddlewareHandler<E4, P>,
        MiddlewareHandler<E5, P>,
        MiddlewareHandler<E6, P>,
        MiddlewareHandler<E7, P>,
        MiddlewareHandler<E8, P>,
        MiddlewareHandler<E9, P>
    ]): Hono$1<IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7,
        E8,
        E9
    ]>, S, BasePath, P>;
    <P extends string, MergedPath extends MergePath<BasePath, P>, E2 extends Env = E, E3 extends Env = IntersectNonAnyTypes<[
        E,
        E2
    ]>, E4 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3
    ]>, E5 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4
    ]>, E6 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5
    ]>, E7 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6
    ]>, E8 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7
    ]>>(path: P, ...handlers: [
        MiddlewareHandler<E2, P>,
        MiddlewareHandler<E3, P>,
        MiddlewareHandler<E4, P>,
        MiddlewareHandler<E5, P>,
        MiddlewareHandler<E6, P>,
        MiddlewareHandler<E7, P>,
        MiddlewareHandler<E8, P>
    ]): Hono$1<IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7,
        E8
    ]>, S, BasePath, MergedPath>;
    <E2 extends Env = E, E3 extends Env = IntersectNonAnyTypes<[
        E,
        E2
    ]>, E4 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3
    ]>, E5 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4
    ]>, E6 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5
    ]>, E7 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6
    ]>, E8 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7
    ]>, E9 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7,
        E8
    ]>, E10 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7,
        E8,
        E9
    ]>, P extends string = MergePath<BasePath, '*'>>(...handlers: [
        MiddlewareHandler<E2, P>,
        MiddlewareHandler<E3, P>,
        MiddlewareHandler<E4, P>,
        MiddlewareHandler<E5, P>,
        MiddlewareHandler<E6, P>,
        MiddlewareHandler<E7, P>,
        MiddlewareHandler<E8, P>,
        MiddlewareHandler<E9, P>,
        MiddlewareHandler<E10, P>
    ]): Hono$1<IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7,
        E8,
        E9,
        E10
    ]>, S, BasePath, P>;
    <P extends string, MergedPath extends MergePath<BasePath, P>, E2 extends Env = E, E3 extends Env = IntersectNonAnyTypes<[
        E,
        E2
    ]>, E4 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3
    ]>, E5 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4
    ]>, E6 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5
    ]>, E7 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6
    ]>, E8 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7
    ]>, E9 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7,
        E8
    ]>>(path: P, ...handlers: [
        MiddlewareHandler<E2, P>,
        MiddlewareHandler<E3, P>,
        MiddlewareHandler<E4, P>,
        MiddlewareHandler<E5, P>,
        MiddlewareHandler<E6, P>,
        MiddlewareHandler<E7, P>,
        MiddlewareHandler<E8, P>,
        MiddlewareHandler<E9, P>
    ]): Hono$1<IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7,
        E8,
        E9
    ]>, S, BasePath, MergedPath>;
    <E2 extends Env = E, E3 extends Env = IntersectNonAnyTypes<[
        E,
        E2
    ]>, E4 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3
    ]>, E5 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4
    ]>, E6 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5
    ]>, E7 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6
    ]>, E8 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7
    ]>, E9 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7,
        E8
    ]>, E10 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7,
        E8,
        E9
    ]>, E11 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7,
        E8,
        E9,
        E10
    ]>, P extends string = MergePath<BasePath, '*'>>(...handlers: [
        MiddlewareHandler<E2, P>,
        MiddlewareHandler<E3, P>,
        MiddlewareHandler<E4, P>,
        MiddlewareHandler<E5, P>,
        MiddlewareHandler<E6, P>,
        MiddlewareHandler<E7, P>,
        MiddlewareHandler<E8, P>,
        MiddlewareHandler<E9, P>,
        MiddlewareHandler<E10, P>,
        MiddlewareHandler<E11, P>
    ]): Hono$1<IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7,
        E8,
        E9,
        E10,
        E11
    ]>, S, BasePath, P>;
    <P extends string, MergedPath extends MergePath<BasePath, P>, E2 extends Env = E, E3 extends Env = IntersectNonAnyTypes<[
        E,
        E2
    ]>, E4 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3
    ]>, E5 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4
    ]>, E6 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5
    ]>, E7 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6
    ]>, E8 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7
    ]>, E9 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7,
        E8
    ]>, E10 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7,
        E8,
        E9
    ]>>(path: P, ...handlers: [
        MiddlewareHandler<E2, P>,
        MiddlewareHandler<E3, P>,
        MiddlewareHandler<E4, P>,
        MiddlewareHandler<E5, P>,
        MiddlewareHandler<E6, P>,
        MiddlewareHandler<E7, P>,
        MiddlewareHandler<E8, P>,
        MiddlewareHandler<E9, P>,
        MiddlewareHandler<E10, P>
    ]): Hono$1<IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7,
        E8,
        E9,
        E10
    ]>, S, BasePath, MergedPath>;
    <P extends string, E2 extends Env = E>(path: P, ...handlers: MiddlewareHandler<E2, MergePath<BasePath, P>>[]): Hono$1<E, S, BasePath, MergePath<BasePath, P>>;
}
interface OnHandlerInterface<E extends Env = Env, S extends Schema = BlankSchema, BasePath extends string = '/'> {
    <M extends string, P extends string, MergedPath extends MergePath<BasePath, P>, R extends HandlerResponse<any> = any, I extends Input = BlankInput, E2 extends Env = E>(method: M, path: P, handler: H<E2, MergedPath, I, R>): Hono$1<IntersectNonAnyTypes<[
        E,
        E2
    ]>, S & ToSchema<M, MergePath<BasePath, P>, I, MergeTypedResponse<R>>, BasePath, MergePath<BasePath, P>>;
    <M extends string, P extends string, MergedPath extends MergePath<BasePath, P>, R extends HandlerResponse<any> = any, I extends Input = BlankInput, I2 extends Input = I, E2 extends Env = E, E3 extends Env = IntersectNonAnyTypes<[
        E,
        E2
    ]>, M1 extends H<E2, MergedPath, I> = H<E2, MergedPath, I>>(method: M, path: P, ...handlers: [
        H<E2, MergedPath, I> & M1,
        H<E3, MergedPath, I2, R>
    ]): Hono$1<IntersectNonAnyTypes<[
        E,
        E2,
        E3
    ]>, S & ToSchema<M, MergePath<BasePath, P>, I2, MergeTypedResponse<R> | MergeMiddlewareResponse<M1>>, BasePath, MergePath<BasePath, P>>;
    <M extends string, P extends string, MergedPath extends MergePath<BasePath, P>, R extends HandlerResponse<any> = any, I extends Input = BlankInput, I2 extends Input = I, I3 extends Input = I & I2, E2 extends Env = E, E3 extends Env = IntersectNonAnyTypes<[
        E,
        E2
    ]>, E4 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3
    ]>, M1 extends H<E2, MergedPath, I> = H<E2, MergedPath, I>, M2 extends H<E3, MergedPath, I2> = H<E3, MergedPath, I2>>(method: M, path: P, ...handlers: [
        H<E2, MergedPath, I> & M1,
        H<E3, MergedPath, I2> & M2,
        H<E4, MergedPath, I3, R>
    ]): Hono$1<IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4
    ]>, S & ToSchema<M, MergePath<BasePath, P>, I3, MergeTypedResponse<R> | MergeMiddlewareResponse<M1> | MergeMiddlewareResponse<M2>>, BasePath, MergePath<BasePath, P>>;
    <M extends string, P extends string, MergedPath extends MergePath<BasePath, P>, R extends HandlerResponse<any> = any, I extends Input = BlankInput, I2 extends Input = I, I3 extends Input = I & I2, I4 extends Input = I & I2 & I3, E2 extends Env = E, E3 extends Env = IntersectNonAnyTypes<[
        E,
        E2
    ]>, E4 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3
    ]>, E5 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4
    ]>, M1 extends H<E2, MergedPath, I> = H<E2, MergedPath, I>, M2 extends H<E3, MergedPath, I2> = H<E3, MergedPath, I2>, M3 extends H<E4, MergedPath, I3> = H<E4, MergedPath, I3>>(method: M, path: P, ...handlers: [
        H<E2, MergedPath, I> & M1,
        H<E3, MergedPath, I2> & M2,
        H<E4, MergedPath, I3> & M3,
        H<E5, MergedPath, I4, R>
    ]): Hono$1<IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5
    ]>, S & ToSchema<M, MergePath<BasePath, P>, I4, MergeTypedResponse<R> | MergeMiddlewareResponse<M1> | MergeMiddlewareResponse<M2> | MergeMiddlewareResponse<M3>>, BasePath, MergePath<BasePath, P>>;
    <M extends string, P extends string, MergedPath extends MergePath<BasePath, P>, R extends HandlerResponse<any> = any, I extends Input = BlankInput, I2 extends Input = I, I3 extends Input = I & I2, I4 extends Input = I & I2 & I3, I5 extends Input = I & I2 & I3 & I4, E2 extends Env = E, E3 extends Env = IntersectNonAnyTypes<[
        E,
        E2
    ]>, E4 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3
    ]>, E5 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4
    ]>, E6 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5
    ]>, M1 extends H<E2, MergedPath, I> = H<E2, MergedPath, I>, M2 extends H<E3, MergedPath, I2> = H<E3, MergedPath, I2>, M3 extends H<E4, MergedPath, I3> = H<E4, MergedPath, I3>, M4 extends H<E5, MergedPath, I4> = H<E5, MergedPath, I4>>(method: M, path: P, ...handlers: [
        H<E2, MergedPath, I> & M1,
        H<E3, MergedPath, I2> & M2,
        H<E4, MergedPath, I3> & M3,
        H<E5, MergedPath, I4> & M4,
        H<E6, MergedPath, I5, R>
    ]): Hono$1<IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6
    ]>, S & ToSchema<M, MergePath<BasePath, P>, I5, MergeTypedResponse<R> | MergeMiddlewareResponse<M1> | MergeMiddlewareResponse<M2> | MergeMiddlewareResponse<M3> | MergeMiddlewareResponse<M4>>, BasePath, MergePath<BasePath, P>>;
    <M extends string, P extends string, MergedPath extends MergePath<BasePath, P>, R extends HandlerResponse<any> = any, I extends Input = BlankInput, I2 extends Input = I, I3 extends Input = I & I2, I4 extends Input = I & I2 & I3, I5 extends Input = I & I2 & I3 & I4, I6 extends Input = I & I2 & I3 & I4 & I5, E2 extends Env = E, E3 extends Env = IntersectNonAnyTypes<[
        E,
        E2
    ]>, E4 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3
    ]>, E5 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4
    ]>, E6 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5
    ]>, E7 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6
    ]>, M1 extends H<E2, MergedPath, I> = H<E2, MergedPath, I>, M2 extends H<E3, MergedPath, I2> = H<E3, MergedPath, I2>, M3 extends H<E4, MergedPath, I3> = H<E4, MergedPath, I3>, M4 extends H<E5, MergedPath, I4> = H<E5, MergedPath, I4>, M5 extends H<E6, MergedPath, I5> = H<E6, MergedPath, I5>>(method: M, path: P, ...handlers: [
        H<E2, MergedPath, I> & M1,
        H<E3, MergedPath, I2> & M2,
        H<E4, MergedPath, I3> & M3,
        H<E5, MergedPath, I4> & M4,
        H<E6, MergedPath, I5> & M5,
        H<E7, MergedPath, I6, R>
    ]): Hono$1<IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7
    ]>, S & ToSchema<M, MergePath<BasePath, P>, I6, MergeTypedResponse<R> | MergeMiddlewareResponse<M1> | MergeMiddlewareResponse<M2> | MergeMiddlewareResponse<M3> | MergeMiddlewareResponse<M4> | MergeMiddlewareResponse<M5>>, BasePath, MergePath<BasePath, P>>;
    <M extends string, P extends string, MergedPath extends MergePath<BasePath, P>, R extends HandlerResponse<any> = any, I extends Input = BlankInput, I2 extends Input = I, I3 extends Input = I & I2, I4 extends Input = I & I2 & I3, I5 extends Input = I & I2 & I3 & I4, I6 extends Input = I & I2 & I3 & I4 & I5, I7 extends Input = I & I2 & I3 & I4 & I5 & I6, E2 extends Env = E, E3 extends Env = IntersectNonAnyTypes<[
        E,
        E2
    ]>, E4 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3
    ]>, E5 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4
    ]>, E6 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5
    ]>, E7 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6
    ]>, E8 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7
    ]>, M1 extends H<E2, MergedPath, I> = H<E2, MergedPath, I>, M2 extends H<E3, MergedPath, I2> = H<E3, MergedPath, I2>, M3 extends H<E4, MergedPath, I3> = H<E4, MergedPath, I3>, M4 extends H<E5, MergedPath, I4> = H<E5, MergedPath, I4>, M5 extends H<E6, MergedPath, I5> = H<E6, MergedPath, I5>, M6 extends H<E7, MergedPath, I6> = H<E7, MergedPath, I6>>(method: M, path: P, ...handlers: [
        H<E2, MergedPath, I> & M1,
        H<E3, MergedPath, I2> & M2,
        H<E4, MergedPath, I3> & M3,
        H<E5, MergedPath, I4> & M4,
        H<E6, MergedPath, I5> & M5,
        H<E7, MergedPath, I6> & M6,
        H<E8, MergedPath, I7, R>
    ]): Hono$1<IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7,
        E8
    ]>, S & ToSchema<M, MergePath<BasePath, P>, I7, MergeTypedResponse<R> | MergeMiddlewareResponse<M1> | MergeMiddlewareResponse<M2> | MergeMiddlewareResponse<M3> | MergeMiddlewareResponse<M4> | MergeMiddlewareResponse<M5> | MergeMiddlewareResponse<M6>>, BasePath, MergePath<BasePath, P>>;
    <M extends string, P extends string, MergedPath extends MergePath<BasePath, P>, R extends HandlerResponse<any> = any, I extends Input = BlankInput, I2 extends Input = I, I3 extends Input = I & I2, I4 extends Input = I & I2 & I3, I5 extends Input = I & I2 & I3 & I4, I6 extends Input = I & I2 & I3 & I4 & I5, I7 extends Input = I & I2 & I3 & I4 & I5 & I6, I8 extends Input = I & I2 & I3 & I4 & I5 & I6 & I7, E2 extends Env = E, E3 extends Env = IntersectNonAnyTypes<[
        E,
        E2
    ]>, E4 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3
    ]>, E5 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4
    ]>, E6 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5
    ]>, E7 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6
    ]>, E8 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7
    ]>, E9 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7,
        E8
    ]>, M1 extends H<E2, MergedPath, I> = H<E2, MergedPath, I>, M2 extends H<E3, MergedPath, I2> = H<E3, MergedPath, I2>, M3 extends H<E4, MergedPath, I3> = H<E4, MergedPath, I3>, M4 extends H<E5, MergedPath, I4> = H<E5, MergedPath, I4>, M5 extends H<E6, MergedPath, I5> = H<E6, MergedPath, I5>, M6 extends H<E7, MergedPath, I6> = H<E7, MergedPath, I6>, M7 extends H<E8, MergedPath, I7> = H<E8, MergedPath, I7>>(method: M, path: P, ...handlers: [
        H<E2, MergedPath, I> & M1,
        H<E3, MergedPath, I2> & M2,
        H<E4, MergedPath, I3> & M3,
        H<E5, MergedPath, I4> & M4,
        H<E6, MergedPath, I5> & M5,
        H<E7, MergedPath, I6> & M6,
        H<E8, MergedPath, I7> & M7,
        H<E9, MergedPath, I8, R>
    ]): Hono$1<IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7,
        E8,
        E9
    ]>, S & ToSchema<M, MergePath<BasePath, P>, I8, MergeTypedResponse<R> | MergeMiddlewareResponse<M1> | MergeMiddlewareResponse<M2> | MergeMiddlewareResponse<M3> | MergeMiddlewareResponse<M4> | MergeMiddlewareResponse<M5> | MergeMiddlewareResponse<M6> | MergeMiddlewareResponse<M7>>, BasePath, MergePath<BasePath, P>>;
    <M extends string, P extends string, MergedPath extends MergePath<BasePath, P>, R extends HandlerResponse<any> = any, I extends Input = BlankInput, I2 extends Input = I, I3 extends Input = I & I2, I4 extends Input = I & I2 & I3, I5 extends Input = I & I2 & I3 & I4, I6 extends Input = I & I2 & I3 & I4 & I5, I7 extends Input = I & I2 & I3 & I4 & I5 & I6, I8 extends Input = I & I2 & I3 & I4 & I5 & I6 & I7, I9 extends Input = I & I2 & I3 & I4 & I5 & I6 & I7 & I8, E2 extends Env = E, E3 extends Env = IntersectNonAnyTypes<[
        E,
        E2
    ]>, E4 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3
    ]>, E5 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4
    ]>, E6 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5
    ]>, E7 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6
    ]>, E8 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7
    ]>, E9 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7,
        E8
    ]>, E10 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7,
        E8,
        E9
    ]>, M1 extends H<E2, MergedPath, I> = H<E2, MergedPath, I>, M2 extends H<E3, MergedPath, I2> = H<E3, MergedPath, I2>, M3 extends H<E4, MergedPath, I3> = H<E4, MergedPath, I3>, M4 extends H<E5, MergedPath, I4> = H<E5, MergedPath, I4>, M5 extends H<E6, MergedPath, I5> = H<E6, MergedPath, I5>, M6 extends H<E7, MergedPath, I6> = H<E7, MergedPath, I6>, M7 extends H<E8, MergedPath, I7> = H<E8, MergedPath, I7>, M8 extends H<E9, MergedPath, I8> = H<E9, MergedPath, I8>>(method: M, path: P, ...handlers: [
        H<E2, MergedPath, I> & M1,
        H<E3, MergedPath, I2> & M2,
        H<E4, MergedPath, I3> & M3,
        H<E5, MergedPath, I4> & M4,
        H<E6, MergedPath, I5> & M5,
        H<E7, MergedPath, I6> & M6,
        H<E8, MergedPath, I7> & M7,
        H<E9, MergedPath, I8> & M8,
        H<E10, MergedPath, I9, R>
    ]): Hono$1<IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7,
        E8,
        E9,
        E10
    ]>, S & ToSchema<M, MergePath<BasePath, P>, I9, MergeTypedResponse<R> | MergeMiddlewareResponse<M1> | MergeMiddlewareResponse<M2> | MergeMiddlewareResponse<M3> | MergeMiddlewareResponse<M4> | MergeMiddlewareResponse<M5> | MergeMiddlewareResponse<M6> | MergeMiddlewareResponse<M7> | MergeMiddlewareResponse<M8>>, BasePath, MergePath<BasePath, P>>;
    <M extends string, P extends string, MergedPath extends MergePath<BasePath, P>, R extends HandlerResponse<any> = any, I extends Input = BlankInput, I2 extends Input = I, I3 extends Input = I & I2, I4 extends Input = I & I2 & I3, I5 extends Input = I & I2 & I3 & I4, I6 extends Input = I & I2 & I3 & I4 & I5, I7 extends Input = I & I2 & I3 & I4 & I5 & I6, I8 extends Input = I & I2 & I3 & I4 & I5 & I6 & I7, I9 extends Input = I & I2 & I3 & I4 & I5 & I6 & I7 & I8, I10 extends Input = I & I2 & I3 & I4 & I5 & I6 & I7 & I8 & I9, E2 extends Env = E, E3 extends Env = IntersectNonAnyTypes<[
        E,
        E2
    ]>, E4 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3
    ]>, E5 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4
    ]>, E6 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5
    ]>, E7 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6
    ]>, E8 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7
    ]>, E9 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7,
        E8
    ]>, E10 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7,
        E8,
        E9
    ]>, E11 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7,
        E8,
        E9,
        E10
    ]>, M1 extends H<E2, MergedPath, I> = H<E2, MergedPath, I>, M2 extends H<E3, MergedPath, I2> = H<E3, MergedPath, I2>, M3 extends H<E4, MergedPath, I3> = H<E4, MergedPath, I3>, M4 extends H<E5, MergedPath, I4> = H<E5, MergedPath, I4>, M5 extends H<E6, MergedPath, I5> = H<E6, MergedPath, I5>, M6 extends H<E7, MergedPath, I6> = H<E7, MergedPath, I6>, M7 extends H<E8, MergedPath, I7> = H<E8, MergedPath, I7>, M8 extends H<E9, MergedPath, I8> = H<E9, MergedPath, I8>, M9 extends H<E10, MergedPath, I9> = H<E10, MergedPath, I9>>(method: M, path: P, ...handlers: [
        H<E2, MergedPath, I> & M1,
        H<E3, MergedPath, I2> & M2,
        H<E4, MergedPath, I3> & M3,
        H<E5, MergedPath, I4> & M4,
        H<E6, MergedPath, I5> & M5,
        H<E7, MergedPath, I6> & M6,
        H<E8, MergedPath, I7> & M7,
        H<E9, MergedPath, I8> & M8,
        H<E10, MergedPath, I9> & M9,
        H<E11, MergedPath, I10, R>
    ]): Hono$1<IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7,
        E8,
        E9,
        E10,
        E11
    ]>, S & ToSchema<M, MergePath<BasePath, P>, I10, MergeTypedResponse<R> | MergeMiddlewareResponse<M1> | MergeMiddlewareResponse<M2> | MergeMiddlewareResponse<M3> | MergeMiddlewareResponse<M4> | MergeMiddlewareResponse<M5> | MergeMiddlewareResponse<M6> | MergeMiddlewareResponse<M7> | MergeMiddlewareResponse<M8> | MergeMiddlewareResponse<M9>>, BasePath, MergePath<BasePath, P>>;
    <M extends string, P extends string, R extends HandlerResponse<any> = any, I extends Input = BlankInput>(method: M, path: P, ...handlers: [
        H<E, MergePath<BasePath, P>, I, R>,
        ...H<E, MergePath<BasePath, P>, I, R>[]
    ]): Hono$1<E, S & ToSchema<M, MergePath<BasePath, P>, I, MergeTypedResponse<R>>, BasePath, MergePath<BasePath, P>>;
    <M extends string, P extends string, MergedPath extends MergePath<BasePath, P>, R extends HandlerResponse<any> = any, I extends Input = BlankInput, E2 extends Env = E>(methods: M[], path: P, handler: H<E2, MergedPath, I, R>): Hono$1<IntersectNonAnyTypes<[
        E,
        E2
    ]>, S & ToSchema<M, MergePath<BasePath, P>, I, MergeTypedResponse<R>>, BasePath, MergePath<BasePath, P>>;
    <M extends string, P extends string, MergedPath extends MergePath<BasePath, P>, R extends HandlerResponse<any> = any, I extends Input = BlankInput, I2 extends Input = I, E2 extends Env = E, E3 extends Env = IntersectNonAnyTypes<[
        E,
        E2
    ]>, M1 extends H<E2, MergedPath, I> = H<E2, MergedPath, I>>(methods: M[], path: P, ...handlers: [
        H<E2, MergedPath, I> & M1,
        H<E3, MergedPath, I2, R>
    ]): Hono$1<IntersectNonAnyTypes<[
        E,
        E2,
        E3
    ]>, S & ToSchema<M, MergePath<BasePath, P>, I2, MergeTypedResponse<R> | MergeMiddlewareResponse<M1>>, BasePath, MergePath<BasePath, P>>;
    <M extends string, P extends string, MergedPath extends MergePath<BasePath, P>, R extends HandlerResponse<any> = any, I extends Input = BlankInput, I2 extends Input = I, I3 extends Input = I & I2, E2 extends Env = E, E3 extends Env = IntersectNonAnyTypes<[
        E,
        E2
    ]>, E4 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3
    ]>, M1 extends H<E2, MergedPath, I> = H<E2, MergedPath, I>, M2 extends H<E3, MergedPath, I2> = H<E3, MergedPath, I2>>(methods: M[], path: P, ...handlers: [
        H<E2, MergedPath, I> & M1,
        H<E3, MergedPath, I2> & M2,
        H<E4, MergedPath, I3, R>
    ]): Hono$1<IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4
    ]>, S & ToSchema<M, MergePath<BasePath, P>, I3, MergeTypedResponse<R> | MergeMiddlewareResponse<M1> | MergeMiddlewareResponse<M2>>, BasePath, MergePath<BasePath, P>>;
    <M extends string, P extends string, MergedPath extends MergePath<BasePath, P>, R extends HandlerResponse<any> = any, I extends Input = BlankInput, I2 extends Input = I, I3 extends Input = I & I2, I4 extends Input = I & I2 & I3, E2 extends Env = E, E3 extends Env = IntersectNonAnyTypes<[
        E,
        E2
    ]>, E4 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3
    ]>, E5 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4
    ]>, M1 extends H<E2, MergedPath, I> = H<E2, MergedPath, I>, M2 extends H<E3, MergedPath, I2> = H<E3, MergedPath, I2>, M3 extends H<E4, MergedPath, I3> = H<E4, MergedPath, I3>>(methods: M[], path: P, ...handlers: [
        H<E2, MergedPath, I> & M1,
        H<E3, MergedPath, I2> & M2,
        H<E4, MergedPath, I3> & M3,
        H<E5, MergedPath, I4, R>
    ]): Hono$1<IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5
    ]>, S & ToSchema<M, MergePath<BasePath, P>, I4, MergeTypedResponse<R> | MergeMiddlewareResponse<M1> | MergeMiddlewareResponse<M2> | MergeMiddlewareResponse<M3>>, BasePath, MergePath<BasePath, P>>;
    <M extends string, P extends string, MergedPath extends MergePath<BasePath, P>, R extends HandlerResponse<any> = any, I extends Input = BlankInput, I2 extends Input = I, I3 extends Input = I & I2, I4 extends Input = I & I2 & I3, I5 extends Input = I & I2 & I3 & I4, E2 extends Env = E, E3 extends Env = IntersectNonAnyTypes<[
        E,
        E2
    ]>, E4 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3
    ]>, E5 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4
    ]>, E6 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5
    ]>, M1 extends H<E2, MergedPath, I> = H<E2, MergedPath, I>, M2 extends H<E3, MergedPath, I2> = H<E3, MergedPath, I2>, M3 extends H<E4, MergedPath, I3> = H<E4, MergedPath, I3>, M4 extends H<E5, MergedPath, I4> = H<E5, MergedPath, I4>>(methods: M[], path: P, ...handlers: [
        H<E2, MergedPath, I> & M1,
        H<E3, MergedPath, I2> & M2,
        H<E4, MergedPath, I3> & M3,
        H<E5, MergedPath, I4> & M4,
        H<E6, MergedPath, I5, R>
    ]): Hono$1<IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6
    ]>, S & ToSchema<M, MergePath<BasePath, P>, I5, MergeTypedResponse<R> | MergeMiddlewareResponse<M1> | MergeMiddlewareResponse<M2> | MergeMiddlewareResponse<M3> | MergeMiddlewareResponse<M4>>, BasePath, MergePath<BasePath, P>>;
    <M extends string, P extends string, MergedPath extends MergePath<BasePath, P>, R extends HandlerResponse<any> = any, I extends Input = BlankInput, I2 extends Input = I, I3 extends Input = I & I2, I4 extends Input = I & I2 & I3, I5 extends Input = I & I2 & I3 & I4, I6 extends Input = I & I2 & I3 & I4 & I5, E2 extends Env = E, E3 extends Env = IntersectNonAnyTypes<[
        E,
        E2
    ]>, E4 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3
    ]>, E5 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4
    ]>, E6 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5
    ]>, E7 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6
    ]>, M1 extends H<E2, MergedPath, I> = H<E2, MergedPath, I>, M2 extends H<E3, MergedPath, I2> = H<E3, MergedPath, I2>, M3 extends H<E4, MergedPath, I3> = H<E4, MergedPath, I3>, M4 extends H<E5, MergedPath, I4> = H<E5, MergedPath, I4>, M5 extends H<E6, MergedPath, I5> = H<E6, MergedPath, I5>>(methods: M[], path: P, ...handlers: [
        H<E2, MergedPath, I> & M1,
        H<E3, MergedPath, I2> & M2,
        H<E4, MergedPath, I3> & M3,
        H<E5, MergedPath, I4> & M4,
        H<E6, MergedPath, I5> & M5,
        H<E7, MergedPath, I6, R>
    ]): Hono$1<IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7
    ]>, S & ToSchema<M, MergePath<BasePath, P>, I6, MergeTypedResponse<R> | MergeMiddlewareResponse<M1> | MergeMiddlewareResponse<M2> | MergeMiddlewareResponse<M3> | MergeMiddlewareResponse<M4> | MergeMiddlewareResponse<M5>>, BasePath, MergePath<BasePath, P>>;
    <M extends string, P extends string, MergedPath extends MergePath<BasePath, P>, R extends HandlerResponse<any> = any, I extends Input = BlankInput, I2 extends Input = I, I3 extends Input = I & I2, I4 extends Input = I & I2 & I3, I5 extends Input = I & I2 & I3 & I4, I6 extends Input = I & I2 & I3 & I4 & I5, I7 extends Input = I & I2 & I3 & I4 & I5 & I6, E2 extends Env = E, E3 extends Env = IntersectNonAnyTypes<[
        E,
        E2
    ]>, E4 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3
    ]>, E5 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4
    ]>, E6 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5
    ]>, E7 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6
    ]>, E8 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7
    ]>, M1 extends H<E2, MergedPath, I> = H<E2, MergedPath, I>, M2 extends H<E3, MergedPath, I2> = H<E3, MergedPath, I2>, M3 extends H<E4, MergedPath, I3> = H<E4, MergedPath, I3>, M4 extends H<E5, MergedPath, I4> = H<E5, MergedPath, I4>, M5 extends H<E6, MergedPath, I5> = H<E6, MergedPath, I5>, M6 extends H<E7, MergedPath, I6> = H<E7, MergedPath, I6>>(methods: M[], path: P, ...handlers: [
        H<E2, MergedPath, I> & M1,
        H<E3, MergedPath, I2> & M2,
        H<E4, MergedPath, I3> & M3,
        H<E5, MergedPath, I4> & M4,
        H<E6, MergedPath, I5> & M5,
        H<E7, MergedPath, I6> & M6,
        H<E8, MergedPath, I7, R>
    ]): Hono$1<IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7,
        E8
    ]>, S & ToSchema<M, MergePath<BasePath, P>, I7, MergeTypedResponse<R> | MergeMiddlewareResponse<M1> | MergeMiddlewareResponse<M2> | MergeMiddlewareResponse<M3> | MergeMiddlewareResponse<M4> | MergeMiddlewareResponse<M5> | MergeMiddlewareResponse<M6>>, BasePath, MergePath<BasePath, P>>;
    <M extends string, P extends string, MergedPath extends MergePath<BasePath, P>, R extends HandlerResponse<any> = any, I extends Input = BlankInput, I2 extends Input = I, I3 extends Input = I & I2, I4 extends Input = I & I2 & I3, I5 extends Input = I & I2 & I3 & I4, I6 extends Input = I & I2 & I3 & I4 & I5, I7 extends Input = I & I2 & I3 & I4 & I5 & I6, I8 extends Input = I & I2 & I3 & I4 & I5 & I6 & I7, E2 extends Env = E, E3 extends Env = IntersectNonAnyTypes<[
        E,
        E2
    ]>, E4 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3
    ]>, E5 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4
    ]>, E6 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5
    ]>, E7 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6
    ]>, E8 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7
    ]>, E9 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7,
        E8
    ]>, M1 extends H<E2, MergedPath, I> = H<E2, MergedPath, I>, M2 extends H<E3, MergedPath, I2> = H<E3, MergedPath, I2>, M3 extends H<E4, MergedPath, I3> = H<E4, MergedPath, I3>, M4 extends H<E5, MergedPath, I4> = H<E5, MergedPath, I4>, M5 extends H<E6, MergedPath, I5> = H<E6, MergedPath, I5>, M6 extends H<E7, MergedPath, I6> = H<E7, MergedPath, I6>, M7 extends H<E8, MergedPath, I7> = H<E8, MergedPath, I7>>(methods: M[], path: P, ...handlers: [
        H<E2, MergedPath, I> & M1,
        H<E3, MergedPath, I2> & M2,
        H<E4, MergedPath, I3> & M3,
        H<E5, MergedPath, I4> & M4,
        H<E6, MergedPath, I5> & M5,
        H<E7, MergedPath, I6> & M6,
        H<E8, MergedPath, I7> & M7,
        H<E9, MergedPath, I8, R>
    ]): Hono$1<IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7,
        E8,
        E9
    ]>, S & ToSchema<M, MergePath<BasePath, P>, I8, MergeTypedResponse<R> | MergeMiddlewareResponse<M1> | MergeMiddlewareResponse<M2> | MergeMiddlewareResponse<M3> | MergeMiddlewareResponse<M4> | MergeMiddlewareResponse<M5> | MergeMiddlewareResponse<M6> | MergeMiddlewareResponse<M7>>, BasePath, MergePath<BasePath, P>>;
    <M extends string, P extends string, MergedPath extends MergePath<BasePath, P>, R extends HandlerResponse<any> = any, I extends Input = BlankInput, I2 extends Input = I, I3 extends Input = I & I2, I4 extends Input = I & I2 & I3, I5 extends Input = I & I2 & I3 & I4, I6 extends Input = I & I2 & I3 & I4 & I5, I7 extends Input = I & I2 & I3 & I4 & I5 & I6, I8 extends Input = I & I2 & I3 & I4 & I5 & I6 & I7, I9 extends Input = I & I2 & I3 & I4 & I5 & I6 & I7 & I8, E2 extends Env = E, E3 extends Env = IntersectNonAnyTypes<[
        E,
        E2
    ]>, E4 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3
    ]>, E5 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4
    ]>, E6 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5
    ]>, E7 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6
    ]>, E8 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7
    ]>, E9 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7,
        E8
    ]>, E10 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7,
        E8,
        E9
    ]>, M1 extends H<E2, MergedPath, I> = H<E2, MergedPath, I>, M2 extends H<E3, MergedPath, I2> = H<E3, MergedPath, I2>, M3 extends H<E4, MergedPath, I3> = H<E4, MergedPath, I3>, M4 extends H<E5, MergedPath, I4> = H<E5, MergedPath, I4>, M5 extends H<E6, MergedPath, I5> = H<E6, MergedPath, I5>, M6 extends H<E7, MergedPath, I6> = H<E7, MergedPath, I6>, M7 extends H<E8, MergedPath, I7> = H<E8, MergedPath, I7>, M8 extends H<E9, MergedPath, I8> = H<E9, MergedPath, I8>>(methods: M[], path: P, ...handlers: [
        H<E2, MergedPath, I> & M1,
        H<E3, MergedPath, I2> & M2,
        H<E4, MergedPath, I3> & M3,
        H<E5, MergedPath, I4> & M4,
        H<E6, MergedPath, I5> & M5,
        H<E7, MergedPath, I6> & M6,
        H<E8, MergedPath, I7> & M7,
        H<E9, MergedPath, I8> & M8,
        H<E10, MergedPath, I9, R>
    ]): Hono$1<IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7,
        E8,
        E9,
        E10
    ]>, S & ToSchema<M, MergePath<BasePath, P>, I9, MergeTypedResponse<R> | MergeMiddlewareResponse<M1> | MergeMiddlewareResponse<M2> | MergeMiddlewareResponse<M3> | MergeMiddlewareResponse<M4> | MergeMiddlewareResponse<M5> | MergeMiddlewareResponse<M6> | MergeMiddlewareResponse<M7> | MergeMiddlewareResponse<M8>>, BasePath, MergePath<BasePath, P>>;
    <M extends string, P extends string, MergedPath extends MergePath<BasePath, P>, R extends HandlerResponse<any> = any, I extends Input = BlankInput, I2 extends Input = I, I3 extends Input = I & I2, I4 extends Input = I & I2 & I3, I5 extends Input = I & I2 & I3 & I4, I6 extends Input = I & I2 & I3 & I4 & I5, I7 extends Input = I & I2 & I3 & I4 & I5 & I6, I8 extends Input = I & I2 & I3 & I4 & I5 & I6 & I7, I9 extends Input = I & I2 & I3 & I4 & I5 & I6 & I7 & I8, I10 extends Input = I & I2 & I3 & I4 & I5 & I6 & I7 & I8 & I9, E2 extends Env = E, E3 extends Env = IntersectNonAnyTypes<[
        E,
        E2
    ]>, E4 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3
    ]>, E5 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4
    ]>, E6 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5
    ]>, E7 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6
    ]>, E8 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7
    ]>, E9 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7,
        E8
    ]>, E10 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7,
        E8,
        E9
    ]>, E11 extends Env = IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7,
        E8,
        E9,
        E10
    ]>, M1 extends H<E2, MergedPath, I> = H<E2, MergedPath, I>, M2 extends H<E3, MergedPath, I2> = H<E3, MergedPath, I2>, M3 extends H<E4, MergedPath, I3> = H<E4, MergedPath, I3>, M4 extends H<E5, MergedPath, I4> = H<E5, MergedPath, I4>, M5 extends H<E6, MergedPath, I5> = H<E6, MergedPath, I5>, M6 extends H<E7, MergedPath, I6> = H<E7, MergedPath, I6>, M7 extends H<E8, MergedPath, I7> = H<E8, MergedPath, I7>, M8 extends H<E9, MergedPath, I8> = H<E9, MergedPath, I8>, M9 extends H<E10, MergedPath, I9> = H<E10, MergedPath, I9>>(methods: M[], path: P, ...handlers: [
        H<E2, MergedPath, I> & M1,
        H<E3, MergedPath, I2> & M2,
        H<E4, MergedPath, I3> & M3,
        H<E5, MergedPath, I4> & M4,
        H<E6, MergedPath, I5> & M5,
        H<E7, MergedPath, I6> & M6,
        H<E8, MergedPath, I7> & M7,
        H<E9, MergedPath, I8> & M8,
        H<E10, MergedPath, I9> & M9,
        H<E11, MergedPath, I10, R>
    ]): Hono$1<IntersectNonAnyTypes<[
        E,
        E2,
        E3,
        E4,
        E5,
        E6,
        E7,
        E8,
        E9,
        E10,
        E11
    ]>, S & ToSchema<M, MergePath<BasePath, P>, I10, MergeTypedResponse<R> | MergeMiddlewareResponse<M1> | MergeMiddlewareResponse<M2> | MergeMiddlewareResponse<M3> | MergeMiddlewareResponse<M4> | MergeMiddlewareResponse<M5> | MergeMiddlewareResponse<M6> | MergeMiddlewareResponse<M7> | MergeMiddlewareResponse<M8> | MergeMiddlewareResponse<M9>>, BasePath, MergePath<BasePath, P>>;
    <M extends string, P extends string, R extends HandlerResponse<any> = any, I extends Input = BlankInput>(methods: M[], path: P, ...handlers: [
        H<E, MergePath<BasePath, P>, I, R>,
        ...H<E, MergePath<BasePath, P>, I, R>[]
    ]): Hono$1<E, S & ToSchema<M, MergePath<BasePath, P>, I, MergeTypedResponse<R>>, BasePath, MergePath<BasePath, P>>;
    <M extends string, const Ps extends string[], I extends Input = BlankInput, R extends HandlerResponse<any> = any, E2 extends Env = E>(methods: M | M[], paths: Ps, ...handlers: H<E2, MergePath<BasePath, Ps[number]>, I, R>[]): Hono$1<E, S & ToSchema<M, MergePath<BasePath, Ps[number]>, I, MergeTypedResponse<R>>, BasePath, Ps extends [
        ...string[],
        infer LastPath extends string
    ] ? MergePath<BasePath, LastPath> : never>;
}
type ToSchemaOutput<RorO, I extends Input | Input['in']> = RorO extends TypedResponse<infer T, infer U, infer F> ? {
    output: unknown extends T ? {} : T;
    outputFormat: I extends {
        outputFormat: string;
    } ? I['outputFormat'] : F;
    status: U;
} : {
    output: unknown extends RorO ? {} : RorO;
    outputFormat: unknown extends RorO ? 'json' : I extends {
        outputFormat: string;
    } ? I['outputFormat'] : 'json';
    status: StatusCode;
};
type ToSchema<M extends string, P extends string, I extends Input | Input['in'], RorO> = IsAny<RorO> extends true ? {
    [K in P]: {
        [K2 in M as AddDollar<K2>]: {
            input: AddParam<ExtractInput<I>, P>;
            output: {};
            outputFormat: ResponseFormat;
            status: StatusCode;
        };
    };
} : [
    RorO
] extends [
    never
] ? {} : [
    RorO
] extends [
    Promise<void>
] ? {} : {
    [K in P]: {
        [K2 in M as AddDollar<K2>]: Simplify<{
            input: AddParam<ExtractInput<I>, P>;
        } & ToSchemaOutput<RorO, I>>;
    };
};
type Schema = {
    [Path: string]: {
        [Method: `$${Lowercase<string>}`]: Endpoint;
    };
};
type AddSchemaIfHasResponse<Merged, S extends Schema, M extends string, P extends string, I extends Input | Input['in'], BasePath extends string> = [
    Merged
] extends [
    Promise<void>
] ? S : S & ToSchema<M, MergePath<BasePath, P>, I, Merged>;
type Endpoint = {
    input: any;
    output: any;
    outputFormat: ResponseFormat;
    status: StatusCode;
};
type ExtractParams<Path extends string> = string extends Path ? Record<string, string> : Path extends `${infer _Start}:${infer Param}/${infer Rest}` ? {
    [K in Param | keyof ExtractParams<`/${Rest}`>]: string;
} : Path extends `${infer _Start}:${infer Param}` ? {
    [K in Param]: string;
} : never;
type FlattenIfIntersect<T> = T extends infer O ? {
    [K in keyof O]: O[K];
} : never;
type MergeSchemaPath<OrigSchema extends Schema, SubPath extends string> = {
    [P in keyof OrigSchema as MergePath<SubPath, P & string>]: [
        OrigSchema[P]
    ] extends [
        Record<string, Endpoint>
    ] ? {
        [M in keyof OrigSchema[P]]: MergeEndpointParamsWithPath<OrigSchema[P][M], SubPath>;
    } : never;
};
type MergeEndpointParamsWithPath<T extends Endpoint, SubPath extends string> = T extends unknown ? {
    input: T['input'] extends {
        param: infer _;
    } ? ExtractParams<SubPath> extends never ? T['input'] : FlattenIfIntersect<T['input'] & {
        param: {
            [K in keyof ExtractParams<SubPath> as K extends `${infer Prefix}{${infer _}}` ? Prefix : K]: string;
        };
    }> : RemoveBlankRecord<ExtractParams<SubPath>> extends never ? T['input'] : T['input'] & {
        param: {
            [K in keyof ExtractParams<SubPath> as K extends `${infer Prefix}{${infer _}}` ? Prefix : K]: string;
        };
    };
    output: T['output'];
    outputFormat: T['outputFormat'];
    status: T['status'];
} : never;
type AddParam<I, P extends string> = ParamKeys<P> extends never ? I : I extends {
    param: infer _;
} ? I : I & {
    param: UnionToIntersection<ParamKeyToRecord<ParamKeys<P>>>;
};
type AddDollar<T extends string> = `$${Lowercase<T>}`;
type MergePath<A extends string, B extends string> = B extends '' ? MergePath<A, '/'> : A extends '' ? B : A extends '/' ? B : A extends `${infer P}/` ? B extends `/${infer Q}` ? `${P}/${Q}` : `${P}/${B}` : B extends `/${infer Q}` ? Q extends '' ? A : `${A}/${Q}` : `${A}/${B}`;
type KnownResponseFormat = 'json' | 'text' | 'redirect';
type ResponseFormat = KnownResponseFormat | string;
type TypedResponse<T = unknown, U extends StatusCode = StatusCode, F extends ResponseFormat = T extends string ? 'text' : T extends JSONValue ? 'json' : ResponseFormat> = {
    _data: T;
    _status: U;
    _format: F;
};
type MergeTypedResponse<T> = T extends Promise<void> ? T : T extends Promise<infer T2> ? T2 extends TypedResponse ? T2 : TypedResponse : T extends TypedResponse ? T : TypedResponse;
type ExtractTypedResponseOnly<T> = T extends TypedResponse ? T : never;
type MergeMiddlewareResponse<T> = T extends (c: any, next: any) => Promise<infer R> ? Exclude<R, void> extends never ? never : Exclude<R, void> extends Response | TypedResponse<any, any, any> ? ExtractTypedResponseOnly<Exclude<R, void>> : never : T extends (c: any, next: any) => infer R ? R extends Response | TypedResponse<any, any, any> ? ExtractTypedResponseOnly<R> : never : never;
type FormValue = string | Blob;
type ParsedFormValue = string | File;
type ValidationTargets<T extends FormValue = ParsedFormValue, P extends string = string> = {
    json: any;
    form: Record<string, T | T[]>;
    query: Record<string, string | string[]>;
    param: Record<P, P extends `${infer _}?` ? string | undefined : string>;
    header: Record<RequestHeader | CustomHeader, string>;
    cookie: Record<string, string>;
};
type ParamKey<Component> = Component extends `:${infer NameWithPattern}` ? NameWithPattern extends `${infer Name}{${infer Rest}` ? Rest extends `${infer _Pattern}?` ? `${Name}?` : Name : NameWithPattern : never;
type ParamKeys<Path> = Path extends `${infer Component}/${infer Rest}` ? ParamKey<Component> | ParamKeys<Rest> : ParamKey<Path>;
type ParamKeyToRecord<T extends string> = T extends `${infer R}?` ? Record<R, string | undefined> : {
    [K in T]: string;
};
type InputToDataByTarget<T extends Input['out'], Target extends keyof ValidationTargets> = T extends {
    [K in Target]: infer R;
} ? R : never;
type RemoveQuestion<T> = T extends `${infer R}?` ? R : T;
type ProcessHead<T> = IfAnyThenEmptyObject<T extends Env ? (Env extends T ? {} : T) : T>;
type IntersectNonAnyTypes<T extends any[]> = T extends [
    infer Head,
    ...infer Rest
] ? ProcessHead<Head> & IntersectNonAnyTypes<Rest> : {};
declare abstract class FetchEventLike {
    abstract readonly request: Request;
    abstract respondWith(promise: Response | Promise<Response>): void;
    abstract passThroughOnException(): void;
    abstract waitUntil(promise: Promise<void>): void;
}
type BodyDataValueDot = {
    [x: string]: string | File | BodyDataValueDot;
};
type BodyDataValueDotAll = {
    [x: string]: string | File | (string | File)[] | BodyDataValueDotAll;
};
type SimplifyBodyData<T> = {
    [K in keyof T]: string | File | (string | File)[] | BodyDataValueDotAll extends T[K] ? string | File | (string | File)[] | BodyDataValueDotAll : string | File | BodyDataValueDot extends T[K] ? string | File | BodyDataValueDot : string | File | (string | File)[] extends T[K] ? string | File | (string | File)[] : string | File;
} & {};
type BodyDataValueComponent<T> = string | File | (T extends {
    all: false;
} ? never : T extends {
    all: true;
} | {
    all: boolean;
} ? (string | File)[] : never);
type BodyDataValueObject<T> = {
    [key: string]: BodyDataValueComponent<T> | BodyDataValueObject<T>;
};
type BodyDataValue<T> = BodyDataValueComponent<T> | (T extends {
    dot: false;
} ? never : T extends {
    dot: true;
} | {
    dot: boolean;
} ? BodyDataValueObject<T> : never);
type BodyData<T extends Partial<ParseBodyOptions> = {}> = SimplifyBodyData<Record<string, BodyDataValue<T>>>;
type ParseBodyOptions = {
    all: boolean;
    dot: boolean;
};
type Body = {
    json: any;
    text: string;
    arrayBuffer: ArrayBuffer;
    blob: Blob;
    formData: FormData;
};
type BodyCache = Partial<Body>;
declare class HonoRequest$1<P extends string = '/', I extends Input['out'] = {}> {
    raw: Request;
    routeIndex: number;
    path: string;
    bodyCache: BodyCache;
    constructor(request: Request, path?: string, matchResult?: Result<[
        unknown,
        RouterRoute
    ]>);
    param<P2 extends ParamKeys<P> = ParamKeys<P>>(key: string extends P ? never : P2 extends `${infer _}?` ? never : P2): string;
    param<P2 extends RemoveQuestion<ParamKeys<P>> = RemoveQuestion<ParamKeys<P>>>(key: P2): string | undefined;
    param(key: string): string | undefined;
    param<P2 extends string = P>(): Simplify<UnionToIntersection<ParamKeyToRecord<ParamKeys<P2>>>>;
    query(key: string): string | undefined;
    query(): Record<string, string>;
    queries(key: string): string[] | undefined;
    queries(): Record<string, string[]>;
    header(name: RequestHeader): string | undefined;
    header(name: string): string | undefined;
    header(): Record<RequestHeader | (string & CustomHeader), string>;
    parseBody<Options extends Partial<ParseBodyOptions>, T extends BodyData<Options>>(options?: Options): Promise<T>;
    parseBody<T extends BodyData>(options?: Partial<ParseBodyOptions>): Promise<T>;
    json<T = any>(): Promise<T>;
    text(): Promise<string>;
    arrayBuffer(): Promise<ArrayBuffer>;
    bytes(): Promise<Uint8Array>;
    blob(): Promise<Blob>;
    formData(): Promise<FormData>;
    addValidatedData(target: keyof ValidationTargets, data: {}): void;
    valid<T extends keyof I & keyof ValidationTargets>(target: T): InputToDataByTarget<I, T>;
    get url(): string;
    get method(): string;
    get [GET_MATCH_RESULT](): Result<[
        unknown,
        RouterRoute
    ]>;
    get matchedRoutes(): RouterRoute[];
    get routePath(): string;
}
type BaseMime = (typeof _baseMimes)[keyof typeof _baseMimes];
declare const _baseMimes: {
    readonly aac: "audio/aac";
    readonly avi: "video/x-msvideo";
    readonly avif: "image/avif";
    readonly av1: "video/av1";
    readonly bin: "application/octet-stream";
    readonly bmp: "image/bmp";
    readonly css: "text/css; charset=utf-8";
    readonly csv: "text/csv; charset=utf-8";
    readonly eot: "application/vnd.ms-fontobject";
    readonly epub: "application/epub+zip";
    readonly gif: "image/gif";
    readonly gz: "application/gzip";
    readonly htm: "text/html; charset=utf-8";
    readonly html: "text/html; charset=utf-8";
    readonly ico: "image/x-icon";
    readonly ics: "text/calendar; charset=utf-8";
    readonly jpeg: "image/jpeg";
    readonly jpg: "image/jpeg";
    readonly js: "text/javascript; charset=utf-8";
    readonly json: "application/json";
    readonly jsonld: "application/ld+json";
    readonly map: "application/json";
    readonly mid: "audio/x-midi";
    readonly midi: "audio/x-midi";
    readonly mjs: "text/javascript; charset=utf-8";
    readonly mp3: "audio/mpeg";
    readonly mp4: "video/mp4";
    readonly mpeg: "video/mpeg";
    readonly oga: "audio/ogg";
    readonly ogv: "video/ogg";
    readonly ogx: "application/ogg";
    readonly opus: "audio/opus";
    readonly otf: "font/otf";
    readonly pdf: "application/pdf";
    readonly png: "image/png";
    readonly rtf: "application/rtf";
    readonly svg: "image/svg+xml; charset=utf-8";
    readonly tif: "image/tiff";
    readonly tiff: "image/tiff";
    readonly ts: "video/mp2t";
    readonly ttf: "font/ttf";
    readonly txt: "text/plain; charset=utf-8";
    readonly wasm: "application/wasm";
    readonly webm: "video/webm";
    readonly weba: "audio/webm";
    readonly webmanifest: "application/manifest+json";
    readonly webp: "image/webp";
    readonly woff: "font/woff";
    readonly woff2: "font/woff2";
    readonly xhtml: "application/xhtml+xml; charset=utf-8";
    readonly xml: "application/xml; charset=utf-8";
    readonly zip: "application/zip";
    readonly '3gp': "video/3gpp";
    readonly '3g2': "video/3gpp2";
    readonly gltf: "model/gltf+json";
    readonly glb: "model/gltf-binary";
};
type HeaderRecord = Record<'Content-Type', BaseMime> | Record<ResponseHeader, string | string[]> | Record<string, string | string[]>;
type Data = string | ArrayBuffer | ReadableStream | Uint8Array<ArrayBuffer>;
interface ExecutionContext {
    waitUntil(promise: Promise<unknown>): void;
    passThroughOnException(): void;
    props: any;
    exports?: any;
}
interface ContextVariableMap {
}
interface ContextRenderer {
}
interface DefaultRenderer {
    (content: string | Promise<string>): Response | Promise<Response>;
}
type Renderer = ContextRenderer extends Function ? ContextRenderer : DefaultRenderer;
type PropsForRenderer = [
    ...Required<Parameters<Renderer>>
] extends [
    unknown,
    infer Props
] ? Props : unknown;
type Layout<T = Record<string, any>> = (props: T) => any;
interface Get<E extends Env> {
    <Key extends keyof E['Variables']>(key: Key): E['Variables'][Key];
    <Key extends keyof ContextVariableMap>(key: Key): ContextVariableMap[Key];
}
interface Set$1<E extends Env> {
    <Key extends keyof E['Variables']>(key: Key, value: E['Variables'][Key]): void;
    <Key extends keyof ContextVariableMap>(key: Key, value: ContextVariableMap[Key]): void;
}
interface NewResponse {
    (data: Data | null, status?: StatusCode, headers?: HeaderRecord): Response;
    (data: Data | null, init?: ResponseOrInit): Response;
}
interface BodyRespond {
    <T extends Data, U extends ContentfulStatusCode>(data: T, status?: U, headers?: HeaderRecord): Response & TypedResponse<T, U, 'body'>;
    <T extends Data, U extends ContentfulStatusCode>(data: T, init?: ResponseOrInit<U>): Response & TypedResponse<T, U, 'body'>;
    <T extends null, U extends StatusCode>(data: T, status?: U, headers?: HeaderRecord): Response & TypedResponse<null, U, 'body'>;
    <T extends null, U extends StatusCode>(data: T, init?: ResponseOrInit<U>): Response & TypedResponse<null, U, 'body'>;
}
interface TextRespond {
    <T extends string, U extends ContentfulStatusCode = ContentfulStatusCode>(text: T, status?: U, headers?: HeaderRecord): Response & TypedResponse<T, U, 'text'>;
    <T extends string, U extends ContentfulStatusCode = ContentfulStatusCode>(text: T, init?: ResponseOrInit<U>): Response & TypedResponse<T, U, 'text'>;
}
interface JSONRespond {
    <T extends JSONValue | {} | InvalidJSONValue, U extends ContentfulStatusCode = ContentfulStatusCode>(object: T, status?: U, headers?: HeaderRecord): JSONRespondReturn<T, U>;
    <T extends JSONValue | {} | InvalidJSONValue, U extends ContentfulStatusCode = ContentfulStatusCode>(object: T, init?: ResponseOrInit<U>): JSONRespondReturn<T, U>;
}
type JSONRespondReturn<T extends JSONValue | {} | InvalidJSONValue, U extends ContentfulStatusCode> = Response & TypedResponse<JSONParsed<T>, U, 'json'>;
interface HTMLRespond {
    <T extends string | Promise<string>>(html: T, status?: ContentfulStatusCode, headers?: HeaderRecord): T extends string ? Response : Promise<Response>;
    <T extends string | Promise<string>>(html: T, init?: ResponseOrInit<ContentfulStatusCode>): T extends string ? Response : Promise<Response>;
}
type ContextOptions<E extends Env> = {
    env: E['Bindings'];
    executionCtx?: FetchEventLike | ExecutionContext | undefined;
    notFoundHandler?: NotFoundHandler<E>;
    matchResult?: Result<[
        H,
        RouterRoute
    ]>;
    path?: string;
};
interface SetHeadersOptions {
    append?: boolean;
}
interface SetHeaders {
    (name: 'Content-Type', value?: BaseMime, options?: SetHeadersOptions): void;
    (name: ResponseHeader, value?: string, options?: SetHeadersOptions): void;
    (name: string, value?: string, options?: SetHeadersOptions): void;
}
type ResponseHeadersInit = [
    string,
    string
][] | Record<'Content-Type', BaseMime> | Record<ResponseHeader, string> | Record<string, string> | Headers;
interface ResponseInit<T extends StatusCode = StatusCode> {
    headers?: ResponseHeadersInit;
    status?: T;
    statusText?: string;
}
type ResponseOrInit<T extends StatusCode = StatusCode> = ResponseInit<T> | Response;
declare class Context<E extends Env = any, P extends string = any, I extends Input = {}> {
    env: E['Bindings'];
    finalized: boolean;
    error: Error | undefined;
    constructor(req: Request, options?: ContextOptions<E>);
    get req(): HonoRequest$1<P, I['out']>;
    get event(): FetchEventLike;
    get executionCtx(): ExecutionContext;
    get res(): Response;
    set res(_res: Response | undefined);
    render: Renderer;
    setLayout: (layout: Layout<PropsForRenderer & {
        Layout: Layout;
    }>) => Layout<PropsForRenderer & {
        Layout: Layout;
    }>;
    getLayout: () => Layout<PropsForRenderer & {
        Layout: Layout;
    }> | undefined;
    setRenderer: (renderer: Renderer) => void;
    header: SetHeaders;
    status: (status: StatusCode) => void;
    set: Set$1<IsAny<E> extends true ? {
        Variables: ContextVariableMap & Record<PropertyKey, any>;
    } : E>;
    get: Get<IsAny<E> extends true ? {
        Variables: ContextVariableMap & Record<PropertyKey, any>;
    } : E>;
    get var(): Readonly<ContextVariableMap & (IsAny<E['Variables']> extends true ? Record<string, any> : E['Variables'])>;
    newResponse: NewResponse;
    body: BodyRespond;
    text: TextRespond;
    json: JSONRespond;
    html: HTMLRespond;
    redirect: <T extends RedirectStatusCode = 302>(location: string | URL, status?: T) => Response & TypedResponse<undefined, T, "redirect">;
    notFound: () => ReturnType<NotFoundHandler>;
}
type GetPath<E extends Env> = (request: Request, options?: {
    env?: E['Bindings'];
}) => string;
type HonoOptions<E extends Env> = {
    strict?: boolean;
    router?: Router<[
        H,
        RouterRoute
    ]>;
    getPath?: GetPath<E>;
};
type MountOptionHandler = (c: Context) => unknown;
type MountReplaceRequest = (originalRequest: Request) => Request;
type MountOptions = MountOptionHandler | {
    optionHandler?: MountOptionHandler;
    replaceRequest?: MountReplaceRequest | false;
};
declare class Hono$1<E extends Env = Env, S extends Schema = {}, BasePath extends string = '/', CurrentPath extends string = BasePath> {
    get: HandlerInterface<E, 'get', S, BasePath, CurrentPath>;
    post: HandlerInterface<E, 'post', S, BasePath, CurrentPath>;
    put: HandlerInterface<E, 'put', S, BasePath, CurrentPath>;
    delete: HandlerInterface<E, 'delete', S, BasePath, CurrentPath>;
    options: HandlerInterface<E, 'options', S, BasePath, CurrentPath>;
    patch: HandlerInterface<E, 'patch', S, BasePath, CurrentPath>;
    query: HandlerInterface<E, 'query', S, BasePath, CurrentPath>;
    all: HandlerInterface<E, 'all', S, BasePath, CurrentPath>;
    on: OnHandlerInterface<E, S, BasePath>;
    use: MiddlewareHandlerInterface<E, S, BasePath>;
    router: Router<[
        H,
        RouterRoute
    ]>;
    readonly getPath: GetPath<E>;
    private _basePath;
    routes: RouterRoute[];
    constructor(options?: HonoOptions<E>);
    private errorHandler;
    route<SubPath extends string, SubEnv extends Env, SubSchema extends Schema, SubBasePath extends string, SubCurrentPath extends string>(path: SubPath, app: Hono$1<SubEnv, SubSchema, SubBasePath, SubCurrentPath>): Hono$1<E, MergeSchemaPath<SubSchema, MergePath<BasePath, SubPath>> | S, BasePath, CurrentPath>;
    basePath<SubPath extends string>(path: SubPath): Hono$1<E, S, MergePath<BasePath, SubPath>, MergePath<BasePath, SubPath>>;
    onError: (handler: ErrorHandler<E>) => Hono$1<E, S, BasePath, CurrentPath>;
    notFound: (handler: NotFoundHandler<E>) => Hono$1<E, S, BasePath, CurrentPath>;
    mount(path: string, applicationHandler: (request: Request, ...args: any) => Response | Promise<Response>, options?: MountOptions): Hono$1<E, S, BasePath, CurrentPath>;
    fetch: (request: Request, env?: E['Bindings'] | {}, executionCtx?: ExecutionContext) => Response | Promise<Response>;
    request: (input: Request | string | URL, requestInit?: RequestInit, Env?: E["Bindings"] | {}, executionCtx?: ExecutionContext) => Response | Promise<Response>;
    fire: () => void;
}
declare class Hono<E extends Env = BlankEnv, S extends Schema = BlankSchema, BasePath extends string = '/'> extends Hono$1<E, S, BasePath> {
    constructor(options?: HonoOptions<E>);
}
type MethodNameAll = `$${typeof METHOD_NAME_ALL_LOWERCASE}`;
type StandardMethods = `$${(typeof METHODS)[number]}`;
type ExpandAllMethod<S> = MethodNameAll extends keyof S ? {
    [M in StandardMethods]: S[MethodNameAll];
} & Omit<S, MethodNameAll> : S;
type HonoRequest = (typeof Hono.prototype)['request'];
type BuildSearchParamsFn = (query: Record<string, string | string[]>) => URLSearchParams;
type ClientRequestOptions<T = unknown> = {
    fetch?: typeof fetch | HonoRequest;
    webSocket?: (...args: ConstructorParameters<typeof WebSocket>) => WebSocket;
    init?: RequestInit;
    buildSearchParams?: BuildSearchParamsFn;
} & (keyof T extends never ? {
    headers?: Record<string, string> | (() => Record<string, string> | Promise<Record<string, string>>);
} : {
    headers: T | (() => T | Promise<T>);
});
type ClientRequest<Prefix extends string, Path extends string, S extends Schema> = {
    [M in keyof ExpandAllMethod<S>]: ExpandAllMethod<S>[M] extends Endpoint & {
        input: infer R;
    } ? R extends object ? HasRequiredKeys<R> extends true ? (args: R, options?: ClientRequestOptions) => Promise<ClientResponseOfEndpoint<ExpandAllMethod<S>[M]>> : (args?: R, options?: ClientRequestOptions) => Promise<ClientResponseOfEndpoint<ExpandAllMethod<S>[M]>> : never : never;
} & {
    $url: <const Arg extends (S[keyof S] extends {
        input: infer R;
    } ? R extends {
        param: infer P;
    } ? R extends {
        query: infer Q;
    } ? {
        param: P;
        query: Q;
    } : {
        param: P;
    } : R extends {
        query: infer Q;
    } ? {
        query: Q;
    } : {} : {}) | undefined = undefined>(arg?: Arg) => HonoURL<Prefix, Path, Arg>;
    $path: <const Arg extends (S[keyof S] extends {
        input: infer R;
    } ? R extends {
        param: infer P;
    } ? R extends {
        query: infer Q;
    } ? {
        param: P;
        query: Q;
    } : {
        param: P;
    } : R extends {
        query: infer Q;
    } ? {
        query: Q;
    } : {} : {}) | undefined = undefined>(arg?: Arg) => BuildPath<Path, Arg>;
} & (S['$get'] extends {
    outputFormat: 'ws';
} ? S['$get'] extends {
    input: infer I;
} ? {
    $ws: (args?: I) => WebSocket;
} : {} : {});
type ClientResponseOfEndpoint<T extends Endpoint = Endpoint> = T extends {
    output: infer O;
    outputFormat: infer F;
    status: infer S;
} ? ClientResponse<O, S extends number ? S : never, F extends ResponseFormat ? F : never> : never;
interface ClientResponse<T, U extends number = StatusCode, F extends ResponseFormat = ResponseFormat> {
    readonly body: ReadableStream | null;
    readonly bodyUsed: boolean;
    ok: U extends SuccessStatusCode ? true : U extends Exclude<StatusCode, SuccessStatusCode> ? false : boolean;
    redirected: boolean;
    status: U;
    statusText: string;
    type: 'basic' | 'cors' | 'default' | 'error' | 'opaque' | 'opaqueredirect';
    headers: Headers;
    url: string;
    redirect(url: string, status: number): Response$1;
    clone(): Response$1;
    bytes(): Promise<Uint8Array<ArrayBuffer>>;
    json(): F extends 'text' ? Promise<never> : F extends 'json' ? Promise<T> : Promise<unknown>;
    text(): F extends 'text' ? (T extends string ? Promise<T> : Promise<never>) : Promise<string>;
    blob(): Promise<Blob>;
    formData(): Promise<FormData>;
    arrayBuffer(): Promise<ArrayBuffer>;
}
type BuildSearch<Arg, Key extends 'query'> = Arg extends {
    [K in Key]: infer Query;
} ? IsEmptyObject<Query> extends true ? '' : `?${string}` : '';
type BuildPathname<P extends string, Arg> = Arg extends {
    param: infer Param;
} ? `${ApplyParam<TrimStartSlash<P>, Param>}` : `/${TrimStartSlash<P>}`;
type BuildPath<P extends string, Arg> = `${BuildPathname<P, Arg>}${BuildSearch<Arg, 'query'>}`;
type BuildTypedURL<Protocol extends string, Host extends string, Port extends string, P extends string, Arg> = TypedURL<`${Protocol}:`, Host, Port, BuildPathname<P, Arg>, BuildSearch<Arg, 'query'>>;
type HonoURL<Prefix extends string, Path extends string, Arg> = IsLiteral<Prefix> extends true ? TrimEndSlash<Prefix> extends `${infer Protocol}://${infer Rest}` ? Rest extends `${infer Hostname}/${infer P}` ? ParseHostName<Hostname> extends [
    infer Host extends string,
    infer Port extends string
] ? BuildTypedURL<Protocol, Host, Port, P, Arg> : never : ParseHostName<Rest> extends [
    infer Host extends string,
    infer Port extends string
] ? BuildTypedURL<Protocol, Host, Port, Path, Arg> : never : URL : URL;
type ParseHostName<T extends string> = T extends `${infer Host}:${infer Port}` ? [
    Host,
    Port
] : [
    T,
    ''
];
type TrimStartSlash<T extends string> = T extends `/${infer R}` ? TrimStartSlash<R> : T;
type TrimEndSlash<T extends string> = T extends `${infer R}/` ? TrimEndSlash<R> : T;
type IsLiteral<T extends string> = [
    T
] extends [
    never
] ? false : string extends T ? false : true;
type ApplyParam<Path extends string, P, Result extends string = ''> = Path extends `${infer Head}/${infer Rest}` ? Head extends `:${infer Param}` ? P extends Record<Param, infer Value extends string> ? IsLiteral<Value> extends true ? ApplyParam<Rest, P, `${Result}/${Value & string}`> : ApplyParam<Rest, P, `${Result}/${Head}`> : ApplyParam<Rest, P, `${Result}/${Head}`> : ApplyParam<Rest, P, `${Result}/${Head}`> : Path extends `:${infer Param}` ? P extends Record<Param, infer Value extends string> ? IsLiteral<Value> extends true ? `${Result}/${Value & string}` : `${Result}/${Path}` : `${Result}/${Path}` : `${Result}/${Path}`;
type IsEmptyObject<T> = keyof T extends never ? true : false;
interface TypedURL<Protocol extends string, Hostname extends string, Port extends string, Pathname extends string, Search extends string> extends URL {
    protocol: Protocol;
    hostname: Hostname;
    port: Port;
    host: Port extends '' ? Hostname : `${Hostname}:${Port}`;
    origin: `${Protocol}//${Hostname}${Port extends '' ? '' : `:${Port}`}`;
    pathname: Pathname;
    search: Search;
    href: `${Protocol}//${Hostname}${Port extends '' ? '' : `:${Port}`}${Pathname}${Search}`;
}
interface Response$1 extends ClientResponse<unknown> {
}
type InferEndpointType<T> = T extends (args: infer R, options: any | undefined) => Promise<infer U> ? U extends ClientResponse<infer O, infer S, infer F> ? {
    input: NonNullable<R>;
    output: O;
    outputFormat: F;
    status: S;
} extends Endpoint ? {
    input: NonNullable<R>;
    output: O;
    outputFormat: F;
    status: S;
} : never : never : never;
type InferResponseType<T, U extends StatusCode = StatusCode> = InferResponseTypeFromEndpoint<InferEndpointType<T>, U>;
type InferResponseTypeFromEndpoint<T extends Endpoint, U extends StatusCode> = T extends {
    output: infer O;
    status: infer S;
} ? S extends U ? O : never : never;
type InferRequestType<T> = T extends (args: infer R, options: any | undefined) => Promise<ClientResponse<unknown>> ? NonNullable<R> : never;
type PathToChain<Prefix extends string, Path extends string, E extends Schema, Original extends string = Path> = Path extends `/${infer P}` ? PathToChain<Prefix, P, E, Path> : Path extends `${infer P}/${infer R}` ? {
    [K in P]: PathToChain<Prefix, R, E, Original>;
} : {
    [K in Path extends '' ? 'index' : Path]: ClientRequest<Prefix, Original, E extends Record<string, unknown> ? E[Original] : never>;
};
type Client<T, Prefix extends string> = T extends Hono$1<any, infer S, any> ? S extends Record<infer K, Schema> ? K extends string ? PathToChain<Prefix, K, S> : never : never : never;
declare const hc: <T extends Hono<any, any, any>, Prefix extends string = string>(baseUrl: Prefix, options?: ClientRequestOptions) => UnionToIntersection<Client<T, Prefix>>;
declare const createMiddleware: <E extends Env = any, P extends string = string, I extends Input = {}, R extends HandlerResponse<any> | void = void>(middleware: MiddlewareHandler<E, P, I, R extends void ? Response : R>) => MiddlewareHandler<E, P, I, R extends void ? Response : R>;
type HTTPExceptionOptions = {
    res?: Response;
    message?: string;
    cause?: unknown;
};
declare class HTTPException extends Error {
    readonly res?: Response;
    readonly status: ContentfulStatusCode;
    constructor(status?: ContentfulStatusCode, options?: HTTPExceptionOptions);
    getResponse(): Response;
}
type FailedResponse<T$1 extends GenericSchema | GenericSchemaAsync> = SafeParseResult<T$1> extends infer U ? Response & TypedResponse<U extends {
    success: false;
} ? U : never, 400, "json"> : never;
type MustBeResponse<T$1, Schema extends GenericSchema | GenericSchemaAsync> = T$1 extends Promise<infer U> ? Promise<MustBeResponse<U, Schema>> : T$1 extends Response | TypedResponse<unknown> ? T$1 : never;
type Hook<T$1 extends GenericSchema | GenericSchemaAsync, E$1 extends Env, P$1 extends string, Target$1 extends keyof ValidationTargets = keyof ValidationTargets, R$1 extends void | Response | TypedResponse<unknown> | Promise<Response | TypedResponse<unknown> | void> = void | Response | TypedResponse<unknown> | Promise<Response | TypedResponse<unknown> | void>> = (result: SafeParseResult<T$1> & {
    target: Target$1;
}, c: Context<E$1, P$1>) => R$1;
type HasUndefined<T$1> = undefined extends T$1 ? true : false;
declare const vValidator: <T extends GenericSchema | GenericSchemaAsync, Target extends keyof ValidationTargets, E extends Env, P extends string, In = InferInput<T>, Out = InferOutput<T>, I extends Input = {
    in: HasUndefined<In> extends true ? {
        [K in Target]?: In extends ValidationTargets[K] ? In : {
            [K2 in keyof In]?: ValidationTargets[K][K2];
        };
    } : {
        [K in Target]: In extends ValidationTargets[K] ? In : {
            [K2 in keyof In]: ValidationTargets[K][K2];
        };
    };
    out: {
        [K in Target]: Out;
    };
}, V extends I = I, R extends void | Response | TypedResponse<unknown> | Promise<Response | TypedResponse<unknown> | void> = FailedResponse<T>>(target: Target, schema: T, hook?: Hook<T, E, P, Target, R>) => Handler<E, P, V, MustBeResponse<R, T>>;
export { Context, HTTPException, Hono, createMiddleware, hc, vValidator };
export type { ClientResponse, InferRequestType, InferResponseType };
