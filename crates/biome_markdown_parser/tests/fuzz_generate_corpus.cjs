#!/usr/bin/env node
// Differential fuzzer corpus generator for Biome's markdown parser.
// Generates random markdown inputs from construct combinators and renders
// reference HTML via commonmark.js.
//
// Usage:
//   node fuzz_generate_corpus.cjs [--count=N] [--seed=N] [--output=path]
//
// Requires `pnpm install` from the repo root (commonmark is a root devDependency).

"use strict";

const { writeFileSync } = require("node:fs");

// Parse CLI args
const args = Object.fromEntries(
  process.argv.slice(2).map((a) => {
    const [k, v] = a.replace(/^--/, "").split("=");
    return [k, v];
  })
);

const count = parseInt(args.count || "1000", 10);
const seed = parseInt(args.seed || "42", 10);
const outputPath = args.output || "corpus.jsonl";

// Load commonmark via require() — relies on cwd having node_modules/commonmark
const { Parser, HtmlRenderer } = require("commonmark");

const parser = new Parser();
const renderer = new HtmlRenderer();

function render(md) {
  return renderer.render(parser.parse(md));
}

// #region Seeded PRNG (xorshift32)
let rngState = seed === 0 ? 1 : seed;
function rand() {
  rngState ^= rngState << 13;
  rngState ^= rngState >> 17;
  rngState ^= rngState << 5;
  return (rngState >>> 0) / 0x100000000;
}
function randInt(min, max) {
  return min + Math.floor(rand() * (max - min + 1));
}
function pick(arr) {
  return arr[randInt(0, arr.length - 1)];
}
function maybe(prob = 0.5) {
  return rand() < prob;
}
// #endregion

// #region Construct combinators

function genParagraph() {
  const words = ["foo", "bar", "baz", "hello", "world", "test", "content"];
  const len = randInt(1, 5);
  return Array.from({ length: len }, () => pick(words)).join(" ") + "\n";
}

function genAtxHeading() {
  const level = randInt(1, 6);
  const text = pick(["Heading", "Title", "Section", "Part"]);
  const trailing = maybe(0.3) ? " " + "#".repeat(level) : "";
  return "#".repeat(level) + " " + text + trailing + "\n";
}

function genSetextHeading() {
  const text = pick(["Foo", "Bar", "Heading"]);
  const marker = maybe(0.5) ? "---" : "===";
  return text + "\n" + marker + "\n";
}

function genThematicBreak() {
  return pick(["---", "***", "___"]) + "\n";
}

function genBulletList() {
  const items = randInt(1, 4);
  const marker = pick(["-", "*", "+"]);
  let result = "";
  for (let i = 0; i < items; i++) {
    result += marker + " " + genInlineContent() + "\n";
  }
  return result;
}

function genOrderedList() {
  const items = randInt(1, 3);
  let result = "";
  for (let i = 0; i < items; i++) {
    result += (i + 1) + ". " + genInlineContent() + "\n";
  }
  return result;
}

function genBlockquote() {
  const lines = randInt(1, 3);
  let result = "";
  for (let i = 0; i < lines; i++) {
    result += "> " + genInlineContent() + "\n";
  }
  return result;
}

function genFencedCode() {
  const fence = maybe(0.5) ? "```" : "~~~";
  const lang = maybe(0.5) ? pick(["js", "rust", "md", ""]) : "";
  const body = pick(["let x = 1;", "code here", "fn main() {}"]);
  return fence + lang + "\n" + body + "\n" + fence + "\n";
}

function genIndentedCode() {
  return "    " + pick(["code line", "let x = 1;", "indented"]) + "\n";
}

function genLinkRefDef() {
  const label = pick(["foo", "bar", "link"]);
  const url = pick(["/url", "https://example.com", "/path"]);
  const title = maybe(0.3) ? ' "' + pick(["title", "my title"]) + '"' : "";
  return "[" + label + "]: " + url + title + "\n";
}

function genInlineContent() {
  const parts = [];
  const len = randInt(1, 4);
  for (let i = 0; i < len; i++) {
    const kind = randInt(0, 6);
    switch (kind) {
      case 0: parts.push(pick(["foo", "bar", "baz", "text", "word"])); break;
      case 1: parts.push("*" + pick(["em", "italic"]) + "*"); break;
      case 2: parts.push("**" + pick(["bold", "strong"]) + "**"); break;
      case 3: parts.push("`" + pick(["code", "x"]) + "`"); break;
      case 4: parts.push("[" + pick(["link", "text"]) + "](url)"); break;
      case 5: parts.push("<" + pick(["span", "b", "i"]) + ">tag</" + pick(["span", "b", "i"]) + ">"); break;
      case 6: parts.push(pick(["foo", "bar"])); break;
    }
  }
  return parts.join(" ");
}

// #endregion

