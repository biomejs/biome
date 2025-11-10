// VIOLATION SHOWCASE: validatable.js
// This mixin itself isn't a violation, but importing from /mixins/ is

import Mixin from '@ember/object/mixin';

export default Mixin.create({
  validate() {
    const errors = [];

    if (!this.name) {
      errors.push('Name is required');
    }

    if (!this.email) {
      errors.push('Email is required');
    }

    return errors;
  },

  isValid() {
    return this.validate().length === 0;
  }
});

// When this is imported like:
// import Validatable from 'app/mixins/validatable';
// ❌ VIOLATION: noEmberMixins
// It triggers the rule because of the /mixins/ path

/* ✅ FIXED VERSION (as utility):

// app/utils/validation.js
export function validate(obj) {
  const errors = [];

  if (!obj.name) {
    errors.push('Name is required');
  }

  if (!obj.email) {
    errors.push('Email is required');
  }

  return errors;
}

export function isValid(obj) {
  return validate(obj).length === 0;
}

// Usage:
import { validate, isValid } from 'app/utils/validation';

export default class MyComponent extends Component {
  validate() {
    return validate(this);
  }

  get isValid() {
    return isValid(this);
  }
}
*/
