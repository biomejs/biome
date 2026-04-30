/* should generate diagnostics */
import { useEffect } from 'react';

function Invalid1() {
	useEffect(() => {
		const observer = new IntersectionObserver(() => { });
		observer.observe(document.body);
	}, []);

	return <div />;
}

function Invalid2() {
	const ref = useRef<HTMLDivElement>(null);

	useEffect(() => {
		if (!ref.current) return;
		const ro = new IntersectionObserver(() => console.log('resize'));
		ro.observe(ref.current);
	}, []);

	return <div ref={ref} />;
}

function Invalid3() {
	useEffect(() => {
		new IntersectionObserver(() => {});
	}, []);

	return <div />;
}

function Invalid4() {
	useEffect(() => {
		const observer = new IntersectionObserver(() => {});
		observer.observe(document.body);
		observer.observe(document.querySelector('.selector')!);
		return () => {
			observer.unobserve(document.body);
		}
	}, []);

	return <div />;
}

function Invalid5() {
	useEffect(() => {
		const observer = new IntersectionObserver(() => {}) as IntersectionObserver;
		observer.observe(document.body);
	}, []);

	return <div />;
}

function Invalid6() {
	useEffect(() => {
		const observer = new IntersectionObserver(() => {});
		for (const element of document.querySelectorAll('.selector')) {
			observer.observe(element);
		}
		return () => {
			for (const element of document.querySelectorAll('.selector')) {
				observer.unobserve(element);
			}
		}
	}, []);

	return <div />;
}

function Invalid7() {
	useEffect(() => {
		const observer = new IntersectionObserver(() => {});
		Array.from(document.querySelectorAll('.selector')).forEach(element => {
			observer.observe(element);
		});
		return () => {
			Array.from(document.querySelectorAll('.selector')).forEach(element => {
				observer.unobserve(element);
			});
		}
	}, []);

	return <div />;
}

function Invalid8() {
	useEffect(() => {
		const observer = new IntersectionObserver(() => {});
		(observer.observe as any)(document.body);
	}, []);

	return <div />;
}
