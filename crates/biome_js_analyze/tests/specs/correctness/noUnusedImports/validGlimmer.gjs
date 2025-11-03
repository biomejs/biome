import Component from '@glimmer/component';
import Button from './Button';
import Card from './Card';

export default class MyComponent extends Component {
  <template>
    <div>
      <Card>
        <Button>Click me</Button>
      </Card>
    </div>
  </template>
}
