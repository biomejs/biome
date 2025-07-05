/* should not generate diagnostics */
import React, {Fragment} from "react";

<>
    <React.Fragment key="1"></React.Fragment>
    <Fragment key="1"></Fragment>
</>

<Fragment>
	<Fragment key="1"></Fragment>
</Fragment>

<React.Fragment>
	<React.Fragment key="1"></React.Fragment>
</React.Fragment>
