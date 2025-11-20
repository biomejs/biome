// should not generate diagnostics
// Custom get() function, not from @ember/object
const customObject = {
  get(key) {
    return this[key];
  }
};

const value = customObject.get('key');
