/* should generate diagnostics */

window.addEventListener("resize", () => element.offsetWidth);
window.addEventListener("resize", function () { return element.clientHeight; });
window.addEventListener("resize", () => element.offsetWidth, { passive: true });
window.addEventListener(`resize`, () => element.offsetWidth);

window.addEventListener("scroll", () => element.getBoundingClientRect());
window.addEventListener("scroll", () => document.documentElement.getClientRects());
window.addEventListener("scroll", () => element?.getBoundingClientRect());
window.addEventListener("scroll", () => element["getBoundingClientRect"]());

window.addEventListener("resize", () => element["clientWidth"]);
window.addEventListener("resize", () => element.offsetWidth += 1);
window.addEventListener("resize", () => element.offsetWidth++);
window.addEventListener("resize", () => window.innerWidth += 1);
window.addEventListener("resize", () => innerHeight++);

window.addEventListener("resize", () => {
    const { offsetWidth } = element;
});
window.addEventListener("resize", () => {
    ({ clientHeight } = element);
});
window.addEventListener("resize", () => {
    const { innerWidth } = window;
});
window.addEventListener("resize", () => {
    ({ height } = visualViewport);
});

window.addEventListener("resize", () => window.innerWidth);
window.addEventListener("resize", () => innerHeight);
window.addEventListener("resize", () => globalThis.innerWidth);
window.addEventListener("resize", () => self.innerHeight);
window.addEventListener("resize", () => visualViewport.width);
window.addEventListener("resize", () => window.visualViewport.height);

addEventListener("resize", () => window.innerWidth);
addEventListener("scroll", () => element.getBoundingClientRect());
scroller.addEventListener("scroll", () => target.getBoundingClientRect());

function resizeHandler() {
    return element.offsetHeight;
}
window.addEventListener("resize", resizeHandler);

function laterReassignedHandler() {
    return element.offsetWidth;
}
window.addEventListener("resize", laterReassignedHandler);
laterReassignedHandler = () => {};

const scrollHandler = () => element.scrollHeight;
window.addEventListener("scroll", scrollHandler);

const functionHandler = function () {
    return element.offsetParent;
};
window.addEventListener("resize", functionHandler);

const eventName = "resize";
window.addEventListener(eventName, () => element.offsetWidth);

function listenToShadowedUndefined(undefined) {
    undefined.addEventListener("resize", () => undefined.offsetWidth);
}
