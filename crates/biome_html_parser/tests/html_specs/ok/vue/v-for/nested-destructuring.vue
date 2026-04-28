<div v-for="{ id, profile: { username, avatar } } in members">{{ id }}: {{ username }}: {{ avatar }}</div>
<div v-for="[foo, [fizz, buzz]] in items">{{ foo }}: {{ fizz }}: {{ buzz }}</div>
<div v-for="{ id, tags: [primary, secondary], profile: { avatar, links: [home, work] } } in members">{{ id }}: {{ primary }}: {{ avatar }}: {{ home }}</div>
<div v-for="[{ id, name }, [firstRole, secondRole]] in entries">{{ id }}: {{ name }}: {{ firstRole }}: {{ secondRole }}</div>
<div v-for="([first, { id, profile: { username } }], index) in groupedMembers">{{ index }}: {{ first }}: {{ id }}: {{ username }}</div>
<div v-for="{ matrix: [[topLeft, topRight], [bottomLeft, bottomRight]], meta: { tags: [primaryTag] } } in dashboards">{{ topLeft }}: {{ bottomRight }}: {{ primaryTag }}</div>
