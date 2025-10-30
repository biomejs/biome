import Component from '@glimmer/component';

export default class MyComponent extends Component {
  doSomething() {
    var x = 1;  // var should trigger warning
    console.log(x);
  }
  
  <template>
    <div>Test</div>
  </template>
}
