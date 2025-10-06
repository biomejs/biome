<script>
import {createApp} from 'vue';

createApp({
    props: [
        'ref',
        'key',
        'foo'
    ]
}).mount('#app');
</script>
