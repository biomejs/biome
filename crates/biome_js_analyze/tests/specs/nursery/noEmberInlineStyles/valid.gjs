// should not generate diagnostics
import Component from '@glimmer/component';

export default class MyComponent extends Component {
  <template>
    <div class="my-styled-div">Text</div>
    <button class="btn-primary">Click</button>
    <MyComponent @class="styled-component" />
  </template>
}
