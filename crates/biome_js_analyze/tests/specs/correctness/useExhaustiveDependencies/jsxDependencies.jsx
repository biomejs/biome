import { getSubComponent } from "@external"
import { useCallback, useEffect, useState } from "react"

export function Component() {
  const Sub = getSubComponent()

  // Missing dependency 'Sub'
  const renderItem = useCallback(() => <Sub />, [])

  return renderItem
}

export function ComponentCorrect() {
  const Sub = getSubComponent()

  // Correct dependency
  const renderItem = useCallback(() => <Sub />, [Sub])

  return renderItem
}

export function LocalComponent() {
    const [count, setCount] = useState(0);

    const LocalSub = () => <div>{count}</div>

    // Missing dependency 'LocalSub'
    const memoized = useCallback(() => <LocalSub />, []);
}
