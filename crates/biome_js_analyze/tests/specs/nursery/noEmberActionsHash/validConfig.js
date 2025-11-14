// should not generate diagnostics

// Non-Ember context - actions property is fine
const config = {
  actions: ['read', 'write', 'delete'],
  permissions: {
    admin: true
  }
};

export default config;
