// should not generate diagnostics
import Component from '@glimmer/component';

export default class MyComponent extends Component {
  <template>
    <button>Click me</button>
    <button tabindex="0">Focusable</button>
    <button tabindex="-1">Not in tab order</button>
    <div>Content without tabindex</div>
  </template>
}
