var r = /[👍]/;
var r = /[\uD83D\uDC4D]/;
var r = /[👍]\\a/;
var r = /(?<=[👍])/;
var r = /[Á]/;
var r = /[Á]/u;
var r = /[\u0041\u0301]/;
var r = /[\u0041\u0301]/u;
var r = /[\u{41}\u{301}]/u;
var r = /[❇️]/;
var r = /[❇️]/u;
var r = /[\u2747\uFE0F]/;
var r = /[\u2747\uFE0F]/u;
var r = /[\u{2747}\u{FE0F}]/u;
var r = /[👶🏻]/;
var r = /[👶🏻]/u;
var r = /[\uD83D\uDC76\uD83C\uDFFB]/u;
var r = /[\u{1F476}\u{1F3FB}]/u;
var r = /[🇯🇵]/;
var r = /[🇯🇵]/i;
var r = /[🇯🇵]/u;
var r = /[\uD83C\uDDEF\uD83C\uDDF5]/u;
var r = /[\u{1F1EF}\u{1F1F5}]/u;
var r = /[👨‍👩‍👦]/;
var r = /[👨‍👩‍👦]/u;
var r = /[\uD83D\uDC68\u200D\uD83D\uDC69\u200D\uD83D\uDC66]/u;
var r = /[\u{1F468}\u{200D}\u{1F469}\u{200D}\u{1F466}]/u;

/[\]👍]/;

// range operator as a normal character in character class
// Issue: https://github.com/biomejs/biome/issues/4950
/[-\u0300]/;
