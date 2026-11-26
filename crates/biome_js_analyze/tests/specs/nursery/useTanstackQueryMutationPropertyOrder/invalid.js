/* should generate diagnostics */
import { useMutation, useMutation as useSave } from "@tanstack/react-query";
import { useMutation as useVueMutation } from "@tanstack/vue-query";

useMutation({
	mutationFn: (todo) => saveTodo(todo),
	onError: (error, todo, saved) => restoreTodo(saved.previous),
	onMutate: (todo) => ({ previous: readTodo(todo.id) }),
	retry: 3,
});

useSave({
	onSettled: (data, error, todo, saved) => refreshTodo(saved.previous),
	gcTime: 5 * 60 * 1000,
	onMutate: (todo) => ({ previous: readTodo(todo.id) }),
});

useVueMutation({
	onError,
	onSettled,
	onMutate,
});

useMutation({
	onSettled,
	onMutate,
	onError,
});

useMutation({
	onError() {},
	"onMutate": () => {},
});

useMutation({
	"onSettled": () => {},
	async onMutate() {},
});

useMutation({
	onError,
	["onMutate"]: () => {},
});

useMutation(({
	// Roll back the change.
	onError, // rollback
	// Remember the previous value.
	onMutate, // snapshot
}), queryClient);

useMutation({ onError, onMutate });

useMutation({ onError, gcTime, onMutate, });

useMutation({
	onError,
	onMutate
});

useMutation({
	onError,
	...sharedOptions,
	onMutate,
});

useMutation({
	onMutate: first,
	onError,
	onMutate: second,
});

function Component() {
	return useMutation({
		onSettled,
		onMutate,
	});
}

useMutation({ mutationFn,
	onError,
	onMutate });

useMutation({
	mutationFn, // send the request
	onError, onMutate,
});

useMutation({ onError,
	onMutate, // snapshot
});

useMutation({ onError, onMutate // snapshot
});

useMutation({
	onError, gcTime, // cache
	onMutate, retry });
