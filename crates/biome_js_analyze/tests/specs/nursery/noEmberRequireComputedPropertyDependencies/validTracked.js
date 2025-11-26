// should not generate diagnostics
import Component from '@glimmer/component';
import { tracked } from '@glimmer/tracking';

export default class MyComponent extends Component {
  @tracked firstName;
  @tracked lastName;

  // Native getter with tracked properties
  get fullName() {
    return `${this.firstName} ${this.lastName}`;
  }
}
