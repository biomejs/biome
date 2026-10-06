/* should not generate diagnostics */
async function addToCart(data) {
	'use server';
}

async function requestUsername(formData) {
	'use server';
	const username = formData.get('username');
}

async function addToCart(data) {
	"use server";
}

async function requestUsername(formData) {
	"use server";
	const username = formData.get('username');
}

function addToCart(data) {
	console.log("test");
	'use server';
}

function requestUsername(formData) {
	const username = formData.get('username');
	'use server';
}

function addToCart(data) {
	console.log("use server");
}

function requestUsername(formData) {
	console.log("use server");
	const username = formData.get('username');
}

const addToCart = async (data) => {
	'use server';
}

const requestUsername = async (formData) => {
	'use server';
	const username = formData.get('username');
}

const addToCart = async (data) => {
	"use server";
}

const requestUsername = async (formData) => {
	"use server";
	const username = formData.get('username');
}

const addToCart = (data) => {
	console.log("test");
	'use server';
}

const requestUsername = (formData) => {
	const username = formData.get('username');
	'use server';
}

const addToCart = (data) => {
	console.log("use server");
}

const requestUsername = (formData) => {
	console.log("use server");
	const username = formData.get('username');
}

const addToCart = async function (data) {
	'use server';
}

const requestUsername = async function (formData) {
	'use server';
	const username = formData.get('username');
}

const addToCart = async function (data) {
	"use server";
}

const requestUsername = async function (formData) {
	"use server";
	const username = formData.get('username');
}

const addToCart = function (data) {
	console.log("test");
	'use server';
}

const requestUsername = function (formData) {
	const username = formData.get('username');
	'use server';
}

const addToCart = function (data) {
	console.log("use server");
}

const requestUsername = function (formData) {
	console.log("use server");
	const username = formData.get('username');
}

async function addToCart(data) {
	`use server`;
}

function addToCart(data) {
	`use server`;
}

const addToCart = async (data) => {
	`use server`;
}

const addToCart = (data) => {
	`use server`;
}

const addToCart = async function (data) {
	`use server`;
}

const addToCart = function (data) {
	`use server`;
}

const addToCart = async function* (data) {
	'use server';
}

const addToCart = async function* (data) {
	"use server";
}

const addToCart = function* (data) {
	'use server';
}

const addToCart = function* (data) {
	"use server";
}

function* addToCart(data) {
	'use server';
}

async function* addToCart(data) {
	'use server';
}

export async function addToCart(data) {
	'use server';
}

export default async function addToCart(data) {
	'use server';
}

export default async function (data) {
	'use server';
}

const validObj1 = {
	async action() {
		'use server';
	}
};

const validObj2 = {
	async action() {
		'use server';
		const x = 1;
	}
};

class Foo {
	constructor() {
		'use server';
	}
}

class Foo {
	constructor() { }

	async action() {
		'use server';
	}
}

class Foo {
	static async action() {
		'use server';
	}
}

function outer() {
	async function inner() {
		'use server';
	}
}

const action = async function named(data) {
	'use server';
}

function addToCart(data) {
	'use strict';
	console.log('use server');
}

function empty() { }

const fn = () => 'use server';

const Valid1 = () => <form action={async () => { 'use server'; }} />

const Valid2 = () => <button onClick={async () => { 'use server'; doSomething(); }} />

async function action() {
	'use strict';
	'use server';
}

function action() {
	'use strict';
	'use server';
}
