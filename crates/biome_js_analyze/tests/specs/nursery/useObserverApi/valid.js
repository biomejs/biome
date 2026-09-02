/* should not generate diagnostics */

window.addEventListener("click", () => element.getBoundingClientRect());
window.addEventListener("scroll", () => updateScrollPosition(window.scrollY));
window.addEventListener("scroll", () => updateScrollPosition(window.pageYOffset));
window.addEventListener("scroll", () => updateScrollPosition(element.scrollTop));
window.addEventListener("scroll", () => updateScrollPosition(element.scrollLeft));

window.addEventListener("resize", () => element.innerWidth);
window.addEventListener("resize", () => custom.visualViewport.width);
window.addEventListener("resize", () => { element.offsetWidth = 1; });
window.addEventListener("resize", () => { (element.offsetWidth) = 1; });
window.addEventListener("resize", () => { window.innerWidth = 1; });
window.addEventListener("resize", () => { innerWidth = 1; });
window.addEventListener("resize", () => { delete element.offsetWidth; });
window.addEventListener("resize", () => { delete (element.offsetWidth); });

window.addEventListener("resize", () => window.offsetWidth);
window.addEventListener("resize", () => document.offsetWidth);
window.addEventListener("resize", () => visualViewport.offsetWidth);
window.addEventListener("scroll", () => window.getBoundingClientRect());
window.addEventListener("scroll", () => document.getBoundingClientRect());
window.addEventListener("scroll", () => visualViewport.getBoundingClientRect());

window.addEventListener("resize", () => {
    const { offsetWidth } = window;
});
window.addEventListener("resize", () => {
    const { offsetWidth } = document;
});
window.addEventListener("resize", () => {
    const { offsetWidth } = visualViewport;
});

window.addEventListener("scroll", () => update());
window.addEventListener("resize", () => update());
window.addEventListener(eventName, () => element.offsetWidth);
window?.addEventListener("resize", () => element.offsetWidth);
window.addEventListener?.("resize", () => element.offsetWidth);
addEventListener?.("resize", () => element.offsetWidth);
window.addEventListener("resize", handler);
window.addEventListener("resize", object.handler);
window.addEventListener("resize");

let mutableHandler = () => element.offsetWidth;
window.addEventListener("resize", mutableHandler);
var variableHandler = () => element.offsetWidth;
window.addEventListener("resize", variableHandler);

function shadowed(addEventListener) {
    addEventListener("resize", () => window.innerWidth);
}

new ResizeObserver((entries) => update(entries)).observe(element);
new IntersectionObserver((entries) => update(entries)).observe(element);

const nestedHandler = () => {
    function nested() {
        return element.offsetWidth;
    }
    nested();
};
window.addEventListener("resize", nestedHandler);

window.addEventListener("resize", () => {
    const object = {
        method() {
            return element.offsetWidth;
        },
    };
    object.method();
});

window.addEventListener("resize", () => () => element.offsetWidth);

const cyclicHandler = cyclicHandler;
window.addEventListener("resize", cyclicHandler);

function reassignedHandler() {
    return element.offsetWidth;
}
reassignedHandler = () => {};
window.addEventListener("resize", reassignedHandler);
