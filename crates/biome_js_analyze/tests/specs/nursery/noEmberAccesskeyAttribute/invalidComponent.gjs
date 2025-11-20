// Component with accesskey argument
import Component from '@glimmer/component';
import Button from './button';

export default class MyForm extends Component {
  <template>
    <Button accesskey="s" />
  </template>
}
