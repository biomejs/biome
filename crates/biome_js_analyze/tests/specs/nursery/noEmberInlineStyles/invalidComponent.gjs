// Inline style on custom component
import Component from '@glimmer/component';
import MyComponent from './MyComponent';

export default class MyWrapper extends Component {
  <template>
    <MyComponent style="margin: 10px;" />
  </template>
}
