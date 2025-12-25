import Script from 'next/script'

export default function SomePageLayout({ children }) {
  return (
    <section>
      <div>{children}</div>
      <Script
        src="https://example.com/script.js"
        strategy="beforeInteractive"
      />
    </section>
  )
}
