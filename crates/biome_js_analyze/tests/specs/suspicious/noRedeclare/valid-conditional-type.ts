// Issue https://github.com/biomejs/biome/issues/2659
type Test<T> = T extends Array<infer U> ? true : false