/* should not generate diagnostics - self-closing components */
import Icon from './Icon';
import Divider from './Divider';
import Image from './Image';

export default class MyComponent {
  <template>
    <div>
      <Icon />
      <Divider />
      <Image />
    </div>
  </template>
}
