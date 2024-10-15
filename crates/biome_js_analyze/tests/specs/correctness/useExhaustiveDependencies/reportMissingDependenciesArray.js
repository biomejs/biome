import {useEffect} from "react";

// should not report errors for the unused `b` when the reportMissingDependenciesArray option is false
function ReportMissingDependenciesArray() {
    const [a] = useState(1);

    useEffect(() => {
        console.log(a);
    });

    return a;
}
