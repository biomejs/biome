// VIOLATION SHOWCASE: analytics.js
// Demonstrates get() and getWithDefault() violations

import Service from '@ember/service';
import { get } from '@ember/object';  // ❌ VIOLATION: noEmberGet
import { getWithDefault } from '@ember/object';  // ❌ VIOLATION: noEmberGetWithDefault

export default class AnalyticsService extends Service {
  trackEvent(eventName, properties) {
    // ❌ VIOLATION: noEmberGet
    // get() is deprecated - use native property access
    const userId = get(this, 'currentUser.id');
    const userName = get(properties, 'user.name');

    // ❌ VIOLATION: noEmberGetWithDefault
    // getWithDefault() is deprecated - use optional chaining with nullish coalescing
    const timestamp = getWithDefault(properties, 'timestamp', Date.now());
    const source = getWithDefault(this, 'source', 'web');

    console.log('Tracking:', eventName, {
      userId,
      userName,
      timestamp,
      source
    });
  }

  trackPageView(route) {
    // ❌ More violations
    const routeName = get(route, 'routeName');
    const params = getWithDefault(route, 'params', {});

    this.trackEvent('pageview', { route: routeName, params });
  }
}

/* ✅ FIXED VERSION:

import Service from '@ember/service';

export default class AnalyticsService extends Service {
  trackEvent(eventName, properties) {
    // Use native property access
    const userId = this.currentUser?.id;
    const userName = properties?.user?.name;

    // Use nullish coalescing operator
    const timestamp = properties?.timestamp ?? Date.now();
    const source = this.source ?? 'web';

    console.log('Tracking:', eventName, {
      userId,
      userName,
      timestamp,
      source
    });
  }

  trackPageView(route) {
    const routeName = route.routeName;
    const params = route.params ?? {};

    this.trackEvent('pageview', { route: routeName, params });
  }
}
*/
