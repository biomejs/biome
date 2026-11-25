<!-- should generate diagnostics -->
<script setup lang="ts">
class Service {
    constructor(@inject(window) value: unknown, @inject(location) private readonly store: unknown) {}
    method(@inject(document.title) value = localStorage) {}
}
</script>
