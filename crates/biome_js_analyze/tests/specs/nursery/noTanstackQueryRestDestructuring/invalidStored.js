/* should generate diagnostics */
import { useQuery } from "@tanstack/react-query";

const stored = useQuery({ queryKey: ["stored"], queryFn: fetchTodos });
const { data: storedData, ...storedRest } = stored;
const props = { ...stored };
