import React, { useState, useEffect, useRef } from "react";
import "./progress.css";

const ProgressBar = ({ duration, label }) => {
    const [progress, setProgress] = useState(0);

    useEffect(() => {
        const startTime = Date.now();

        const updateProgress = () => {
            const elapsed = Date.now() - startTime;
            const newProgress = elapsed / 1000;
            console.log(newProgress, "p");

            if (Math.floor(newProgress) >= duration) {
                clearInterval(interval);
            } else {
                setProgress(newProgress);
            }
        };

        const interval = setInterval(updateProgress, 100);

        return () => {
            clearInterval(interval);
        };
    }, [duration]);

    return (
        <div className="prog-cont">
            <span class="label">{label}</span>
            <div className="progress-bar-container">
                <div className="bar" style={{ width: `calc(3 * ${progress}%)` }}>
                    <span className="time">{progress.toFixed(2)}s</span>
                </div>
            </div>
        </div>
    );
};

export default ProgressBar;
