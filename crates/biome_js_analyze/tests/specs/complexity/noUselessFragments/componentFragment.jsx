import { Suspense } from "react";
import { Comp } from "./components/Comp";

export function Component() {
  return (
    <div>
      <Suspense fallback={<></>}>
        <h1>Hi</h1>
      </Suspense>

      <Comp prop={<></>} />
      <Comp prop={<Fragment></Fragment>} />
      <Comp prop={<React.Fragment></React.Fragment>} />
    </div>
  );
}
