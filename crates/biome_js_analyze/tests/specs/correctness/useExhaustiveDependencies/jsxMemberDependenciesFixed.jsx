// should not generate diagnostics

import { useCallback } from "react";

const MyNS = {
  MyComponent: () => null,
};

export function Component() {
  const render = useCallback(() => <MyNS.MyComponent />, [MyNS]);
  return render();
}
