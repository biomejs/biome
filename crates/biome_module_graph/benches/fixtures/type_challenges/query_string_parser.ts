type Equal<Left, Right> =
	(<T>() => T extends Left ? 1 : 2) extends <T>() => T extends Right ? 1 : 2
		? true
		: false;

type Contains<
	Values extends readonly unknown[],
	Value,
> = Values extends readonly [infer Head, ...infer Tail]
	? Equal<Head, Value> extends true
		? true
		: Contains<Tail, Value>
	: false;

type Parameter<Part extends string> = Part extends `${infer Key}=${infer Value}`
	? [Key, Value]
	: [Part, true];

type Tokenize<
	Query extends string,
	Tokens extends [string, string | true][] = [],
> = Query extends `${infer Head}&${infer Tail}`
	? Tokenize<Tail, Head extends "" ? Tokens : [...Tokens, Parameter<Head>]>
	: Query extends ""
		? Tokens
		: [...Tokens, Parameter<Query>];

type Collect<
	Tokens extends readonly unknown[],
	Key,
	Result extends unknown[] = [],
> = Tokens extends readonly [infer Head, ...infer Tail]
	? Head extends [Key, infer Value]
		? Contains<Result, Value> extends true
			? Collect<Tail, Key, Result>
			: Collect<Tail, Key, [...Result, Value]>
		: Collect<Tail, Key, Result>
	: Result extends [infer Only]
		? Only
		: Result;

type ParseTokens<Tokens extends [string, string | true][]> = {
	[Key in Tokens[number][0]]: Collect<Tokens, Key>;
};

export type ParseQueryString<Query extends string> = ParseTokens<
	Tokenize<Query>
>;

export declare function parseQuery<const Query extends string>(
	query: Query,
): ParseQueryString<Query>;

type Values<Value> = Value extends readonly unknown[] ? Value[number] : Value;
type FlagKeys<Query> = {
	[Key in keyof Query]: true extends Values<Query[Key]> ? Key : never;
}[keyof Query];
type RepeatedKeys<Query> = {
	[Key in keyof Query]: Query[Key] extends readonly unknown[] ? Key : never;
}[keyof Query];
type QueryEvents<Query> = {
	[Key in keyof Query & string as `on${Capitalize<Key>}Change`]: (
		value: Query[Key],
	) => void;
};
type QueryEntries<Query> = {
	[Key in keyof Query]: readonly [Key, Query[Key]];
}[keyof Query];

export type QuerySummary<Query extends string> = {
	parsed: ParseQueryString<Query>;
	flags: FlagKeys<ParseQueryString<Query>>;
	repeated: RepeatedKeys<ParseQueryString<Query>>;
	entries: QueryEntries<ParseQueryString<Query>>;
	events: QueryEvents<ParseQueryString<Query>>;
};

export const catalogQuery =
	"filter0&filter1=value1&filter2=value2&filter3=value3&filter4=value4&filter5=value5&filter6=value6&filter7=value7&filter8=value8&filter9&filter10=value10&filter11=value11&filter0=value12&filter1=value13&filter2=value14&filter3=value15&filter4=value16&filter5=value0&filter6&filter7=value2&filter8=value3&filter9=value4&filter10=value5&filter11=value6" as const;
export const catalog = parseQuery(catalogQuery);
export type CatalogSummary = QuerySummary<typeof catalogQuery>;

export const analyticsQuery =
	"filter0&filter1=value1&filter2=value2&filter3=value3&filter4=value4&filter5=value5&filter6=value6&filter7=value7&filter8=value8&filter9&filter10=value10&filter11=value11&filter0=value12&filter1=value13&filter2=value14&filter3=value15&filter4=value16&filter5=value0&filter6&filter7=value2&filter8=value3&filter9=value4&filter10=value5&filter11=value6&filter0=value7&filter1=value8&filter2=value9&filter3&filter4=value11&filter5=value12&filter6=value13&filter7=value14&filter8=value15&filter9=value16&filter10=value0&filter11=value1&filter0&filter1=value3&filter2=value4&filter3=value5&filter4=value6&filter5=value7&filter6=value8&filter7=value9&filter8=value10&filter9&filter10=value12&filter11=value13&filter0=value14&filter1=value15&filter2=value16&filter3=value0&filter4=value1&filter5=value2&filter6&filter7=value4&filter8=value5&filter9=value6&filter10=value7&filter11=value8&filter0=value9&filter1=value10&filter2=value11&filter3" as const;
