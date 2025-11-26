// should not generate diagnostics
import Component from '@glimmer/component';

export default class MyComponent extends Component {
  <template>
    <input />
    <button>Click me</button>
    <CustomInput />
  </template>
}
