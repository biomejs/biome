type A =
    {    [   a: string   ] :       number      }

type B = {
    (a: string, b: symbol, c: symbol, d: symbol )
}

type C = {
    (loreum: string, ipsum: symbol, lapis: symbol, emerald: symbol, consequence: symbol, something_with_long_name: symbol, some_other_time: symbol )
}
type D = {
    <Aaaaaaaaaaaaaaaaaaaaa,bvvvvvvvvvvvvvvvvvvvvvv,ceeeeeee,deeeeeeeeeeeeee,deeeeeeeeeeeeeee,deeeeeeeeeeeeeeee,deeeeeeeewweeeeee,>
    (loreum: string, ipsum: symbol, lapis: symbol, emerald: symbol, consequence: symbol, something_with_long_name: symbol, some_other_time: symbol )
}

type E = {
    <Aaaaaaaaaaaaaaaaaaaaa>
    (loreum: string )
}


type F = {
    <Aaaaaaaaaaaaaaaaaaaaa,bvvvvvvvvvvvvvvvvvvvvvv,ceeeeeee,deeeeeeeeeeeeee,deeeeeeeeeeeeeee,deeeeeeeeeeeeeeee,deeeeeeeewweeeeee,>
    (loreum: string )
}

type G = {
    <Aaaaaaaaaaaaaaaaaaaaa>
    (loreum: string, ipsum: symbol, lapis: symbol, emerald: symbol, consequence: symbol, something_with_long_name: symbol, some_other_time: symbol )
}

type H = {
    a?(): number; b?(): number, c?(): number
    d(): string
    bvvvvvvvvvvvvvvvvvvvvvv?(loreum: string, ipsum: symbol, lapis: symbol, emerald: symbol,): G
}

type LoooooooooooooongTypeReturneeeeeeeeed = "0";
type I = {
    new(loreum: string, ipsum: symbol, lapis: symbol, emerald: symbol, consequence: symbol, something_with_long_name: symbol, some_other_time: symbol ): LoooooooooooooongTypeReturneeeeeeeeed
}

type J = {
    get     something( ): LoooooooooooooongTypeReturneeeeeeeeed
}

type K = { set     something( something_with_long_name: string ) }

type L = { set     something( something_with_long_name: string, ) }
