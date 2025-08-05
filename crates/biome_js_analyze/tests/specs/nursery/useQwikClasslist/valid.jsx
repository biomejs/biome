// should not generate diagnostics
import { component$ } from "@builder.io/qwik";
import styles from "./MyComponent.module.css";

export default component$((props) => {
  const arrayClass = [
    styles.container,
    "p-8",
    props.isHighAttention ? "text-green-500" : "text-slate-500",
    { active: true },
  ];

  const objectClass = {
    "text-green-500": props.isHighAttention,
    "p-4": true,
  };

  return (
    <>
      <div class={arrayClass}>Hello world!</div>
      <div class={objectClass}>Hello world!</div>
    </>
  );
});

// should not generate diagnostics: spread props with valid class usage
export const SpreadPropsValid = (props) => (
  <div {...props} class={{ foo: true, bar: props.active }} />
);

// should not generate diagnostics: class as string with spread props
export const SpreadPropsStringValid = (props) => (
  <div {...props} class={"foo bar"} />
); 