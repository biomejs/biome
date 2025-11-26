// should not generate diagnostics
import Component from '@glimmer/component';

export default class MyComponent extends Component {
  someMethod() {
    // No jQuery usage at all
    console.log('Hello');
  }
}
