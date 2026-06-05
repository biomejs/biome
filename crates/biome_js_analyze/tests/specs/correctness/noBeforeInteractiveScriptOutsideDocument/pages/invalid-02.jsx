/* should generate diagnostics */
import MyScript from 'next/script'

export default function Index() {
  return (
    <div>
      <MyScript
        src="https://example.com/script.js"
        strategy="beforeInteractive"
      ></MyScript>
    </div>
  )
}
