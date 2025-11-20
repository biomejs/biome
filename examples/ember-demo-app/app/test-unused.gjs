import Component from '@glimmer/component';
import UnusedComponent from './unused-component';
import UsedComponent from './used-component';

export default class MyComponent extends Component {
  <template>
    <UsedComponent />
  </template>
}
