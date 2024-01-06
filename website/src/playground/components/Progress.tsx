import { useEffect, useState } from "react";
import "../../styles/_progress.scss";

const ProgressBar = ({
	duration,
	label,
}: {
	duration: number;
	label: string;
}) => {
	const [progress, setProgress] = useState(0);

	useEffect(() => {
		const startTime = Date.now();

		const updateProgress = () => {
			const elapsed = Date.now() - startTime;
			const newProgress = elapsed / 1000;

			if (Math.floor(newProgress) >= duration) {
				clearInterval(interval);
			} else {
				setProgress(Math.min(newProgress, duration));
			}
		};

		const interval = setInterval(updateProgress, 100);

		return () => {
			clearInterval(interval);
		};
	}, [duration]);

	return (
		<div className="prog-cont">
			<span className="label">{label}</span>
			<div className="progress-bar-container">
				<div className="bar" style={{ width: `calc(3 * ${progress}%)` }}>
					<span className="time">{progress.toFixed(2)}s</span>
				</div>
			</div>
		</div>
	);
};

export default ProgressBar;
