{#snippet parts()}{#if leftIcon}<span class="left-icon">{@render leftIcon()}</span>{/if}{@render children?.()}{#if rightIcon}<span class="right-icon">{@render rightIcon()}</span>{/if}{/snippet}

<p><span>{format("communication.contacts.info")}</span>{#if showAddons}<span>{format("communication.contacts.addons")}</span>{/if}</p>

<label><span class="addon">{format("dashboards.sensorMaps.lat")}</span><BoundInput value={latitude} onchange={setLatitude} /></label>

<span class="back"><Icon name="BiChevronLeft" size="24px" /><span class="dashboard-name">{dashboardName}</span></span>
