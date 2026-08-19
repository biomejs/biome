<!-- should generate diagnostics -->
<html lang="lorem"></html>
<html lang="en-babab"></html>
<html lang="en-GB-something"></html>
<html lang="zh-Xxxx"></html>
<html lang="zh-Hans-ZZ"></html>
<html lang="en-US-GB-Extra"></html>
<!-- static Vue bindings with invalid language codes should also be flagged -->
<html :lang="'lorem'"></html>
<html v-bind:lang="'en-babab'"></html>
