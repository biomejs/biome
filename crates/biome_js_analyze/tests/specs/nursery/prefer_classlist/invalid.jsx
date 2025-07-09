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