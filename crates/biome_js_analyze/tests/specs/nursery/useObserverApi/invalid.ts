/* should generate diagnostics */

window.addEventListener("resize" as const, () => element.offsetWidth);
window.addEventListener(("scroll" satisfies string), () => element.getBoundingClientRect());
window.addEventListener("resize", (() => element.clientWidth) as EventListener);

const handler = (() => element.offsetHeight) as EventListener;
window.addEventListener("resize", handler);

window.addEventListener("resize", () => element[("offsetWidth" as const)]);
window.addEventListener("scroll", () => element[("getClientRects" as const)]());

window.addEventListener("resize", () => {
    const { [("clientWidth" as const)]: width } = element;
    return width;
});

function listenToElement(element: HTMLElement) {
    window.addEventListener("resize", () => element.offsetWidth);
}

function listenToElementMethod(element: Element) {
    window.addEventListener("scroll", () => element.getBoundingClientRect());
}

const typedElement: HTMLElement = element;
window.addEventListener("resize", () => typedElement.offsetWidth);

class CustomElement extends HTMLElement {}

function listenToCustomElement(element: CustomElement) {
    window.addEventListener("resize", () => element.offsetWidth);
}

class CustomElementWithMember extends HTMLElement {
    customMethod() {}
}

function listenToCustomElementWithMember(element: CustomElementWithMember) {
    window.addEventListener("resize", () => element.offsetWidth);
}

interface CustomSvgElement extends SVGElement {}

function listenToCustomSvgElement(element: CustomSvgElement) {
    window.addEventListener("resize", () => element.offsetWidth);
}

interface BaseElement extends HTMLElement {}
interface IndirectElement extends BaseElement {}

function listenToIndirectElement(element: IndirectElement) {
    window.addEventListener("resize", () => element.offsetWidth);
}

type Size = {
    offsetWidth: number;
};

function listenToSize(size: Size) {
    window.addEventListener("resize", () => size.offsetWidth);
}

function listenToAssertedElement(size: Size) {
    window.addEventListener("resize", () => (size as unknown as HTMLElement).offsetWidth);
}
