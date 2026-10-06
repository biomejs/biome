/* should generate diagnostics */
const text = node.innerText;
const computed = node["innerText"];
const maybe = node?.innerText;
const maybeComputed = node?.["innerText"];

node.innerText = "Biome";
node["innerText"] += "!";

const {innerText} = node;
const {innerText = ""} = node;
const {innerText: textValue} = node;

function readNode({innerText}) {
    return innerText;
}
