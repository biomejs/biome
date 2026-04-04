/* should generate diagnostics */
import Script from 'next/script'

export default function Page() {
  return (
    <Script dangerouslySetInnerHTML={{ __html: `console.log('Hello world!');` }} />
  )
}
