import { isMobile, toggleMobileSidebar } from "./mobile";

const originalTitle = document.title;

const tocSidebar: HTMLElement = document.querySelector(".toc-sidebar")!;

const tocLinks: HTMLAnchorElement[] = Array.from(
	document.querySelectorAll(".toc a"),
);

const headingElements: {
	heading: HTMLElement;
	link: HTMLAnchorElement;
}[] = tocLinks.map((link) => {
	return {
		heading: document.querySelector(
			`[id="${String(link.getAttribute("href")).slice(1)}"]`,
		)!,
		link,
	};
});

type CalculatedHeading = {
	id: string;
	link: HTMLAnchorElement;
	titles: string[];
	level: number;
	start: number;
	end: number;
};

/**
 * @typedef {Object}
 */

class Manager {
	constructor() {
		this.headingsCalculated = [];
		this.hasInitializedHeadings = false;
		this.lastActiveHeading = undefined;
		this.isNavCollapsed = false;
		this.navHeight = undefined;
	}

	headingsCalculated: CalculatedHeading[];
	hasInitializedHeadings: boolean;
	navHeight: number | undefined;
	isNavCollapsed: boolean;
	lastActiveHeading: undefined | number;

	handleTocClick(event: MouseEvent) {
		const target = event.target;
		event.preventDefault();
		if (!(target instanceof HTMLElement)) {
			return;
		}

		if (target.hasAttribute("href")) {
			const hash = target.getAttribute("href") ?? "";
			window.location.hash = hash;
			this.scrollToHeading(hash);

			if (isMobile) {
				toggleMobileSidebar();
			}
		}
	}

	/**
	 * @returns {number}
	 */
	getScrollY() {
		return scrollY + this.getScrollOffset();
	}

	/**
	 * @returns {number}
	 */
	getScrollOffset() {
		let offset = 0;

		// Account for header
		offset += isMobile ? 64 : 80;

		// Give everything a tiny bit of margin so it's not up against the edges
		offset += 20;

		return offset;
	}

	getHeadingTop(heading: HTMLElement): number {
		return heading.offsetTop - this.getScrollOffset();
	}

	calculateHeading(i: number, stack: CalculatedHeading[]): CalculatedHeading {
		const { heading, link } = headingElements[i]!;
		const id = heading.getAttribute("id")!;

		// Extract the level from the H tag
		const level = Number(heading.tagName[1]);

		// Get the headings above this one for us in document.title
		let titles: string[] = [(heading.textContent ?? "").trim()];
		for (let i = stack.length - 1; i >= 0; i--) {
			const heading = stack[i]!;
			if (heading.level < level) {
				titles = heading.titles.concat(titles);
				break;
			}
		}

		// Calculate when this heading ends. It's either at the beginning of the next heading, or page bottom.
		const start = this.getHeadingTop(heading);
		let end;

		const nextHeading = headingElements[i + 1];
		if (nextHeading) {
			end = this.getHeadingTop(nextHeading.heading);
		} else {
			end = document.body.clientHeight;
		}

		return { level, id, titles, link, start, end };
	}

	calculateHeadingsPositions() {
		// Don't calculate heading positions unless we've scrolled down
		if (!this.hasInitializedHeadings && scrollY <= 100) {
			return;
		}

		// If we've calculated all the headings then we just need to validate the last one
		// and if it's the same we can skip.
		if (this.hasInitializedHeadings) {
			const i = headingElements.length - 1;
			const existing = this.headingsCalculated[i];
			if (existing !== undefined) {
				const recalculated = this.calculateHeading(i, []);
				if (
					recalculated.start === existing.start &&
					recalculated.end === existing.end
				) {
					return;
				}
			}
		}

		this.hasInitializedHeadings = true;
		this.headingsCalculated = [];
		for (let i = 0; i < headingElements.length; i++) {
			this.headingsCalculated.push(
				this.calculateHeading(i, this.headingsCalculated),
			);
		}
	}

	/**
	 * Check if a heading is currently in view
	 */
	isVisibleHeading(i: number): boolean {
		const { start, end } = this.headingsCalculated[i]!;
		const scrollY = this.getScrollY();
		return scrollY >= start && scrollY <= end;
	}

	toggleActiveHeading(i: number, activating: boolean) {
		const { link, titles } = this.headingsCalculated[i]!;

		// Only automatically rewrite the heading on the homepage
		if (location.pathname === "/") {
			if (activating) {
				document.title = `${titles.join(": ")} — ${originalTitle}`;
				history.replaceState({}, "", link.getAttribute("href"));
			} else {
				document.title = originalTitle;
			}
		}

		let target: Element | null = link;
		while (target != null && target.tagName !== "DIV") {
			if (target.tagName === "LI") {
				target.classList.toggle("active");
			}
			target = target.parentElement;
		}
	}

