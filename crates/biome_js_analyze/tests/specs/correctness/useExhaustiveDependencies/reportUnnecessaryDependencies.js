import {useEffect} from "react";

// should not report errors for the unused `b` when the reportUnecessaryDependencies option is false
function ReportUnecessaryDependencies() {
    const [b] = useState("hello")
    const [a] = useState("world");

    useEffect(() => {
        console.log(a);
    }, [a, b]);

    return a;
}
