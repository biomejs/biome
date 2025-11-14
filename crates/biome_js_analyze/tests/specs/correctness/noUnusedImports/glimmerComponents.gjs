/* should not generate diagnostics - components used in template */
import Component from '@glimmer/component';
import Button from './Button';
import Card from './Card';
import { Header, Footer } from './Layout';

export default class MyComponent extends Component {
  <template>
    <Card>
      <Header />
      <Button>Click me</Button>
      <Footer />
    </Card>
  </template>
}
