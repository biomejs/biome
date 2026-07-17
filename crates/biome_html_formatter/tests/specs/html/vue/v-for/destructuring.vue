<div v-for="{  id,name  }    in    users">{{ id }}: {{ name }}</div>
<div v-for="[ first,second ]  in pairs">{{ first }}: {{ second }}</div>
<div v-for="({ id,name },index) in users">{{ index }}: {{ id }}: {{ name }}</div>
<div v-for="({ id, ...rest }, [ first, second ]) of entries">{{ id }}: {{ rest }}</div>
