// should not generate diagnostics
import type { ComponentProps, FC } from 'react';

export const CardImage: FC<ComponentProps<'img'>> = ({
	 srcSet,
	 ...restProps
 }) => {
	return (
		// biome-ignore lint/a11y/useAltText: suppressed
		<img
			srcSet={srcSet}
			loading="lazy"
			{...restProps}
		/>
	);
};
