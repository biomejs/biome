/* should not generate diagnostics */
import Script from 'next/script'

export default function Page() {
  const props = { id: "my-script", strategy: "beforeInteractive" };
  return (
    <Script {...props}>{`console.log('Hello world!');`}</Script>
  )
}
