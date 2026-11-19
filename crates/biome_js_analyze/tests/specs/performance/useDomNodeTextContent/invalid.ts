/* should generate diagnostics */
declare const element: HTMLElement;
element.innerText;

declare const div: HTMLDivElement;
div.innerText;

declare const maybeElement: HTMLElement | null;
maybeElement.innerText;

declare const mixed: { innerText: string } | HTMLElement;
mixed.innerText;

function readElement({ innerText }: HTMLElement) {
    return innerText;
}

function writeElement(element: HTMLElement) {
    element.innerText = "Biome";
}

function readDestructuredElement({ item }: { item: HTMLElement }) {
    return item.innerText;
}
