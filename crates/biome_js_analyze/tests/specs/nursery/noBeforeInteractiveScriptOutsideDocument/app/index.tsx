import { Script } from 'myscript'

export default function Index({ children }) {
  return (
    <div>
      <Script
        src="https://example.com/script.js"
        strategy="beforeInteractive"
      />
    </div>
  )
}