export const analytics = parseQuery(analyticsQuery);
export type AnalyticsSummary = QuerySummary<typeof analyticsQuery>;

export const permissionsQuery =
	"filter0&filter1=value1&filter2=value2&filter3=value3&filter4=value4&filter5=value5&filter6=value6&filter7=value7&filter8=value8&filter9&filter10=value10&filter11=value11&filter0=value12&filter1=value13&filter2=value14&filter3=value15&filter4=value16&filter5=value0&filter6&filter7=value2&filter8=value3&filter9=value4&filter10=value5&filter11=value6&filter0=value7&filter1=value8&filter2=value9&filter3&filter4=value11&filter5=value12&filter6=value13&filter7=value14&filter8=value15&filter9=value16&filter10=value0&filter11=value1&filter0&filter1=value3&filter2=value4&filter3=value5&filter4=value6&filter5=value7&filter6=value8&filter7=value9&filter8=value10&filter9&filter10=value12&filter11=value13&filter0=value14&filter1=value15&filter2=value16&filter3=value0&filter4=value1&filter5=value2&filter6&filter7=value4&filter8=value5&filter9=value6&filter10=value7&filter11=value8&filter0=value9&filter1=value10&filter2=value11&filter3&filter4=value13&filter5=value14&filter6=value15&filter7=value16&filter8=value0&filter9=value1&filter10=value2&filter11=value3&filter0&filter1=value5&filter2=value6&filter3=value7&filter4=value8&filter5=value9&filter6=value10&filter7=value11&filter8=value12&filter9&filter10=value14&filter11=value15&filter0=value16&filter1=value0&filter2=value1&filter3=value2&filter4=value3&filter5=value4&filter6&filter7=value6&filter8=value7&filter9=value8&filter10=value9&filter11=value10&filter0=value11&filter1=value12&filter2=value13&filter3&filter4=value15&filter5=value16&filter6=value0&filter7=value1&filter8=value2&filter9=value3&filter10=value4&filter11=value5&filter0&filter1=value7&filter2=value8&filter3=value9&filter4=value10&filter5=value11&filter6=value12&filter7=value13&filter8=value14&filter9&filter10=value16&filter11=value0&filter0=value1&filter1=value2&filter2=value3&filter3=value4&filter4=value5&filter5=value6&filter6&filter7=value8" as const;
export const permissions = parseQuery(permissionsQuery);
export type PermissionsSummary = QuerySummary<typeof permissionsQuery>;

export const mixed = parseQuery(
	"debug&debug=true&debug&sort=name&sort=date&sort=name&empty=&flag&flag=false&flag",
);
export const catalogFilters = [
	catalog.filter0,
	catalog.filter3,
	catalog.filter7,
	catalog.filter11,
];
export const analyticsFilters = [
	analytics.filter0,
	analytics.filter4,
	analytics.filter8,
	analytics.filter11,
];
export const permissionFilters = [
	permissions.filter0,
	permissions.filter5,
	permissions.filter9,
	permissions.filter11,
];

export type Route = "catalog" | "analytics" | "permissions";
export type Locale = "en" | "de" | "ja";
export type Mode = "preview" | "live";
export type RouteQueries = {
	[Path in `${Locale}/${Route}/${Mode}`]: Path extends `${infer Language}/${infer Page}/${infer State}`
		? ParseQueryString<`locale=${Language}&page=${Page}&mode=${State}&debug&tag=one&tag=two&tag=one&tag`>
		: never;
};
export type RouteHandlers = {
	[Path in keyof RouteQueries as `/${Path}`]: (query: RouteQueries[Path]) => {
		events: QueryEvents<RouteQueries[Path]>;
		entries: QueryEntries<RouteQueries[Path]>[];
		repeated: RepeatedKeys<RouteQueries[Path]>[];
	};
};
export declare const handlers: RouteHandlers;
export const routeResult = handlers["/en/catalog/preview"](
	parseQuery(
		"locale=en&page=catalog&mode=preview&debug&tag=one&tag=two&tag=one&tag",
	),
);
export const routeEvents = routeResult.events;
export const routeEntries = routeResult.entries;
export type CombinedQuery =
	ParseQueryString<`${typeof catalogQuery}&${typeof analyticsQuery}&${typeof permissionsQuery}`>;
export declare const combined: CombinedQuery;
export const combinedFilters = [
	combined.filter0,
	combined.filter1,
	combined.filter6,
	combined.filter10,
];
