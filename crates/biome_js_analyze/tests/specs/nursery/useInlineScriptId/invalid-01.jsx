/* should generate diagnostics */
import Script from 'next/script'

export default function Page() {
  return (
    <Script>{`console.log('Hello world!');`}</Script>
  )
}
