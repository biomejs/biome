/* should not generate diagnostics */

function Button<T = string>() {
  return <div></div>;
}

const button = <Button<number> />;