	/**
	 * Computing the heading positions is expensive so we only do it when we absolutely have to.
	 */
	ensureCalculatedHeadings() {
		if (!this.hasInitializedHeadings) {
			this.calculateHeadingsPositions();
		}
	}

	/**
	 * Triggered on window resize and scroll. This needs to be very fast, avoiding DOM inspection completely by caching
	 * and then validating.
	 *
	 * This checks if we should collapse the navigation, and what heading to highlight in the table of contents
	 */

	refresh() {
		if (this.lastActiveHeading !== undefined) {
			if (this.isVisibleHeading(this.lastActiveHeading)) {
				return;
			}
			this.toggleActiveHeading(this.lastActiveHeading, false);
			this.lastActiveHeading = undefined;
		}

		this.ensureCalculatedHeadings();
		for (let i = 0; i < this.headingsCalculated.length; i++) {
			if (this.isVisibleHeading(i)) {
				// Set the heading as active
				this.lastActiveHeading = i;
				this.toggleActiveHeading(i, true);

				// Make sure TOC link is visible
				let linkTop =
					this.headingsCalculated[i]!.link.offsetTop - tocSidebar.offsetTop;
				if (i === 0) {
					linkTop = 0;
				}
				const visibleStart = tocSidebar.scrollTop;
				const visibleEnd = tocSidebar.scrollTop + tocSidebar.clientHeight;
				const isVisible = linkTop > visibleStart && linkTop < visibleEnd;
				if (!isVisible) {
					tocSidebar.scrollTop = linkTop;
				}

				break;
			}
		}
	}

	scrollToHeading(hash: string, callback?: undefined | (() => void)): boolean {
		// Allow passing in raw link href
		const id = hash.replace(/^(#)/, "");

		const heading = document.getElementById(id);
		if (!heading) {
			return false;
		}

		if (callback !== undefined) {
			callback();
		}

		heading.setAttribute("tabindex", "-1");
		heading.focus();

		this.ensureCalculatedHeadings();
		window.scrollTo(0, this.getHeadingTop(heading));

		return true;
	}

	/**
	 * Fully scroll and copy hash to tech when clicking an anchor next to a heading
	 */
	handleHeadingAnchorClick(event: MouseEvent, target: HTMLElement) {
		event.preventDefault();

		const hash = target.getAttribute("href") ?? "";
		window.location.hash = hash;
		this.scrollToHeading(hash);

		if (navigator.clipboard !== undefined) {
			navigator.clipboard.writeText(window.location.href);
		}

		// Only another copied text can appear here so delete it if it exists
		if (target.nextElementSibling != null) {
			target.nextElementSibling.remove();
		}

		const copied = document.createElement("span");
		copied.classList.add("header-copied");
		copied.textContent = "Copied to clipboard";
		target.parentElement?.appendChild(copied);
		requestAnimationFrame(() => {
			requestAnimationFrame(() => {
				copied.style.opacity = "0";
			});
		});
		copied.addEventListener("transitionend", () => {
			copied.remove();
		});
	}

	/**
	 * Intercept link clicks, if they are just hashes on the current page then
	 * just scroll.
	 */
	handleAnchorClick(event: MouseEvent, target: HTMLElement) {
		const maybeHref = target.getAttribute("href");
		if (maybeHref == null) {
			return;
		}

		let href: string = maybeHref;

		// Remove current origin
		if (href.startsWith(location.origin)) {
			href = href.slice(location.origin.length);
		}

		// Remove current pathname
		if (href.startsWith(location.pathname)) {
			href = href.slice(location.pathname.length);
		}

		// If href starts with a hash then it's referring to the current page
		if (href[0] !== "#") {
			return;
		}

		this.scrollToHeading(href, () => {
			event.preventDefault();
			location.hash = href;
		});
	}

	handleGlobalClick(event: MouseEvent) {
		const { target } = event;
		if (!(target instanceof HTMLElement)) {
			return;
		}

		if (event.ctrlKey || event.metaKey) {
			return;
		}

		if (target.closest(".toc") != null) {
			this.handleTocClick(event);
		} else if (target.matches('.content a[href^="#"]')) {
			this.handleHeadingAnchorClick(event, target);
		} else if (target.matches("a")) {
			this.handleAnchorClick(event, target);
		}
	}

	attach() {
		this.refresh();

		if (window.location.hash !== "") {
			this.scrollToHeading(window.location.hash);
		}

		window.addEventListener("scroll", this.refresh.bind(this), {
			passive: true,
		});
		window.addEventListener("resize", this.refresh.bind(this), {
			passive: true,
		});
		window.addEventListener(
			"resize",
			this.calculateHeadingsPositions.bind(this),
			{ passive: true },
		);

		document.addEventListener(
			"click",
			this.handleGlobalClick.bind(this),
			false,
		);
	}
}

if (tocLinks.length > 0) {
	const manager = new Manager();

	window.addEventListener("DOMContentLoaded", () => {
		manager.attach();
	});
}
