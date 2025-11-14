// should not generate diagnostics

// Composition pattern
import { action } from '@ember/object';
import Service from '@ember/service';

export default class MyComponent {
  @action
  handleClick() {}
}
