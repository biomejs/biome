import Service from '@ember/service';

export default Service.extend({
  init() {
    this._super(...arguments);
    this.setupService();
  }
});
