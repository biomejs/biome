/* should generate diagnostics */
import { useIsFetching as useQuery } from "@tanstack/react-query";
import { useQueries as useSuspenseQuery } from "@tanstack/react-query";

// The local name is a hook name.
const { data, ...rest } = useQuery();

// The imported name takes precedence over the local name, so this is a `useQueries` call.
const [{ data: first, ...firstRest }] = useSuspenseQuery([]);
