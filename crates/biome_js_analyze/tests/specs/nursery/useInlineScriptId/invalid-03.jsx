/* should generate diagnostics */
import Script from 'next/script'

export default function Page() {
  const props = { strategy: "beforeInteractive" };
  return (
    <Script {...props}>{`console.log('Hello world!');`}</Script>
  )
}
