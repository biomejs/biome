import Script from 'next/script'

export default function Index() {
  return (
    <div>
      <Script
        src="https://example.com/script.js"
        strategy="beforeInteractive"
      ></Script>
    </div>
  )
}
