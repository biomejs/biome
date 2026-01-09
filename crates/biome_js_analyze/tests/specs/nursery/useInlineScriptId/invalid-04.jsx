/* should generate diagnostics */
import MyScript from 'next/script'

export default function Page() {
  return (
    <MyScript {...{ strategy: "beforeInteractive" }}>{`console.log('Hello world!');`}</MyScript>
  )
}
