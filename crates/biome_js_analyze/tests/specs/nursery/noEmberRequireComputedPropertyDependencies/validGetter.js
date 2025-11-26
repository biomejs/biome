// should not generate diagnostics
import Component from '@glimmer/component';

export default class MyComponent extends Component {
  // Native getter syntax - no dependencies needed
  get fullName() {
    return `${this.firstName} ${this.lastName}`;
  }
}
