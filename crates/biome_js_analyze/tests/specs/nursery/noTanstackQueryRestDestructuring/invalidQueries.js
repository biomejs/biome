/* should generate diagnostics */
import { useQueries, useSuspenseQueries } from "@tanstack/react-query";

const [{ data: first, ...firstRest }] = useQueries({ queries: [] });
const [{ ...suspenseFirst }] = useSuspenseQueries({ queries: [] });
