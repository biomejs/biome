// See https://github.com/biomejs/biome/issues/4677

// some types for the different locations
import TypeOnFunctionParam from "mod";
import TypeOnClassMethodParam from "mod";
import TypeOnClassConstructorParam from "mod";
import TypeOnClassField from "mod";
import TypeOnGlobalVariable from "mod";
import TypeOnFunctionVariable from "mod";
import TypeOnTypeDef from "mod";

/**
 * @typedef {TypeOnTypeDef} TestTypeOnTypeDef 
 */

/**
 * @param {TypeOnFunctionParam} param
 */
function testTypeOnFunction(param) {}

class TestTypeOnClassMethodParam {
	/**
	 * @param {TypeOnClassMethodParam} param
	 */
	method(param) {}
}

class TestTypeOnClassConstructorParam {
	/**
	 * @param {TypeOnClassConstructorParam} param
	 */
	constructor(param) {}
}

class TestTypeOnClassField {
	constructor(param) {
        /**
         * @type {TypeOnClassField}
         */
        this.var = 10;
    }
}

/**
 * @type {TypeOnGlobalVariable}
 */
let testTypeOnGlobalVariable;

function testTypeOnFunctionVariable() {
	/**
	 * @type {TypeOnFunctionVariable}
	 */
	let testTypeOnFunctionVariable;
}


