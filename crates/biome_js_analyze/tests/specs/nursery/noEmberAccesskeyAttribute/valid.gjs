// should not generate diagnostics
import Component from '@glimmer/component';

export default class MyButton extends Component {
  <template>
    <button>Save</button>
    <button aria-label="Save">S</button>
  </template>
}
