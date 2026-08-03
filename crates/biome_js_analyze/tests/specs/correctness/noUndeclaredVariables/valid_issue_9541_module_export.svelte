<!-- should not generate diagnostics -->
<script lang="ts" module>
  export const someFunction = () => {
    console.log("hai!");
  };

  const someOtherFunction = () => {
    console.log("heyy");
  };
</script>

<script lang="ts">
  someFunction();
  someOtherFunction();
</script>
