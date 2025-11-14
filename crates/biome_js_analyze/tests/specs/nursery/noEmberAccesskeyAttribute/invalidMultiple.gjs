// Multiple accesskey attributes in template
import Component from '@glimmer/component';

export default class MyForm extends Component {
  <template>
    <div>
      <button accesskey="s">Save</button>
      <input type="text" accesskey="n" />
      <a href="/help" accesskey="h">Help</a>
    </div>
  </template>
}
