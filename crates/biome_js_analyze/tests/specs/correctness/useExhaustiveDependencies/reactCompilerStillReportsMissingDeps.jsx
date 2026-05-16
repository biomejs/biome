import { useEffect } from "react";

function Component({ thing }) {
    useEffect(() => {
        console.log(thing);
    }, []);
}
