/* should not generate diagnostics */
import Script from 'next/script'

export default function Page() {
  return (
    <Script id="my-script" dangerouslySetInnerHTML={{ __html: `console.log('Hello world!');` }}></Script>
  )
}
