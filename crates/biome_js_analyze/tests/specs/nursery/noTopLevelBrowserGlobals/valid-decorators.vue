<!-- should not generate diagnostics -->
<script setup lang="ts">
class Service {
    constructor(factory = @register(window) class {}) {}
    method(factory = class { method(@inject(document) value: unknown) {} }) {}
}
</script>
