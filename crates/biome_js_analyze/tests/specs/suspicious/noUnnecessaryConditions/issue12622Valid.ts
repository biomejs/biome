/* should not generate diagnostics */

type Media = { readonly items: ReadonlyArray<string> } | null;

export function paths(media: Media): string[] {
	return [...(media?.items ?? [])];
}

type Movie = { characters: string[] } | undefined;

export function characters(movie: Movie): string[] {
	return movie?.characters ?? [];
}
