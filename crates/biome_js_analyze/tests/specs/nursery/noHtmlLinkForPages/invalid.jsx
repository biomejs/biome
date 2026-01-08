/* should generate diagnostics */

export const Page = () => {
  return (
    <a href='/'>Homepage</a>
  );
}

export const Page = () => {
  return (
    <a href='/list/foo/bar'>Homepage</a>
  );
}

export const Page = () => {
  return (
    <a href='/list/foo?q=bar'>Homepage</a>
  );
}

export const Page = () => {
  return (
    <a href='/photo/1/#section'>Photo</a>
  );
}

export const Page = () => {
  return (
    <>
      <a href='./photo'>Photo</a>
      <a href='../photo'>Photo</a>
    </>
  );
}
