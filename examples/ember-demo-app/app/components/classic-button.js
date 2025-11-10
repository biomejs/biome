// VIOLATION SHOWCASE: classic-button.js
// Demonstrates classic Ember class violation

import Component from '@ember/component';
import { computed } from '@ember/object';

// ❌ VIOLATION: noEmberClassicClasses
// Using .extend() is the old "classic" Ember pattern
export default Component.extend({
  tagName: 'button',
  classNames: ['classic-button'],

  // ❌ VIOLATION: noEmberRequireComputedPropertyDependencies
  // computed() without dependency keys
  label: computed(function() {
    return this.text || 'Click me';
  }),

  click() {
    if (this.onClick) {
      this.onClick();
    }
  }
});

/* ✅ FIXED VERSION:

import Component from '@glimmer/component';

export default class ClassicButton extends Component {
  // Use native getter instead of computed
  get label() {
    return this.args.text || 'Click me';
  }

  handleClick = () => {
    if (this.args.onClick) {
      this.args.onClick();
    }
  }

  <template>
    <button
      class="classic-button"
      {{on "click" this.handleClick}}
    >
      {{this.label}}
    </button>
  </template>
}
*/
