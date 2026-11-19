<template>
  <div>before<span>inline</span>after</div>
  <div>before<span aria-label="long descriptive label">inline</span>after</div>
  <div>before<meter value=".5" aria-label="progress"></meter>after</div>
  <div>before <meter value=".5"></meter>after with enough text to wrap</div>
  <div>before<meter value=".5"></meter> after with source whitespace</div>
  <div>before<img src="a-very-long-image-name.png" />after</div>
</template>

<template><div>before<span>inline</span>after</div><div>before<span aria-label="long descriptive label">inline</span>after</div><div>before<meter value=".5" aria-label="progress"></meter>after</div><div>before <meter value=".5"></meter>after with enough text to wrap</div><div>before<meter value=".5"></meter> after with source whitespace</div><div>before<img src="a-very-long-image-name.png"/>after</div></template>
