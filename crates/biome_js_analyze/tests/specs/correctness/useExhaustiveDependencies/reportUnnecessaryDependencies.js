import {useEffect} from "react";

// should not report errors for the unused `b` when the reportUnnecessaryDependencies option is false
function ReportUnnecessaryDependencies() {
    const [b] = useState("hello")
    const [a] = useState("world");

    useEffect(() => {
        console.log(a);
    }, [a, b]);

    return a;
}
