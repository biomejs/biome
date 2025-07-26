import { component$ } from "@builder.io/qwik";
import classnames from "classnames";
import styles from "./MyComponent.module.css";

export default component$((props) => {
  return (
    <div class={classnames(
      styles.container,
      "p-8",
      {
        "text-green-500": props.isHighAttention,
        "text-slate-500": !props.isHighAttention,
      },
      { active: true }
    )}>
      Hello world!
    </div>
  );
});

// Invalid: classnames with spread props
export const SpreadPropsInvalid = (props) => (
  <div {...props} class={classnames('foo', { bar: true })} />
);

// Invalid: classnames in nested call
export const NestedClassnamesInvalid = () => (
  <div class={classnames(classnames('foo', { bar: true }))} />
); 