type Foo = Record<
  // comment
  string,
  number
>

type Single = Array<
  // comment
  string
>

type Block = Record<
  /* comment */
  string,
  number
>

type Between = Record<string,
  // comment
  number
>

type Trailing = Record<string, number // comment
>

type Inline = Record</* comment */ string, number>

type Object = Array<
  // comment
  { value: string }
>

type Nested = Promise<Record<
  // comment
  string,
  number
>>
