<!-- should not generate diagnostics -->
<script lang="ts" module>
  export const someFunction = () => {
    console.log("hai!");
  };

  export function someExportedFunctionDeclaration() {
    console.log("yo!");
  }

  const someOtherFunction = () => {
    console.log("heyy");
  };
</script>

<script lang="ts">
  someFunction();
  someExportedFunctionDeclaration();
  someOtherFunction();
</script>
