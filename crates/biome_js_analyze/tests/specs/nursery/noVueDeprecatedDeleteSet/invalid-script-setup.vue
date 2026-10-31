<!-- should generate diagnostics -->
<script setup>
import * as vue from "vue";
import { set as s, del as d } from "vue";

s(obj, key, value);
d(obj, key);
vue.set(obj, key, value);
vue.del(obj, key);
</script>
