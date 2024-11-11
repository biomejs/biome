#!/usr/bin/node
import * as process from "node:process";
import * as util from "node:util";
import { Browser, BrowserErrorCaptureEnum } from "happy-dom";

function parseAriaSpec(doc, { url, version }) {
	const permalink = doc.querySelector("a.u-url")?.href;
	const roleDefs = doc.querySelectorAll(".role");
	const roles = {};
	for (const def of roleDefs) {
		const name = def.id;
		const { deprecatedInVersion, description } = parseDescription(
			def.querySelector("p"),
		);
		// We use the terminology of the ARIA 1.3 specification
		// See <https://www.w3.org/TR/wai-aria-1.3/#Properties>
		roles[name] = {
			description,
			deprecatedInVersion,
			isAbstract: parseBoolean(def.querySelector(".role-abstract")),
			superclassRoles: textContents(
				def.querySelectorAll(".role-parent code") ??
					def.querySelector(".role-parent-head + td code"),
			),
			subclassRoles: textContents(
				def.querySelectorAll(".role-children code") ??
					def.querySelector(".role-children-head + td code"),
			),
			// We added fallback (`??`) because some roles miss the `.role-base` classes.
			baseConcepts: parseConceptList(
				def.querySelector(".role-base") ??
					def.querySelector(".role-base-head + td"),
				version,
			),
			// We added fallback (`??`) because some roles miss the `.role-related` classes.
			relatedConcepts: parseConceptList(
				def.querySelector(".role-related") ??
					def.querySelector(".role-related-head + td"),
				version,
			),
			allowedChildRoles: textContents(
				def.querySelectorAll(".role-mustcontain code") ??
					def.querySelector(".role-mustcontain-head + td code"),
			),
			requiredParentRoles: textContents(
				def.querySelectorAll(".role-scope code") ??
					def.querySelector(".role-scope-head + td code"),
			),
			requiredAttributes: parseAttributeList(
				def.querySelector(".role-required-properties") ??
					def.querySelector(".role-required-properties-head + td"),
			),
			supportedAttributes: parseAttributeList(
				def.querySelector(".role-properties") ??
					def.querySelector(".role-properties-head + td"),
			),
			inheritedAttributes: parseAttributeList(
				def.querySelector(".role-inherited") ??
					def.querySelector(".role-inherited-head + td"),
			),
			prohibitedAttributes: parseAttributeList(
				def.querySelector(".role-disallowed") ??
					def.querySelector(".role-disallowed-head + td"),
			),
			nameFrom: parseNamesFrom(def.querySelector(".role-namefrom")),
			isAccessibleNameRequired: parseBoolean(
				def.querySelector(".role-namerequired") ??
					def.querySelector(".role-namerequired-head + td"),
			),
			hasPresentationalChildren: parseBoolean(
				def.querySelector(".role-childpresentational") ??
					def.querySelector(".role-childpresentational-head + td"),
			),
			implicitValuesForRole: parseImplicitValForRole(
				def.querySelectorAll(".implicit-values code") ??
					def.querySelector(".implicit-values-head + td code"),
			),
		};
	}
	const propertyDefs = doc.querySelectorAll(".property");
	const attributes = {};
	for (const def of propertyDefs) {
		const name = def.id;
		const { deprecatedInVersion, description } = parseDescription(
			def.querySelector("p"),
		);
		attributes[name] = {
			type: "property",
			description,
			deprecatedInVersion,
			relatedConcepts: parseConceptList(
				def.querySelector(".property-related, .state-related"),
				version,
			),
			usedInRoles: textContents(
				def.querySelectorAll(
					".property-applicability code, .state-applicability code",
				),
			),
			inheritsIntoRoles: textContents(
				def.querySelectorAll(
					".property-descendants code, .state-descendants code",
				),
			),
			valueType:
				def.querySelector(".property-value a, .state-value a")?.textContent ??
				null,
			values: parseAttributeValues(
				def.querySelectorAll(".value-name, .value-description"),
			),
		};
	}
	const stateDefs = doc.querySelectorAll(".state");
	for (const def of stateDefs) {
		const name = def.id;
		const { deprecatedInVersion, description } = parseDescription(
			def.querySelector("p"),
		);
		attributes[name] = {
			type: "state",
			description,
			deprecatedInVersion,
			relatedConcepts: parseConceptList(
				def.querySelector(".property-related, .state-related"),
				version,
			),
			usedInRoles: textContents(
				def.querySelectorAll(
					".property-applicability code, .state-applicability code",
				),
			),
			inheritsIntoRoles: textContents(
				def.querySelectorAll(
					".property-descendants code, .state-descendants code",
				),
			),
			valueType:
				def.querySelector(".property-value a, .state-value a")?.textContent ??
				null,
			values: parseAttributeValues(
				def.querySelectorAll(".value-name, .value-description"),
			),
		};
	}
	return {
		permalink,
		url,
		version,
		roles,
		attributes,
	};
}

const DESCRIPTION_REGEX = /(?:\[Deprecated in ARIA ([\d.]+)\])?[ ]?(.+)/s;

function parseDescription(node) {
	const text = node?.textContent;
	if (text != null) {
		const [_, deprecatedInVersion, description] = text.match(DESCRIPTION_REGEX);
		return {
			deprecatedInVersion,
			description: description.trim().replace(/\n[ ]*/g, ""),
		};
	}
	return {};
}

function parseBoolean(node) {
	return node?.textContent === "True";
}

function parseNamesFrom(node) {
	return (
		node?.textContent
			.trim()
			.split("\n")
			.map((name) => name.trim())
			.filter(
				(name) =>
					name === "author" || name === "contents" || name === "prohibited",
			) ?? []
	);
}

