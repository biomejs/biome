/* should generate diagnostics */

// IIFE with arrow function returning a Promise
function pattern1() {
	(() => {
		return new Promise((resolve) => resolve('iife'));
	})();
}

// IIFE with async arrow function
function pattern2() {
	(async () => {
		return 'value';
	})();
}

// IIFE with regular function returning a Promise
function pattern3() {
	(function() {
		return new Promise((resolve) => resolve('iife'));
	})();
}

// IIFE with async regular function
function pattern4() {
	(async function() {
		return 'value';
	})();
}

// Top-level IIFE returning a Promise
(() => {
	return new Promise((resolve) => resolve('iife'));
})();

// Top-level async IIFE
(async () => {
	return 'value';
})();
