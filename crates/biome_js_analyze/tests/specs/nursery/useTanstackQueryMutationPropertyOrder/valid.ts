/* should not generate diagnostics */
import { type useMutation } from "@tanstack/react-query";
import type { useMutation as useSave } from "@tanstack/react-query";

useMutation({
	onError,
	onMutate,
});

useSave({
	onError,
	onMutate,
});
