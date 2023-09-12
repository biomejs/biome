import React, {Fragment} from "react";

<div>
    {/* invalid */}
    <Fragment>
        <p>Lorem</p>
        <strong>Ipsum</strong>
    </Fragment>

    <React.Fragment>
        <acronym>Lorem</acronym>
        <aside>Ipsum</aside>
    </React.Fragment>

    </*comment*/Fragment>
        <p>Lorem</p>
        <strong>Ipsum</strong>
    </ /*comment*/Fragment>
</div>