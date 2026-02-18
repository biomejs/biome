// should not generate diagnostics

import { getSubComponent } from "@external";
import { useCallback } from "react";

export function Component() {
  const Sub = getSubComponent();

  const render = useCallback(() => <Sub />, [Sub]);

  return render();
}

