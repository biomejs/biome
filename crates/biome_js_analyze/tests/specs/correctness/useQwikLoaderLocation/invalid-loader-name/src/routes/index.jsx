/* should generate diagnostics */
import { routeLoader$ } from '@builder.io/qwik-city';

export const getProductDetails = routeLoader$(async (requestEvent) => {
	const res = await fetch(`https://.../products/${requestEvent.params.productId}`);
	const product = await res.json();
	return product;
});
