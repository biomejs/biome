/* should not generate diagnostics */
export function Panic() {
  return <div>{foo && <>{`(${bar})`}</>}</div>;
}
