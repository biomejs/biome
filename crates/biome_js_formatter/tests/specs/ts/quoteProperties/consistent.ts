interface Members {
  "prop-1": number;
  method(): void;
  get value(): number;
  set value(value: number);
}

type Literal = { "a-b": number; c: string };
