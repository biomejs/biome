"use client";

export default async function MyComponent() {
  return <div>Hello</div>;
}

// Another case
"use client";

async function AnotherComponent() {
  return <span>World</span>;
}
export default AnotherComponent;