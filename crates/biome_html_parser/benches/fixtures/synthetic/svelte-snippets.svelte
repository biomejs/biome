<script>
  let items = [
    { id: 1, title: 'First', desc: 'Description 1' },
    { id: 2, title: 'Second', desc: 'Description 2' },
    { id: 3, title: 'Third', desc: 'Description 3' },
  ];
  let users = [
    { name: 'Alice', role: 'admin', active: true },
    { name: 'Bob', role: 'user', active: false },
    { name: 'Carol', role: 'editor', active: true },
  ];
  let products = [
    { name: 'Widget', price: 9.99, inStock: true },
    { name: 'Gadget', price: 19.99, inStock: false },
  ];
</script>

{#snippet header(title, subtitle)}
  <header class="page-header">
    <h1>{title}</h1>
    <p class="subtitle">{subtitle}</p>
  </header>
{/snippet}

{#snippet footer(copyright, year)}
  <footer class="page-footer">
    <p>&copy; {year} {copyright}</p>
    <nav>
      <a href="/privacy">Privacy</a>
      <a href="/terms">Terms</a>
    </nav>
  </footer>
{/snippet}

{#snippet card(title)}
  <article class="card">
    <header class="card-header">
      <h2>{title}</h2>
    </header>
    <div class="card-content">
      <slot />
    </div>
  </article>
{/snippet}

{#snippet button(label, variant = 'primary')}
  <button
    class="btn btn-{variant}"
    type="button"
  >
    {label}
  </button>
{/snippet}

{#snippet icon(name, size = 24)}
  <svg width={size} height={size} class="icon icon-{name}">
    <use href="/icons.svg#{name}" />
  </svg>
{/snippet}

{#snippet badge(text, type = 'info')}
  <span class="badge badge-{type}">
    {text}
  </span>
{/snippet}

{#snippet avatar(name, src, size = 'medium')}
  <div class="avatar avatar-{size}">
    {#if src}
      <img {src} alt={name} />
    {:else}
      <span class="initials">{name.charAt(0)}</span>
    {/if}
  </div>
{/snippet}

{#snippet listItem(item, index)}
  <li class="list-item" data-index={index}>
    <span class="item-number">{index + 1}</span>
    <span class="item-title">{item.title}</span>
    <span class="item-desc">{item.desc}</span>
  </li>
{/snippet}

{#snippet userRow(user)}
  <tr class:user-active={user.active}>
    <td>{user.name}</td>
    <td>{user.role}</td>
    <td>
      {#if user.active}
        {@render badge('Active', 'success')}
      {:else}
        {@render badge('Inactive', 'warning')}
      {/if}
    </td>
  </tr>
{/snippet}

{#snippet productCard(product)}
  <div class="product-card" class:out-of-stock={!product.inStock}>
    <h3>{product.name}</h3>
    <p class="price">${product.price.toFixed(2)}</p>
    {#if product.inStock}
      {@render badge('In Stock', 'success')}
    {:else}
      {@render badge('Out of Stock', 'danger')}
    {/if}
  </div>
{/snippet}

{#snippet modal(title)}
  <div class="modal-overlay">
    <div class="modal">
      <header>
        <h2>{title}</h2>
        <button aria-label="Close">
          {@render icon('close', 20)}
        </button>
      </header>
      <div class="modal-body">
        <slot />
      </div>
    </div>
  </div>
{/snippet}

{#snippet tooltip(content)}
  <span class="tooltip">
    {@render content()}
  </span>
{/snippet}

{#snippet skeleton(width = '100%', height = '20px')}
  <div class="skeleton" style:width style:height></div>
{/snippet}

{#snippet emptyState(message, icon = 'inbox')}
  <div class="empty-state">
    {@render icon(icon, 48)}
    <p>{message}</p>
  </div>
{/snippet}

{#snippet pagination(current, total)}
  <nav class="pagination">
    <button disabled={current === 1}>
      Previous
    </button>
    <span>Page {current} of {total}</span>
    <button disabled={current === total}>
      Next
    </button>
  </nav>
{/snippet}

{#snippet breadcrumb(items)}
  <nav class="breadcrumb">
    {#each items as item, i}
      {#if i > 0}
        <span class="separator">/</span>
      {/if}
      {#if item.href}
        <a href={item.href}>{item.label}</a>
      {:else}
        <span>{item.label}</span>
      {/if}
    {/each}
  </nav>
{/snippet}

{#snippet tabs(tabsList, activeTab)}
  <div class="tabs">
    {#each tabsList as tab}
      <button
        class="tab"
        class:active={tab.id === activeTab}
      >
        {tab.label}
      </button>
    {/each}
  </div>
{/snippet}

{#snippet alert(message, type = 'info', dismissible = false)}
  <div class="alert alert-{type}" role="alert">
    {@render icon(type, 20)}
    <span>{message}</span>
    {#if dismissible}
      <button aria-label="Dismiss">
        {@render icon('close', 16)}
      </button>
    {/if}
  </div>
{/snippet}

{#snippet menuItem(label, href, iconName, active = false)}
  <a {href} class="menu-item" class:active>
    {@render icon(iconName, 20)}
    <span>{label}</span>
  </a>
{/snippet}

{#snippet statCard(label, value, change, prefix = '')}
  <div class="stat-card">
    <span class="stat-label">{label}</span>
    <span class="stat-value">{prefix}{value}</span>
    {#if change}
      <span class="stat-change" class:positive={change > 0} class:negative={change < 0}>
        {change > 0 ? '+' : ''}{change}%
      </span>
    {/if}
  </div>
{/snippet}

{@render header('Dashboard', 'Welcome back!')}

<main>
  {@render card('Recent Items')}

  <ul>
    {#each items as item, i}
      {@render listItem(item, i)}
    {/each}
  </ul>

  <table>
    <thead>
      <tr>
        <th>Name</th>
        <th>Role</th>
        <th>Status</th>
      </tr>
    </thead>
    <tbody>
      {#each users as user}
        {@render userRow(user)}
      {/each}
    </tbody>
  </table>

  <div class="products">
    {#each products as product}
      {@render productCard(product)}
    {/each}
  </div>

  {@render alert('Operation completed successfully', 'success', true)}
  {@render alert('Please check your input', 'warning')}
  {@render alert('An error occurred', 'error', true)}

  {@render statCard('Revenue', '12,345', 15.3, '$')}
  {@render statCard('Users', '1,234', -2.5)}
  {@render statCard('Orders', '567', 8.1)}

  {@render pagination(3, 10)}

  {@render menuItem('Home', '/', 'home', true)}
  {@render menuItem('Settings', '/settings', 'settings', false)}
  {@render menuItem('Profile', '/profile', 'user', false)}

  {@render emptyState('No items found', 'inbox')}
  {@render emptyState('No notifications', 'bell')}

  {@render skeleton('100%', '40px')}
  {@render skeleton('80%', '20px')}
  {@render skeleton('60%', '20px')}
</main>

{@render footer('My Company', 2024)}
