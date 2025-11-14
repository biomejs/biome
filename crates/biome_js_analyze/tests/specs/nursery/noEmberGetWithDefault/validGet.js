// should not generate diagnostics
import { get } from '@ember/object';

class MyComponent {
  someMethod() {
    return this.property ?? 'fallback';
  }
}