function parseImplicitValForRole(nodeList) {
	const arrayed = textContents(nodeList);
	if (arrayed != null) {
		const result = {};
		for (let i = 0; i < arrayed.length; i += 2) {
			const name = arrayed[i];
			const value = arrayed[i + 1];
			result[name] = value;
		}
		return result;
	}
}

function parseConceptList(node, version) {
	if (node != null) {
		const list = textContents(node.querySelectorAll("li"));
		if (list != null) {
			return list.flatMap((attribute) => parseConcept(attribute, version));
		}
		// There is a single attribute (no list)
		return parseConcept(node.textContent, version);
	}
}

/**
 * <input>
 * <input type="checkbox">
 * <input[type="checkbox"]>
 */
const HTML_CONCEPT_REGEX = /<(\w+)(?: ?\[?(\w+)="?(\w+)"?\]?)?>/g;

const MODULE_CONCEPT_REGEX =
	/(DAISY|DOM|Dublin Core|X?HTML|html|JAPI|SMIL|SVG|XForms)\d*(?: \[\1\d*\])? ([a-z]\w*)/gi;
const CONCEPT_IN_MODULE_REGEX =
	/^([a-z]\w*) in \[?(DAISY|DOM|Dublin Core|X?HTML|html|JAPI|SMIL|SVG|XForms)\d*\]?/gi;

const CONCEPT_ATTRIBUTE_IN_MODULE_REGEX =
	/([a-z]\w*) attribute in \[?(DAISY|DOM|Dublin Core|X?HTML|JAPI|SMIL|SVG|XForms)\d*\]?/gi;

function parseConcept(spacedText, version) {
	const text = spacedText.trim();
	const result = [];
	for (const [_, name, attributeName, attributeValue] of text.matchAll(
		HTML_CONCEPT_REGEX,
	)) {
		let attributes;
		if (attributeName != null && attributeValue != null) {
			attributes = { [attributeName]: attributeValue };
		}
		result.push({
			type: "element",
			name: name.toLowerCase(),
			attributes,
			module: "html",
		});
	}
	for (const [_, module, name] of text.matchAll(MODULE_CONCEPT_REGEX)) {
		if (name === "and") {
			// Avoid false positives
			continue;
		}
		result.push({
			type: version === "1.1" ? "any" : "element",
			name: name.toLowerCase(),
			module: module.toLowerCase(),
		});
	}
	for (const [_, name, module] of text.matchAll(CONCEPT_IN_MODULE_REGEX)) {
		result.push({
			type: version === "1.1" ? "any" : "element",
			name: name.toLowerCase(),
			module: module.toLowerCase(),
		});
	}
	for (const [_, name, module] of text.matchAll(
		CONCEPT_ATTRIBUTE_IN_MODULE_REGEX,
	)) {
		result.push({
			type: "attribute",
			name: name.toLowerCase(),
			module: module.toLowerCase(),
		});
	}
	if (result.length === 0) {
		if (text.includes(" ")) {
			result.push({
				type: "text",
				name: text,
			});
		} else {
			result.push({
				type: "role",
				name: text,
				module: "aria",
			});
		}
	}
	return result;
}

function parseAttributeList(node) {
	if (node != null) {
		const list = textContents(node.querySelectorAll("li"));
		if (list != null) {
			return list.map((attribute) => parseAttribute(attribute));
		}
		// There is a single attribute (no list)
		return [parseAttribute(node.textContent)];
	}
}

const ATTRIBUTE_REGEX =
	/([\w\d-]+)(?: \(state\))?(?: \(.*deprecated.*in ARIA ([\d.]+)\))?/;

function parseAttribute(text) {
	const [_, name, deprecatedInVersion] = text.match(ATTRIBUTE_REGEX);
	if (deprecatedInVersion != null) {
		return { name, deprecatedInVersion };
	}
	return name;
}

function parseAttributeValues(nodeList) {
	const arrayed = textContents(nodeList);
	if (arrayed != null) {
		const result = {};
		for (let i = 0; i < arrayed.length; i += 2) {
			const defaultIndex = arrayed[i].indexOf(" (default)");
			const isDefault = defaultIndex !== -1;
			const value = isDefault ? arrayed[i].slice(0, defaultIndex) : arrayed[i];
			result[value] = {
				description: arrayed[i + 1],
				isDefault,
			};
		}
		return result;
	}
}

function textContents(nodeList) {
	if (nodeList != null && nodeList.length > 0) {
		return Array.from(new Set(nodeList)).map((x) => x.textContent);
	}
}

const DEFAULT_ARIA_SSPEC = "wai-aria-1.2";

const SPEC_REGEX = /^[a-z_-]+(\d+(?:\.\d+)*)$/;

async function run({ positionals: [spec = DEFAULT_ARIA_SSPEC] }) {
	if (!SPEC_REGEX.test(spec)) {
		console.error(
			`"${spec}" is not a valid spec name. default: "${DEFAULT_ARIA_SSPEC}"`,
		);
		process.exit(1);
	}
	const [_, version] = spec.match(SPEC_REGEX);
	const url = `https://www.w3.org/TR/${spec}/`;

	const browser = new Browser({
		settings: { errorCapture: BrowserErrorCaptureEnum.processLevel },
	});
	const page = browser.newPage();
	await page.goto(url);

	const parsed = parseAriaSpec(page.mainFrame.document, { url, version });
	const json = JSON.stringify(parsed, null, "  ");
	console.log(json);
}

await run(
	util.parseArgs({
		allowPositionals: true,
	}),
);
