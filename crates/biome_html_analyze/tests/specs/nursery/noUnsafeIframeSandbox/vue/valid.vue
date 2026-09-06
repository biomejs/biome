<!-- should not generate diagnostics -->
<a sandbox="allow-scripts allow-same-origin"></a>
<iframe sandbox=""></iframe>
<iframe sandbox="allow-downloads"></iframe>
<iframe sandbox="allow-scripts"></iframe>
<iframe sandbox="allow-same-origin"></iframe>
<iframe sandbox="allow-downloads allow-scripts"></iframe>
<iframe sandbox="allow-downloads allow-same-origin"></iframe>
<iframe sandbox="allow-downloads allow-scripts allow-forms"></iframe>
<iframe :sandbox="'allow-scripts'"></iframe>
<iframe :sandbox="'allow-downloads allow-same-origin'"></iframe>
<iframe v-bind:sandbox="'allow-downloads allow-scripts allow-forms'"></iframe>
<iframe :sandbox="dynamicSandbox"></iframe>
