// should not generate diagnostics
import { setupTest } from 'ember-qunit';
import { module, test } from 'qunit';

module('Example', function (hooks) {
  setupTest(hooks);

  test('it works', async function () {
    // No pauseTest call
  });
});
