import Button from './Button';
import Card from './Card';
import UnusedComponent from './UnusedComponent';  // This should warn!

export default class MyComponent {
  <template>
    <Button>Click</Button>
    <Card>Content</Card>
  </template>
}
