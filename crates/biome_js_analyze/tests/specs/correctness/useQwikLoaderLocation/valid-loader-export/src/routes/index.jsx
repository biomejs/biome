/* should not generate diagnostics */
import { routeLoader$ } from '@builder.io/qwik-city';
export { useFormLoader };
const useFormLoader = routeLoader$(() => {
	return null;
});
