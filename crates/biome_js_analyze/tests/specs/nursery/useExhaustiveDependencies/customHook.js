import { useEffect } from "react";

function MyComponent() {
    let a = 1;
    useEffect(() => {
        console.log(a);
    });
    useMyEffect(() => {
        console.log(a);
    });
}
