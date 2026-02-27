/* should not generate diagnostics */
import MyScript from 'next/script'

export default function Page() {
  return (
    <MyScript src="https://example.com/script.js" />
  )
}
