import NextScript from 'next/script'

export function MyApp({ Component, pageProps }) {
  return (
    <div>
      <Component {...pageProps} />
      <NextScript src='https://polyfill.io/v3/polyfill.min.js?features=Array.prototype.copyWithin' />
    </div>
  );
}
