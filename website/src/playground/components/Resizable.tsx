import { createLocalStorage } from "@/playground/utils";
import type React from "react";
import { createRef, useEffect, useMemo, useState } from "react";

interface Props {
	name: string;
	direction: "top" | "right" | "left";
	className: string;
	children: React.ReactNode;
}

const RESIZE_TOLERANCE = 5;
const MINIMUM_SIZE = 100;

type ResizeHandler = {
	styleProperty: string;
	resizingCursor: string;
	calculateDistance: (container: HTMLDivElement, event: MouseEvent) => number;
	calculateSize: (container: HTMLDivElement, event: MouseEvent) => number;
};

const handlers: {
	[K in Props["direction"]]: ResizeHandler;
} = {
	top: {
		resizingCursor: "row-resize",
		styleProperty: "height",
		calculateDistance: (container, event) => container.offsetTop - event.pageY,
		calculateSize: (container, event) =>
			event.pageY - (container.offsetTop + container.clientHeight),
	},
	left: {
		resizingCursor: "col-resize",
		styleProperty: "width",
		calculateDistance: (container, event) => container.offsetLeft - event.pageX,
		calculateSize: (container, event) =>
			container.offsetLeft + container.clientWidth - event.pageX,
	},
	right: {
		resizingCursor: "col-resize",
		styleProperty: "width",
		calculateDistance: (container, event) =>
			container.offsetLeft + container.clientWidth - event.pageX,
		calculateSize: (container, event) => event.pageX - container.offsetLeft,
	},
};

export default function Resizable({
	name,
	direction,
	className,
	children,
}: Props) {
	const sizeStore = useMemo(() => createLocalStorage(`${name}-size`), [name]);

	const [isResizing, setIsResizing] = useState(false);
	const [canResize, setCanResize] = useState(false);

	const [rawSize, setSize] = useState<number | undefined>(
		sizeStore.getNumber(),
	);

	const size =
		rawSize === undefined ? undefined : Math.max(MINIMUM_SIZE, rawSize);

	const ref: React.RefObject<HTMLDivElement> = createRef();

	const handler = handlers[direction];

	const cursor = canResize || isResizing ? handler.resizingCursor : undefined;

	useEffect(() => {
		function onMouseMove(event: MouseEvent) {
			const container = ref.current;
			if (container == null) {
				return;
			}

			const distance = Math.abs(handler.calculateDistance(container, event));

			if (isResizing) {
				event.preventDefault();
				const size = Math.abs(handler.calculateSize(container, event));
				setSize(size);
				sizeStore.set(size);
			}

			if (distance <= RESIZE_TOLERANCE) {
				setCanResize(true);
			} else if (canResize) {
				setCanResize(false);
			}
		}

		function onContextMenu(e: MouseEvent) {
			if (isResizing || canResize) {
				e.preventDefault();
				sizeStore.clear();
				setSize(undefined);
				setCanResize(false);
				setIsResizing(false);
			}
		}

		function onMouseDown() {
			if (canResize) {
				setIsResizing(true);
			}
		}

		function onMouseUp() {
			setIsResizing(false);
		}

		window.addEventListener("contextmenu", onContextMenu);
		window.addEventListener("mousedown", onMouseDown);
		window.addEventListener("mouseup", onMouseUp);
		window.addEventListener("mousemove", onMouseMove);

		return () => {
			window.removeEventListener("contextmenu", onContextMenu);
			window.removeEventListener("mousedown", onMouseDown);
			window.removeEventListener("mouseup", onMouseUp);
			window.removeEventListener("mousemove", onMouseMove);
		};
	}, [ref, isResizing, canResize, handler, sizeStore]);

	useEffect(() => {
		if (cursor === undefined) {
			document.body.style.removeProperty("cursor");
		} else {
			document.body.style.cursor = cursor;
		}

		return () => {
			document.body.style.removeProperty("cursor");
		};
	}, [cursor]);

	return (
		<div
			ref={ref}
			className={className}
			style={{
				[handler.styleProperty]: size,
				flexShrink: size === undefined ? undefined : 0,
			}}
		>
			{children}
		</div>
	);
}
