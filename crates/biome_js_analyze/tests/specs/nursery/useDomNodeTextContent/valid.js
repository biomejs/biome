/* should not generate diagnostics */
const text = node.textContent;
const computed = node["textContent"];
const maybe = node?.textContent;
const maybeComputed = node?.["textContent"];

node.textContent = "Biome";
node[key] = "Biome";

const {textContent} = node;
const {"innerText": textValue} = node;

({innerText} = node);
