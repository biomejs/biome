import Component from '@glimmer/component';

export default class MyComponent extends Component {
  actions = {
    handleClick() {
      console.log('clicked');
    }
  }
}
