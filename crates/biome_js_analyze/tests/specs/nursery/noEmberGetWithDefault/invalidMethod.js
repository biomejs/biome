import Component from '@glimmer/component';

class MyComponent extends Component {
  someMethod() {
    return this.getWithDefault('property', 'fallback');
  }
}
