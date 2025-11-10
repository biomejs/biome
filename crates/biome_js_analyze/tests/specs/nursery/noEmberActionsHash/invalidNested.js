import Route from '@ember/routing/route';

export default Route.extend({
  model() {
    return {};
  },

  actions: {
    save() {
      console.log('saving');
    },
    cancel() {
      console.log('canceling');
    }
  }
});
