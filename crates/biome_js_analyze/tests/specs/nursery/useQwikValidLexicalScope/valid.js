// should not generate diagnostics
import { component$, useTask$, $ } from '@builder.io/qwik';

export const HelloWorld = component$(() => {
  const print = $((msg) => {
    console.log(msg);
  });

  const handleClick = $((event) => {
    console.log('clicked', event);
  });

  const processData = $((data) => {
    return data.map(item => item.name);
  });

  useTask$(() => {
    print("Hello World");
  });

  return <h1>Hello</h1>;
}); 