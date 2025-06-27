// should not generate diagnostics
import type { FC } from 'react';

export const Test: FC = () => {
	return (
		<li>
			{/** biome-ignore lint/a11y/useSemanticElements: - */}
			<div
				role="checkbox"
				onClick={() => onSelect(id)}
				className={css.button}
				aria-label={'a cool aria-label'}
				aria-checked={selected}
				tabIndex={0}
			>
				Nothing
			</div>
		</li>
	);
};
