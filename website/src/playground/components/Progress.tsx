import "@/styles/_progress.scss";
import { useEffect, useState } from "react";

const ProgressBar = ({
	label,
	duration,
	maxDuration,
	color,
}: {
	label: string;
	duration: number;
	maxDuration: number;
	color: string;
}) => {
	const [progress, setProgress] = useState(0);
	// Calculate the relative progress based on maxDuration
	const maxRelativeProgress = (duration / maxDuration) * 100;

	useEffect(() => {
		const startTime = Date.now();

		const updateProgress = () => {
			const elapsed = Date.now() - startTime;
			const elapsedSeconds = elapsed / 1000;
			// Calculate current progress based on elapsed time and relative maximum
			const newProgress = (elapsedSeconds / duration) * maxRelativeProgress;

			if (newProgress >= maxRelativeProgress) {
				// Ensure progress does not exceed calculated relative maximum
				setProgress(maxRelativeProgress);
				clearInterval(interval);
			} else {
				setProgress(newProgress);
			}
		};

		const interval = setInterval(updateProgress, 100);

		return () => {
			clearInterval(interval);
		};
	}, [duration, maxRelativeProgress]);

	return (
		<div className="prog-cont">
			<span className="label">{label}</span>
			<div className="progress-bar-container">
				<div
					className="bar"
					style={{ width: `${progress}%`, backgroundColor: `${color}` }}
				>
					<span className="time">
						{((progress / maxRelativeProgress) * duration).toFixed(2)}s
					</span>
				</div>
			</div>
		</div>
	);
};

const ProgressBarContainer = ({
	data,
}: { data: Array<{ duration: number; label: string; color: string }> }) => {
	const maxDuration = data.reduce(
		(max, { duration }) => Math.max(max, duration),
		0,
	);

	return (
		<div>
			{data.map(({ duration, label, color }) => (
				<ProgressBar
					duration={duration}
					label={label}
					maxDuration={maxDuration}
					color={color}
					key={label}
				/>
			))}
		</div>
	);
};

export default ProgressBarContainer;
