import Component from "@glimmer/component";

export const Header = <template>
  <header>
    <h1>My App</h1>
  </header>
</template>;

export const Footer = <template>
  <footer>
    <p>© 2024</p>
  </footer>
</template>;

export default class MyComponent extends Component {
	<template>
    <div>
      <Header />
      <main>Content here</main>
      <Footer />
    </div>
  </template>;
}
