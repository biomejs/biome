/* should not generate diagnostics */

import Link from 'next/link';

export const Page = () => {
  return (
    <Link href='/'>
      <a>Homepage</a>
    </Link>
  );
}

export const Page = () => {
  return (
    <a href='#heading'>Homepage</a>
  );
};

export const Page = () => {
  return (
    <a href=''>Homepage</a>
  );
};

export const Page = () => {
  return (
    <a href='https://example.com/'>Homepage</a>
  );
}

export const Page = () => {
  return (
    <>
      <a href='/static-file.csv' download>Download</a>
      <a href='/presentation.pdf'>View PDF</a>
    </>
  );
}

export const Page = () => {
  return (
    <a target="_blank" href='/new-tab'>New Tab</a>
  );
}

export const Page = () => {
  return (
    <Link href='/photo/1/'>Photo</Link>
  );
}

export const Page = () => {
  return (
    <Link href='./photo'>Photo</Link>
  );
}

export const Page = () => {
  return (
    <Link href='../photo'>Photo</Link>
  );
}

export const Page = () => {
  return (
  	<>
	   <a href='mailto:test@example.com'>Email</a>
	   <a href='tel:+1234567890'>Phone</a>
   </>
  );
}