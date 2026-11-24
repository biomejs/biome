<!-- should generate diagnostics -->
<script module lang="ts">
  declare module API { export interface Result {} }
  declare module "ExternalLib" { export interface Result {} }
</script>
<script lang="ts">
  type Result = API.Result;
  type Missing = ExternalLib.Result;
</script>
