import React, { MouseEvent } from 'react';

function Component() {
    const onClick = (event: MouseEvent) => { };
    const onDblClick = (event: React.MouseEvent) => { };

    return <div onClick={onClick} onDblClick={onDblClick}></div>;
}
