/* should not generate diagnostics */

function listenToNumber(innerWidth: number) {
    window.addEventListener("resize", () => innerWidth);
}

type ViewportLike = {
    width: number;
};

function listenToViewportLike(viewport: ViewportLike) {
    window.addEventListener("resize", () => viewport.width);
}

function listenToTypedWindow(window_: Window) {
    window_.addEventListener("resize", () => window_.innerWidth);
}

function listenToDynamicEvent(eventName: "resize" | "scroll") {
    window.addEventListener(eventName, () => element.offsetWidth);
}

function listenToTypedEvent(eventName: "scroll") {
    window.addEventListener(eventName, () => element.getBoundingClientRect());
}

function listenToAssertedEvent(eventName: string) {
    window.addEventListener(eventName as "resize", () => element.offsetWidth);
}

function writeToElement(element: HTMLElement) {
    window.addEventListener("resize", () => {
        (element.offsetWidth) = 1;
        (element.offsetWidth as number) = 1;
        (element.offsetWidth!) = 1;
        (<number>element.offsetWidth) = 1;
    });
}

({}).addEventListener("resize", () => window.innerWidth);
window.addEventListener("resize", () => ({}).offsetWidth);
window.addEventListener("scroll", () => [].getBoundingClientRect());
