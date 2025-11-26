// Modern Ember uses native JavaScript classes
// should not generate diagnostics
import Component from '@glimmer/component';
import { tracked } from '@glimmer/tracking';
import { action } from '@ember/object';

export default class MyComponent extends Component {
  @tracked foo = 'bar';

  @action
  handleClick() {
    this.foo = 'baz';
  }
}
