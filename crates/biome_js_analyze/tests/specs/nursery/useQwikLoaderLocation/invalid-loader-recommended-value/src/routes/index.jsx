/* should generate diagnostics */
import { routeLoader$ } from '@builder.io/qwik-city';

async function fetcher() {
	const res = await fetch(`https://.../products/${requestEvent.params.productId}`);
	const product = await res.json();
	return product;
}

export const useProductDetails = routeLoader$(fetcher);
