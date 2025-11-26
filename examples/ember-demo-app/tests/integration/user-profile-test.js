// VIOLATION SHOWCASE: user-profile-test.js
// Demonstrates pauseTest violations

import { module, test } from 'qunit';
import { setupRenderingTest } from 'ember-qunit';
import { render, click } from '@ember/test-helpers';
import { pauseTest } from '@ember/test-helpers';  // ❌ VIOLATION: noEmberPauseTest
import { hbs } from 'ember-cli-htmlbars';

module('Integration | Component | user-profile', function(hooks) {
  setupRenderingTest(hooks);

  test('it renders user name', async function(assert) {
    this.set('user', { name: 'John Doe', email: 'john@example.com' });

    await render(hbs`<UserProfile @user={{this.user}} />`);

    // ❌ VIOLATION: noEmberPauseTest
    // pauseTest() should be removed before committing
    await pauseTest();

    assert.dom('h2').hasText('John Doe');
  });

  test('it handles edit button click', async function(assert) {
    await render(hbs`<UserProfile @user={{this.user}} />`);

    // ❌ VIOLATION: noEmberPauseTest
    // Using this.pauseTest() should also be removed
    this.pauseTest();

    await click('button');
    assert.dom('.edit-form').exists();
  });
});

/* ✅ FIXED VERSION:

import { module, test } from 'qunit';
import { setupRenderingTest } from 'ember-qunit';
import { render, click } from '@ember/test-helpers';
import { hbs } from 'ember-cli-htmlbars';

module('Integration | Component | user-profile', function(hooks) {
  setupRenderingTest(hooks);

  test('it renders user name', async function(assert) {
    this.set('user', { name: 'John Doe', email: 'john@example.com' });
    await render(hbs`<UserProfile @user={{this.user}} />`);

    assert.dom('h2').hasText('John Doe');
  });

  test('it handles edit button click', async function(assert) {
    await render(hbs`<UserProfile @user={{this.user}} />`);

    await click('button');
    assert.dom('.edit-form').exists();
  });
});
*/
