import { useState, useEffect } from 'react';

export const OtherComponent = () => {
	const [stringContent, setString] =
		useState('Something');

	useEffect(() => {
		setString((content) => {
			return `${content} other`;
		});
	}, []);

	return stringContent;
};
