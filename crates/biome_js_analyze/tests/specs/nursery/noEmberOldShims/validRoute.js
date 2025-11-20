// should not generate diagnostics
import Route from '@ember/routing/route';

export default class MyRoute extends Route {
  model() {
    return this.store.findAll('post');
  }
}
