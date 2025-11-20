/* should not generate diagnostics - components used in JS and template */
import Button from './Button';
import Card from './Card';
import { createComponent } from './utils';

const buttonInstance = createComponent(Button);

export default class MyComponent {
  <template>
    <Card>
      <Button>Click</Button>
    </Card>
  </template>
}
