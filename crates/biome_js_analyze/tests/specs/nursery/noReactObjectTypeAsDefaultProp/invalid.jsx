/* should generate diagnostics */

// function declaration
function Foo({
  a = {},
  b = ["one", "two"],
  c = /regex/i,
  d = () => {},
  e = function () {},
  f = class {},
  g = new Thing(),
  h = <Thing />,
  i = Symbol("foo"),
}) {
  return null;
}

// arrow function
const Bar = ({
  a = {},
  b = ["one", "two"],
  c = /regex/i,
  d = () => {},
  e = function () {},
  f = class {},
  g = new Thing(),
  h = <Thing />,
  i = Symbol("foo"),
}) => {
  return null;
};

// memo / forwardRef wrappers
const Wrapped = memo(({ a = {} }) => null);
const Forwarded = forwardRef(({ b = [] }, ref) => null);

// aliased and nested defaults
function Aliased({ value: renamed = {} }) {
  return renamed;
}

function NestedDefault({ config: { value = {} } = {} }) {
  return value;
}

// PascalCase name is enough; Biome detects components by name, not by return value
export default function NotReturningJsx({ foo = {} }) {}
