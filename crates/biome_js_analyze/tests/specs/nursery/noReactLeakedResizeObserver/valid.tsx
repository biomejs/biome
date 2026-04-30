/* should not generate diagnostics */
import { useEffect, useRef } from 'react';

function Valid1() {
	useEffect(() => {
		const observer = new ResizeObserver(() => {});
		observer.observe(document.body);
		return () => {
			observer.disconnect();
		}
	}, []);

	return <div />;
}

function Valid2() {
	useEffect(() => {
		const observer = new ResizeObserver(() => {}) as ResizeObserver;
		observer.observe(document.body);
		return () => {
			observer.disconnect();
		}
	}, []);

	return <div />;
}

function Valid3() {
	useEffect(() => {
		const observer = new ResizeObserver(() => {});
		observer.observe(document.body as HTMLElement);
		return () => {
			observer.unobserve(document.body);
		}
	}, []);

	return <div />;
}

function Valid4() {
	const ref = useRef<HTMLDivElement>(null);

	useEffect(() => {
		if (!ref.current) return;
		const ro = new ResizeObserver(() => console.log('resize'));
		ro.observe(ref.current);
		return () => ro.disconnect();
	}, []);

	return <div ref={ref} />;
}

function Valid5() {
	useEffect(() => {
		const observer = new ResizeObserver(() => {});
		observer.observe(document.body);
		return () => {
			observer.unobserve(document.body);
			observer.disconnect();
		}
	}, []);

	return <div />;
}

function Valid6() {
	useEffect(() => {
		const observer = new ResizeObserver(() => {});
		observer.observe(document.body);
		observer.observe(document.querySelector('.selector')!);
		return () => {
			observer.disconnect();
		}
	}, []);

	return <div />;
}

function Valid7() {
	useEffect(() => {
		const observer = new ResizeObserver(() => {});
		observer.observe(document.body);
		observer.observe(document.querySelector('.selector')!);
		return () => {
			observer.unobserve(document.body);
			observer.unobserve(document.querySelector('.selector')!);
			observer.disconnect();
		}
	}, []);

	return <div />;
}

function Valid8() {
	useEffect(() => {
		const observer = new ResizeObserver(() => {});
		observer.observe(document.body);
		return () => {
			observer.unobserve(document.body);
		}
	}, []);

	return <div />;
}

function Valid9() {
	useEffect(() => {
		const scrollRoot = scrollRootRef.current;
		if (!scrollRoot) {
			return undefined;
		}

		const resizeObserver = new ResizeObserver(getAndSetScrollOffsets);
		resizeObserver.observe(scrollRoot);

		return () => {
			resizeObserver.unobserve(scrollRoot);
		};
	}, [elementRef, scrollRootRef]);

	return <div />;
}

function Valid10() {
	useEffect(() => {
		const observer = new ResizeObserver(() => {});
		for (const element of document.querySelectorAll('.selector')) {
			observer.observe(element);
		}
		return () => {
			observer.disconnect();
		}
	}, []);

	return <div />;
}

function Valid11() {
	useEffect(() => {
		const observer = new ResizeObserver(() => {});
		Array.from(document.querySelectorAll('.selector')).forEach(element => {
			observer.observe(element);
		});
		return () => {
			observer.disconnect();
		}
	}, []);

	return <div />;
}

function Valid12() {
	const observerRef = useRef<ResizeObserver>(new ResizeObserver(() => {}));
	useEffect(() => {
		const observer = observerRef.current;
		if (!observer) return;
		observer.observe(document.body);
		observer.observe(document.querySelector('.selector')!);
		return () => {
			observer.unobserve(document.body);
			observer.unobserve(document.querySelector('.selector')!);
		}
	}, []);

	return <div />;
}

function Valid13() {
	const observerRef = useRef<ResizeObserver>(new ResizeObserver(() => {}));
	useEffect(() => {
		observerRef.current.observe(document.body);
		observerRef.current.observe(document.querySelector('.selector')!);
		return () => {
			observerRef.current.unobserve(document.body);
			observerRef.current.unobserve(document.querySelector('.selector')!);
		}
	}, []);

	return <div />;
}
