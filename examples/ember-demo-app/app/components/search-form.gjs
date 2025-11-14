// VIOLATION SHOWCASE: search-form.gjs
// Demonstrates positive tabindex violation

import Component from '@glimmer/component';
import { tracked } from '@glimmer/tracking';

export default class SearchForm extends Component {
  @tracked query = '';

  <template>
    <form>
      {{! ❌ VIOLATION: noEmberPositiveTabindex
          Setting explicit tab order with positive values is bad for accessibility }}
      <input
        type="search"
        value={{this.query}}
        placeholder="Search..."
        tabindex="3"
      />
      <button type="submit" tabindex="1">Search</button>
      <button type="reset" tabindex="2">Clear</button>
    </form>

    {{! ✅ FIXED VERSION:
    <form>
      <input
        type="search"
        value={{this.query}}
        placeholder="Search..."
      />
      <button type="submit">Search</button>
      <button type="reset">Clear</button>
    </form>
    }}
  </template>
}
