// should generate diagnostics
import type { ComponentProps, FC } from 'react';

export const CardImage: FC<ComponentProps<'img'>> = ({
	 srcSet,
	 ...restProps
 }) => {
	return (
		<img
			srcSet={srcSet}
			loading="lazy"
			{...restProps}
		/>
	);
};
