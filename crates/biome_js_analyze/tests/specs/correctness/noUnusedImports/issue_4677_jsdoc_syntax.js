// See https://github.com/biomejs/biome/issues/4677

// https://jsdoc.app/tags-type

import SymbolName from "mod";
import MultipleTypes from "mod";
import ArraysGeneric from "mod";
import ArraysBrackets from "mod";
import AnyGenerics from "mod";
import ObjectProperties from "mod";
import Nullable from "mod";
import NonNullable from "mod";
import VarArgs from "mod";
import OptionalParameter from "mod";

/**
 * @type {SymbolName}
 */
let testSymbolName;

/**
 * @type {(number|MultipleTypes)}
 */
let testMultipleTypes;

/**
 * @type {Array.<ArraysGeneric>}
 */
let testArraysGeneric;

/**
 * @type {ArraysBrackets[]}
 */
let testArraysBrackets;

/**
 * @type {Object.<string, AnyGenerics>}
 */
let testAnyGenerics;

/**
 * @type {{a:number, b:ObjectProperties}}
 */
let testObjectProperties;

/**
 * @type {?Nullable}
 */
let testNullable;

/**
 * @type {!NonNullable}
 */
let testNonNullable;

/**
 * @param {...VarArgs} params
 */
function testVarArgs(params) {}

/**
 * @param {OptionalParameter=} optional
 */
function testOptionalParameter(optional) {}