// #region Interaction combinators (the high-value generators)

function genHeadingInList() {
  const heading = maybe(0.5)
    ? "#".repeat(randInt(1, 3)) + " " + pick(["Foo", "Bar"])
    : pick(["Foo", "Bar"]) + "\n  " + pick(["---", "==="]);
  return "- " + heading + "\n";
}

function genSetextInBlockquote() {
  const text = pick(["Foo", "Bar", "Content"]);
  const marker = maybe(0.5) ? "---" : "===";
  return "> " + text + "\n> " + marker + "\n";
}

function genCodeInList() {
  const fence = maybe(0.5) ? "```" : "~~~";
  const indent = "  ";
  return "- item\n\n" + indent + fence + "\n" + indent + "code\n" + indent + fence + "\n";
}

function genInlineHtmlNearBlockquote() {
  // Valid multiline tag (attr on next line, not starting with >)
  const valid = "text <div\nclass=\"a\">ok</div> end.\n";
  // Invalid multiline tag (> at line start = blockquote)
  const invalid = "text <div class=\"a\"\n>ok</div> end.\n";
  return maybe(0.5) ? valid : invalid;
}

function genMixedListMarkers() {
  const m1 = pick(["-", "*", "+"]);
  let m2 = pick(["-", "*", "+"]);
  while (m2 === m1) m2 = pick(["-", "*", "+"]);
  return m1 + " item one\n\n" + m2 + " item two\n";
}

function genNestedListLazyContinuation() {
  const outer = pick(["-", "*"]);
  const inner = pick(["-", "*"]);
  return outer + " outer\n  " + inner + " nested\n  lazy line\n";
}

function genLinkDefWithTrailing() {
  return pick([
    "[valid]: /url\n",
    "[valid-title]: /url \"title\"\n",
    "[invalid]: /url trailing text\n",
    "[angle]: </url> trailing\n",
  ]);
}

function genListWithBlankLines() {
  const marker = pick(["-", "*"]);
  const tight = maybe(0.5);
  let result = marker + " item one\n";
  if (!tight) result += "\n";
  result += marker + " item two\n";
  if (!tight) result += "\n";
  result += marker + " item three\n";
  return result;
}

function genBlockquoteWithContinuation() {
  const lazy = maybe(0.5);
  let result = "> first line\n";
  if (lazy) {
    result += "lazy continuation\n";
  } else {
    result += "> continued\n";
  }
  return result;
}

// #endregion

// #region Document generator

const blockGenerators = [
  { fn: genParagraph, weight: 2 },
  { fn: genAtxHeading, weight: 2 },
  { fn: genSetextHeading, weight: 1 },
  { fn: genThematicBreak, weight: 1 },
  { fn: genBulletList, weight: 2 },
  { fn: genOrderedList, weight: 1 },
  { fn: genBlockquote, weight: 2 },
  { fn: genFencedCode, weight: 1 },
  { fn: genIndentedCode, weight: 1 },
  { fn: genLinkRefDef, weight: 1 },
  // Interaction combinators — higher weight to bias toward interaction bugs
  { fn: genHeadingInList, weight: 3 },
  { fn: genSetextInBlockquote, weight: 3 },
  { fn: genCodeInList, weight: 2 },
  { fn: genInlineHtmlNearBlockquote, weight: 2 },
  { fn: genMixedListMarkers, weight: 2 },
  { fn: genNestedListLazyContinuation, weight: 2 },
  { fn: genLinkDefWithTrailing, weight: 2 },
  { fn: genListWithBlankLines, weight: 2 },
  { fn: genBlockquoteWithContinuation, weight: 2 },
];

const totalWeight = blockGenerators.reduce((sum, g) => sum + g.weight, 0);

function pickWeighted() {
  let r = rand() * totalWeight;
  for (const g of blockGenerators) {
    r -= g.weight;
    if (r <= 0) return g.fn;
  }
  return blockGenerators[blockGenerators.length - 1].fn;
}

function genDocument() {
  const blocks = randInt(1, 5);
  let result = "";
  for (let i = 0; i < blocks; i++) {
    const gen = pickWeighted();
    result += gen();
    if (maybe(0.6)) result += "\n"; // blank line between blocks
  }
  return result;
}

// #endregion

// #region Main

const output = [];
const seen = new Set();

for (let i = 0; i < count; i++) {
  const md = genDocument();

  // Deduplicate
  if (seen.has(md)) continue;
  seen.add(md);

  try {
    const html = render(md);
    output.push(JSON.stringify({ markdown: md, html }));
  } catch {
    // Skip inputs that crash commonmark.js (shouldn't happen)
    continue;
  }
}

writeFileSync(outputPath, output.join("\n") + "\n");
console.log(`Generated ${output.length} test cases (seed=${seed}) → ${outputPath}`);

// #endregion
