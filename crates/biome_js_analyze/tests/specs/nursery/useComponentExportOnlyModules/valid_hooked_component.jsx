import React, { memo } from 'react';

export const Component = () => <></>;
export const MemoArrowComponent = memo(() => <></>);
export const MemoNamedComponent = memo(Component);
export const MemoFunctionComponent = memo(function () {});
export const ReactMemoComponent = React.memo(() => <></>);
export const ReactMemoNamedComponent = React.memo(Component);
export const ForwardRefArrowComponent = forwardRef(() => <></>);
export const ForwardRefFunctionComponent = forwardRef(function () {});
export const ReactForwardRefComponent = React.forwardRef(() => <></>);
