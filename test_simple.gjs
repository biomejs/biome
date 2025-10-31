import Component from "@glimmer/component";

export default class SimpleComponent extends Component {
	<template>
    Hello World
  </template>

	get message() {
		return "test";
	}
}
