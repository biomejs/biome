// should not generate diagnostics

import Component from '@glimmer/component';
import { action } from '@ember/object';

export default class MyComponent extends Component {
  @action
  handleClick() {
    console.log('clicked');
  }

  @action
  handleSubmit() {
    console.log('submitted');
  }
}
