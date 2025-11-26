import { get } from '@ember/object';
import Component from '@glimmer/component';

export default class MyComponent extends Component {
  someMethod() {
    return get(this, 'anotherProperty');
  }
}
