/* should not generate diagnostics */

// Fixture for https://github.com/biomejs/biome/issues/10704

const FACTIONS = [
  { name: "REPUBLIC", id: 1 },
  { name: "EMPIRE", id: 2 },
] as const;

function findFaction(id: number) {
  return FACTIONS.find((f) => f.id === id)?.name ?? null;
}

type Movie = {
  characters?: Array<{
    isJedi: boolean;
    name: string;
  }>;
};

export function getJediNames(movie: Movie): Array<string> {
  const jedis = movie.characters?.filter((character) => character.isJedi) ?? [];

  return jedis.map((character) => character.name);
}

type Blah = 'a' | 'c';
type ListListing = Record<string | Blah, string>;

const LIST: ListListing = { a: '1', b: '2', c: '3' };

function isKey(k: string): k is Blah {
  return k !== 'b';
}

function test(listing: ListListing): Blah[] {
  return Object.keys(listing)
    .map((k) => (isKey(k) ? k : undefined))
    .filter((k) => k !== undefined);
}

test(LIST);
