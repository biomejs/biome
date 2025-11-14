// VIOLATION SHOWCASE: index.js (route)
// Demonstrates old shims and classic classes

import Route from '@ember/routing/route';
import DS from 'ember-data';  // ❌ VIOLATION: noEmberOldShims

// ❌ VIOLATION: noEmberClassicClasses
// Using .extend() instead of native class
export default Route.extend({
  model() {
    // ❌ VIOLATION: Using old ember-data API
    return DS.PromiseArray.create({
      promise: this.store.findAll('post')
    });
  },

  setupController(controller, model) {
    this._super(controller, model);
    controller.set('isLoading', false);
  }
});

/* ✅ FIXED VERSION:

import Route from '@ember/routing/route';
import { inject as service } from '@ember/service';

export default class IndexRoute extends Route {
  @service store;

  async model() {
    // Use modern async/await
    return await this.store.findAll('post');
  }

  setupController(controller, model) {
    super.setupController(controller, model);
    controller.isLoading = false;
  }
}
*/
