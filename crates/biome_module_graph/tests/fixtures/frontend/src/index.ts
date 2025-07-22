import { sharedFoo } from "shared";
import { useSWRConfig } from "swr";

import { bar } from "./bar";

const { mutate } = useSWRConfig();
const mutateResult = mutate("/v1/endpoint");
