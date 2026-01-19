/**
 * Animated spinner component for loading states.
 */

import React, { useState, useEffect } from "react";

const SPINNER_FRAMES = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

interface SpinnerProps {
	/** Message to display next to the spinner */
	message: string;
}

/**
 * Renders an animated spinner with a message.
 */
export function Spinner({ message }: SpinnerProps) {
	const [frameIndex, setFrameIndex] = useState(0);

	useEffect(() => {
		const interval = setInterval(() => {
			setFrameIndex((prev) => (prev + 1) % SPINNER_FRAMES.length);
		}, 80);

		return () => clearInterval(interval);
	}, []);

	return (
		<box flexDirection="row">
			<text fg="#00FFFF">{SPINNER_FRAMES[frameIndex]}</text>
			<text fg="#CCCCCC"> {message}</text>
		</box>
	);
}
