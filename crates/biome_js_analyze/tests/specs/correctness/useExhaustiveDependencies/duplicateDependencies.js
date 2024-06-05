import { useCallback } from "react";

function Component1({ a }) {
    const handle = useCallback(() => {
      console.log(a);
    }, [a, a]);
}

function Component2() {
    const [local,SetLocal] = useState(0);
    const handle = useCallback(() => {
      console.log(local);
    }, [local, local]);
}