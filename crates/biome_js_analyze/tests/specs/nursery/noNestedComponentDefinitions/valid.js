/* should not generate diagnostics */
import React, {memo} from 'react';
const CustomLib = {
    memo: (Component) => Component,
}

const customWrapper = (Component) => Component;

function ComponentAsFunctionDeclaration() {}
export default function ComponentAsDefaultFunctionDeclaration() {}
const ComponentAsAnonymousFunctionExpression = function () {};
const ComponentAsNamedFunctionExpression = function unusedName() {};
const ComponentAsArrowFunctionExpression = () => {};
const ComponentAsArrowFunctionExpressionMemo = memo(() => {});
const ComponentAsArrowFunctionExpressionReactMemo = React.memo(function unusedName() {});
const ComponentAsArrowFunctionExpressionCustomLibMemo = CustomLib.memo(() => {});


// Can nest components inside regular functions
function nonComponentFunction() {
    function ComponentAsFunctionDeclaration() {}
    const ComponentAsAnonymousFunctionExpression = function () {};
    const ComponentAsNamedFunctionExpression = function unusedName() {};
    const ComponentAsArrowFunctionExpression = () => {};
    const ComponentAsArrowFunctionExpressionMemo = memo(() => {});
    const ComponentAsArrowFunctionExpressionReactMemo = React.memo(function unusedName() {});
    const ComponentAsArrowFunctionExpressionCustomLibMemo = CustomLib.memo(() => {});
}

// Can nest components inside hooks
function useHookName() {
    function ComponentAsFunctionDeclaration() {}
    const ComponentAsAnonymousFunctionExpression = function () {};
    const ComponentAsNamedFunctionExpression = function unusedName() {};
    const ComponentAsArrowFunctionExpression = () => {};
    const ComponentAsArrowFunctionExpressionMemo = memo(() => {});
    const ComponentAsArrowFunctionExpressionReactMemo = React.memo(function unusedName() {});
    const ComponentAsArrowFunctionExpressionCustomLibMemo = CustomLib.memo(() => {});
}

// Functions with more than one argument are not considered components
function ParentComponent() {
    function ComponentAsFunctionDeclaration(a1, a2) {}
    const ComponentAsAnonymousFunctionExpression = function (a1, a2) {};
    const ComponentAsNamedFunctionExpression = function unusedName(a1, a2) {};
    const ComponentAsArrowFunctionExpression = (a1, a2) => {};
}

// Can nest regular functions inside components
function ParentComponent() {
    const arrowExpressionFunction = () => {};
    function functionDeclaration() {}
    const anonymousFunctionExpression = function () {};
    const namedFunctionExpression = function unusedName() {};
}

// Ignores React wrappers if not assigned to a Pascal-cased variable
function ParentComponent() {
    memo(() => {});
    React.memo(function unusedName() {});
    alert(CustomLib.memo(() => {}));
    const component = memo(() => {});
}

// Ignores unknown wrappers
function ParentComponent() {
    const component = customWrapper(() => {});
    const component2 = customWrapper(function unusedName() {});
    const component3 = customWrapper((param1, param2) => {});
}
