/* should generate diagnostics */
import { useMutation } from "@tanstack/react-query";

useMutation<Todo, Error, Todo, { previous: Todo }>({
	mutationFn: (todo) => saveTodo(todo),
	onError: (error, todo, saved) => restoreTodo(saved?.previous),
	onMutate: (todo) => ({ previous: readTodo(todo.id) }),
});

useMutation!({
	onSettled,
	onMutate,
});
