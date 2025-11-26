// Test file to check what patterns we detect and which we miss

import Button from './Button';
import Card from './Card';
import formatDate from './formatDate';

export default class MyComponent {
  items = [1, 2, 3];
  condition = true;
  firstName = "John";
  lastName = "Doe";
  user = { name: "Alice" };
  date = new Date();

  handleClick(event) {
    console.log('clicked', event);
  }

  #privateCount = 0;

  unusedVariable = "should warn";

  <template>
    {{! 1. Component usage - currently detected ✅ }}
    <Button />
    <Card>Content</Card>

    {{! 2. Simple property - currently detected ✅ }}
    {{this.date}}

    {{! 3. Private field - currently detected ✅ }}
    {{this.#privateCount}}

    {{! 4. Block helper with variable - might NOT be detected ❌ }}
    {{#each this.items as |item|}}
      {{item}}
    {{/each}}

    {{! 5. Conditional - might NOT be detected ❌ }}
    {{#if this.condition}}
      Yes
    {{/if}}

    {{! 6. Helper with multiple args - partially detected? }}
    {{concat this.firstName this.lastName}}

    {{! 7. Nested property access - partially detected? }}
    {{this.user.name}}

    {{! 8. Modifier with method reference - currently detected ✅ }}
    <button {{on "click" this.handleClick}}>Click</button>

    {{! 9. Helper function call - currently detected ✅ }}
    {{formatDate this.date}}
  </template>
}
