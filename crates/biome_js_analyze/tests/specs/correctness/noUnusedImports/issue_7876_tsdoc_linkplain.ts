// See https://github.com/biomejs/biome/issues/7876

// some types for the different locations
import type LinkOnFunction from "mod";

import type LinkOnVariable from "mod";
import type LinkOnClass from "mod";

import type LinkOnClassField from "mod";
import type LinkOnClassMethod from "mod";
import type LinkOnClassConstructor from "mod";
import type LinkOnClassAccessor from "mod";
import type LinkOnClassIndexer from "mod";

import type LinkOnInterface from "mod";
import type LinkOnInterfaceField from "mod";
import type LinkOnInterfaceMethod from "mod";
import type LinkOnInterfaceIndexer from "mod";

import type LinkOnEnum from "mod";
import type LinkOnEnumMember from "mod";

import type LinkOnObjectProperty from "mod";


/**
 * {@linkplain LinkOnFunction}
 */
function testLinkOnFunction() { }

/**
 * {@linkplain LinkOnVariable}
 */
const testLinkOnVariable = 3;

/**
 * {@linkplain LinkOnClass}
 */
class TestLinkOnClass { }

class TestLinkOnClassField {
    /**
     * {@linkplain LinkOnClassField}
     */
    field: number;
}

class TestLinkOnClassMethod {
    /**
     * {@linkplain LinkOnClassMethod}
     */
    method(): void { }
}

class TestLinkOnClassConstructor {
    /**
     * {@linkplain LinkOnClassConstructor}
     */
    constructor() { }
}

class TestLinkOnClassAccessor {
    /**
     * {@linkplain LinkOnClassAccessor}
     */
    get accessor(): number { return 0 }
}

class TestLinkOnClassIndexer {
    /**
     * {@linkplain LinkOnClassIndexer}
     */
    [index: number]: string;
}

/**
 * {@linkplain LinkOnInterface}
 */
interface TestLinkOnInterface { }

interface TestLinkOnInterfaceField {
    /**
     * {@linkplain LinkOnInterfaceField}
     */
    field: string;
}

interface TestLinkOnInterfaceMethod {
    /**
     * {@linkplain LinkOnInterfaceMethod}
     */
    method(): string;
}

interface TestLinkOnInterfaceIndexer {
    /**
     * {@linkplain LinkOnInterfaceIndexer}
     */
    [index: number]: string;
}

/**
 * {@linkplain LinkOnEnum}
 */
enum TestLinkOnEnum {
}

enum TestLinkOnEnumMember {
    /**
     * {@linkplain LinkOnEnumMember}
     */
    member
}

const testLinkOnObjectProperty = {
	/**
	 * {@linkplain LinkOnObjectProperty}
	 */
	property: 0,
};
