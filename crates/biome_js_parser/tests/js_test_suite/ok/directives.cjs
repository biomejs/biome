// SCRIPT
"use new"
let a = 10;
"use strict"; // not a directive
function test() {
  'use strict';
  let b = 10;
  'use strict'; // not a directive
}
(function () {
  "use strict";
  "use strict"
    .length; // not a directive
  let c = 10;
  "use strict"; // not a directive
});
let b = () => {
  "use strict";
  let e = 10;
  "use strict";  // not a directive
}
{
  "use strict"; // not a directive
}
