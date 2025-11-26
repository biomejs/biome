// VIOLATION SHOWCASE: actions-demo.js
// Demonstrates actions hash violation

import Component from '@glimmer/component';
import { tracked } from '@glimmer/tracking';

export default class ActionsDemo extends Component {
  @tracked count = 0;

  // ❌ VIOLATION: noEmberActionsHash
  // The actions hash is deprecated in Octane
  actions = {
    increment() {
      this.count++;
    },

    decrement() {
      this.count--;
    },

    reset() {
      this.count = 0;
    }
  }
}

/* ✅ FIXED VERSION:

import Component from '@glimmer/component';
import { tracked } from '@glimmer/tracking';
import { action } from '@ember/object';

export default class ActionsDemo extends Component {
  @tracked count = 0;

  @action
  increment() {
    this.count++;
  }

  @action
  decrement() {
    this.count--;
  }

  @action
  reset() {
    this.count = 0;
  }
}
*/
