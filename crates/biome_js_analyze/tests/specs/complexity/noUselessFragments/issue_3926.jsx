export function Panic() {
  return <div>{foo && <>{`(${bar})`}</>}</div>;
}