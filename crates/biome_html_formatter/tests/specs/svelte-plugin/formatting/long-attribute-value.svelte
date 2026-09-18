<input
  type="password"
  use:fits={{ store: passwordStore, clean: [ensureString] }}
  use:breaks={{ store: passwordStore, clean: [ensureString], validate: [minLength(6), maxLength(25)] }}
  on:fits={() => ''}
  on:breaks={() => { const a = ''; a.toLowerCase(); }}
  in:fits={{ store: passwordStore, clean: [ensureString] }}
  in:breaks={{ store: passwordStore, clean: [ensureString], validate: [minLength(6), maxLength(25)] }}
  out:fits={{ store: passwordStore, clean: [ensureString] }}
  out:breaks={{ store: passwordStore, clean: [ensureString], validate: [minLength(6), maxLength(25)] }}
  transition:fits={{ store: passwordStore, clean: [ensureString] }}
  transition:breaks={{ store: passwordStore, clean: [ensureString], validate: [minLength(6), maxLength(25)] }}
  animate:fits={{ store: passwordStore, clean: [ensureString] }}
  animate:breaks={{ store: passwordStore, clean: [ensureString], validate: [minLength(6), maxLength(25)] }}
  class:fits={'short check that fits in one line' ? true : false}
  class:breaks={'some really long condition check happening here' ? true : false}
   />
