// VIOLATION SHOWCASE: user-profile.gjs
// This file demonstrates 4 template-related rule violations

import Component from '@glimmer/component';
import { tracked } from '@glimmer/tracking';

export default class UserProfile extends Component {
  @tracked isEditing = false;

  <template>
    <div class="user-profile">
      <h2>{{@user.name}}</h2>

      {{! ❌ VIOLATION: noEmberAccesskeyAttribute
          Keyboard shortcuts using accesskey are inconsistent across browsers }}
      <button accesskey="e" type="button">
        Edit Profile
      </button>

      {{! ❌ VIOLATION: noEmberAutofocus
          Autofocus can cause accessibility issues for screen readers }}
      <input
        type="text"
        value={{@user.email}}
        autofocus
        placeholder="Email"
      />

      {{! ❌ VIOLATION: noEmberInlineStyles
          Use CSS classes instead of inline styles }}
      <div style="display: flex; gap: 10px; margin-top: 20px;">
        <button>Save</button>
        <button>Cancel</button>
      </div>

      {{! ❌ VIOLATION: noEmberPositiveTabindex
          Positive tabindex disrupts natural tab order }}
      <a href="/profile/settings" tabindex="1">
        Settings
      </a>
      <a href="/profile/privacy" tabindex="2">
        Privacy
      </a>
    </div>

    {{! ✅ FIXED VERSION (commented out):
    <div class="user-profile">
      <h2>{{@user.name}}</h2>

      <button type="button">
        Edit Profile
      </button>

      <input
        type="text"
        value={{@user.email}}
        placeholder="Email"
      />

      <div class="action-buttons">
        <button>Save</button>
        <button>Cancel</button>
      </div>

      <a href="/profile/settings">Settings</a>
      <a href="/profile/privacy">Privacy</a>
    </div>
    }}
  </template>
}
