import Script from 'next/script';

export function MyApp({ Component, pageProps }) {
  return (
    <div>
        <Component {...pageProps} />
        <script src='https://polyfill.io/v3/polyfill.min.js?features=AbortController'></script>
        <script src='https://polyfill.io/v3/polyfill.min.js?features=IntersectionObserver'></script>
        <Script src='https://polyfill.io/v3/polyfill.min.js?features=IntersectionObserver' />
        <Script src='https://polyfill-fastly.io/v3/polyfill.min.js?features=IntersectionObserver' />
    </div>
  );
}