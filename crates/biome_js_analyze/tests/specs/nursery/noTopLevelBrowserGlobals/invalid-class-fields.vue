<!-- should generate diagnostics -->
<script setup>
class Client {
    [window.name] = 1;
    static width = window.innerWidth;
    static { document.title; }
}
class ClientElement extends HTMLElement {}
</script>
