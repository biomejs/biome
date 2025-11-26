// VIOLATION SHOWCASE: application.js (controller)
// Demonstrates importing from mixins

import Controller from '@ember/controller';
import Validatable from 'app/mixins/validatable';  // ❌ VIOLATION: noEmberMixins

// ❌ VIOLATION: noEmberClassicClasses
export default Controller.extend(Validatable, {
  name: '',
  email: '',

  // ❌ VIOLATION: noEmberActionsHash
  actions: {
    save() {
      if (this.isValid()) {
        console.log('Saving:', this.name, this.email);
      } else {
        console.log('Validation errors:', this.validate());
      }
    }
  }
});

/* ✅ FIXED VERSION:

import Controller from '@ember/controller';
import { tracked } from '@glimmer/tracking';
import { action } from '@ember/object';
import { validate, isValid } from 'app/utils/validation';

export default class ApplicationController extends Controller {
  @tracked name = '';
  @tracked email = '';

  validate() {
    return validate(this);
  }

  get isValid() {
    return isValid(this);
  }

  @action
  save() {
    if (this.isValid) {
      console.log('Saving:', this.name, this.email);
    } else {
      console.log('Validation errors:', this.validate());
    }
  }
}
*/
