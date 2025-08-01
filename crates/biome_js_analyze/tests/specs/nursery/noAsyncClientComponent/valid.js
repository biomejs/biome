/* should not generate diagnostics */

export default async function MyComponent() {
  return <div>Hello</div>;
}

export default async function myFunction() {
  return 'not a component';
}

export default function MyComponent() {
  return <div>Hello</div>;
}
