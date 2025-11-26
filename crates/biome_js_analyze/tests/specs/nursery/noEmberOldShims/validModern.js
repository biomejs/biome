// should not generate diagnostics
import Component from '@glimmer/component';
import { dasherize } from '@ember/string';

export default class MyComponent extends Component {
  get processedName() {
    return dasherize(this.args.name);
  }
}
