import { get } from '@ember/object';

export default class MyComponent {
  someMethod() {
    const obj = this.getObject();
    return get(obj, 'property');
  }
}
