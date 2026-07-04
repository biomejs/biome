<!-- should not generate diagnostics -->
<template>
  <!-- Custom components should not trigger the rule -->
  <Button role="button"></Button>
  <Nav role="navigation"></Nav>
  <Article role="article"></Article>
  <Dialog role="dialog"></Dialog>
  <Form role="form"></Form>
  <Table role="table"></Table>

  <!-- Kebab-case custom elements should not trigger the rule -->
  <my-button role="button"></my-button>
  <my-nav role="navigation"></my-nav>
  <my-article role="article"></my-article>

  <!-- Native elements with non-redundant roles -->
  <article role="presentation"></article>
  <article v-bind:role="roleValue"></article>
  <article :role="roleValue"></article>
  <!-- Static Vue bindings with non-redundant roles are valid -->
  <article :role="'presentation'"></article>
  <div :role="'button'"></div>
  <div v-bind:role="'navigation'"></div>
  <span></span>
  <div></div>
</template>
