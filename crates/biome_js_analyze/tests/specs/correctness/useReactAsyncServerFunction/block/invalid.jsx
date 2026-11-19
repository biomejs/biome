/* should generate diagnostics */
function addToCart(data) {
	'use server';
}

function requestUsername(formData) {
	'use server';
	const username = formData.get('username');
}

function addToCart(data) {
	"use server";
}

function requestUsername(formData) {
	"use server";
	const username = formData.get('username');
}

const addToCart = (data) => {
	'use server';
}

const requestUsername = (formData) => {
	'use server';
	const username = formData.get('username');
}

const addToCart = (data) => {
	"use server";
}

const requestUsername = (formData) => {
	"use server";
	const username = formData.get('username');
}

const addToCart = function (data) {
	'use server';
}

const requestUsername = function (formData) {
	'use server';
	const username = formData.get('username');
}

const addToCart = function (data) {
	"use server";
}

const requestUsername = function (formData) {
	"use server";
	const username = formData.get('username');
}

export function addToCart(data) {
	'use server';
}

export default function addToCart(data) {
	'use server';
}

export default function (data) {
	'use server';
}

const obj = {
	action() {
		'use server';
	}
};

class Foo {
	constructor() { }

	action() {
		'use server';
	}
}

class Foo {
	static action() {
		'use server';
	}
}

function outer() {
	function inner() {
		'use server';
	}
}

const action = function named(data) {
	'use server';
}

const Invalid1 = () => <form action={() => { 'use server'; }} />

const Invalid2 = () => <form
	action={function () {
		'use server';
		doSomething();
	}}
/>
