import { setupTest } from 'ember-qunit';
import { module, test } from 'qunit';
import { pauseTest } from '@ember/test-helpers';

module('Example', function (hooks) {
  setupTest(hooks);

  test('it works', async function () {
    await pauseTest();
  });
});
