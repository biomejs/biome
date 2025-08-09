import React, {memo} from 'react';
const CustomLib = {
    memo: (Component) => Component,
    forwardRef: (Component) => Component,
}

let ComponentVar;

function ParentComponentAsFunctionDeclaration() {
    function ComponentAsFunctionDeclaration() {}
    function ComponentAsFunctionDeclarationWithParam(param) {}
    const ComponentAsAnonymousFunctionExpression = function () {};
    const ComponentAsAnonymousFunctionExpressionWithParam = function (param) {};
    const ComponentAsNamedFunctionExpression = function unusedName() {};
    const ComponentAsNamedFunctionExpressionWithParam = function unusedName(param) {};
    const ComponentAsArrowFunctionExpression = () => {};
    const ComponentAsArrowFunctionExpressionWithParam = (param) => {};
    ComponentVar = () => {};
    ComponentVar = (param) => {};
    const ComponentAsArrowFunctionExpressionMemo = memo(() => {});
    const ComponentAsArrowFunctionExpressionMemoWithParam = memo((param) => {});
    const ComponentAsArrowFunctionExpressionReactMemo = React.memo(function unusedName() {});
    const ComponentAsArrowFunctionExpressionCustomLibMemo = CustomLib.memo(() => {});
    const ComponentAsArrowFunctionExpressionCustomLibForwardRef = CustomLib.forwardRef((param1, param2) => {});
}

export default function ParentComponentAsDefaultFunctionDeclaration() {
    function ComponentAsFunctionDeclaration() {}
};

const ComponentAsAnonymousFunctionExpression = function (param) {
    function ComponentAsFunctionDeclaration() {}
};

const ComponentAsNamedFunctionExpression = function unusedName() {
    function ComponentAsFunctionDeclaration() {}
};

const ComponentAsArrowFunctionExpression = () => {
    function ComponentAsFunctionDeclaration() {}
};

const ComponentAsArrowFunctionExpressionMemo = memo(() => {
    function ComponentAsFunctionDeclaration() {}
});

const ComponentAsArrowFunctionExpressionReactMemo = React.memo(function unusedName() {
    function ComponentAsFunctionDeclaration() {}
});

const ComponentAsArrowFunctionExpressionReactForwardRef = React.forwardRef(function unusedName(
    param1,
    param2
) {
    function ComponentAsFunctionDeclaration() {}
});
