/* should generate diagnostics */
import { useQuery } from "@tanstack/react-query";

function Component() {
	const query = useQuery({ queryKey: ["a"], queryFn });

	useEffect(() => {}, [query]);
	React.useMemo(() => query.data, [query]);
}
