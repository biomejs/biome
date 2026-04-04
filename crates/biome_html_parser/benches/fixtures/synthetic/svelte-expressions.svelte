<script>
  let firstName = 'John';
  let lastName = 'Doe';
  let age = 30;
  let items = [1, 2, 3, 4, 5];
  let user = { profile: { name: 'Alice', settings: { theme: 'dark' } } };
  let htmlContent = '<strong>Bold text</strong>';
  let condition = true;
  let count = 0;
  let price = 19.99;
  let currency = 'USD';
  let formattedDate = new Date().toLocaleDateString();
  let items2 = [{ name: 'A', value: 1 }, { name: 'B', value: 2 }];
</script>

<div class="expressions-container">
  <h1>Welcome, {firstName} {lastName}!</h1>
  <p>Age: {age} years old</p>
  <p>Full name length: {(firstName + ' ' + lastName).length}</p>
  <p>Uppercase: {firstName.toUpperCase()} {lastName.toUpperCase()}</p>
  <p>Is adult: {age >= 18 ? 'Yes' : 'No'}</p>
  <p>Next year: {age + 1}</p>
  <p>Decades lived: {Math.floor(age / 10)}</p>
  
  <div>
    <span>Count: {count}</span>
    <span>Double: {count * 2}</span>
    <span>Triple: {count * 3}</span>
    <span>Square: {count ** 2}</span>
    <span>Is even: {count % 2 === 0}</span>
  </div>

  <p>Price: {price.toFixed(2)} {currency}</p>
  <p>Discounted: {(price * 0.9).toFixed(2)}</p>
  <p>Tax included: {(price * 1.2).toFixed(2)}</p>
  <p>Formatted: {new Intl.NumberFormat('en-US', { style: 'currency', currency }).format(price)}</p>

  <p>Date: {formattedDate}</p>
  <p>Timestamp: {Date.now()}</p>
  <p>Random: {Math.random().toFixed(4)}</p>
  <p>PI: {Math.PI.toFixed(5)}</p>

  <ul>
    {#each items as item, i}
      <li>
        Index: {i}, Value: {item}, 
        Square: {item ** 2}, 
        Cube: {item ** 3},
        Is even: {item % 2 === 0 ? 'yes' : 'no'},
        Double: {item * 2}
      </li>
    {/each}
  </ul>

  <p>Nested: {user.profile.name} - Theme: {user.profile.settings.theme}</p>
  <p>Computed: {items.reduce((a, b) => a + b, 0)}</p>
  <p>Average: {(items.reduce((a, b) => a + b, 0) / items.length).toFixed(2)}</p>
  <p>Max: {Math.max(...items)}</p>
  <p>Min: {Math.min(...items)}</p>

  {@html htmlContent}
  {@html '<div class="injected">' + htmlContent + '</div>'}
  {@html condition ? '<span>True</span>' : '<span>False</span>'}
  {@html items.map(i => `<span>${i}</span>`).join('')}

  {@const fullName = firstName + ' ' + lastName}
  {@const initials = firstName[0] + lastName[0]}
  {@const reversed = firstName.split('').reverse().join('')}
  {@const doubled = items.map(x => x * 2)}
  {@const filtered = items.filter(x => x > 2)}
  {@const sum = items.reduce((a, b) => a + b, 0)}
  {@const avg = sum / items.length}
  {@const greeting = `Hello, ${fullName}!`}
  {@const isValid = firstName.length > 0 && lastName.length > 0}
  {@const displayName = fullName || 'Anonymous'}
  {@const status = age >= 18 ? 'adult' : 'minor'}
  {@const category = age < 13 ? 'child' : age < 20 ? 'teen' : 'adult'}

  <p>Full name: {fullName}</p>
  <p>Initials: {initials}</p>
  <p>Reversed: {reversed}</p>
  <p>Doubled: {doubled.join(', ')}</p>
  <p>Filtered: {filtered.join(', ')}</p>
  <p>Sum: {sum}</p>
  <p>Average: {avg.toFixed(2)}</p>
  <p>Greeting: {greeting}</p>
  <p>Valid: {isValid}</p>
  <p>Display: {displayName}</p>
  <p>Status: {status}</p>
  <p>Category: {category}</p>

  {@debug firstName, lastName, age, items}
  {@debug user, condition, count}
  {@debug items2}

  <p>Boolean: {true} {false}</p>
  <p>Null/Undefined: {null} {undefined}</p>
  <p>Array length: {items.length}</p>
  <p>String methods: {'hello'.toUpperCase()} {'WORLD'.toLowerCase()}</p>
  <p>Template: {`Count: ${count}`}</p>
  <p>Expression: {(2 + 3) * 4}</p>
  <p>Comparison: {5 > 3} {5 < 3} {5 === 5}</p>
  <p>Logical: {true && 'yes'} {false || 'no'}</p>
  <p>Nullish: {null ?? 'default'} {undefined ?? 'fallback'}</p>
  <p>Optional: {user?.profile?.name}</p>
</div>
