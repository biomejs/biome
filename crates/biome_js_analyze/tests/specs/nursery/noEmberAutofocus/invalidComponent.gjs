// autofocus attribute on custom component
import Component from '@glimmer/component';

export default class MyComponent extends Component {
  <template>
    <CustomInput autofocus={{true}} />
  </template>
}
