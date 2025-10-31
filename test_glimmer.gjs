import Component from '@glimmer/component';

export default class MyComponent extends Component {
  <template>
    <div>
      <h1>{{@title}}</h1>
      <p>{{this.message}}</p>
      {{#if @showButton}}
        <button {{on "click" this.handleClick}}>Click me!</button>
      {{/if}}
    </div>
  </template>

  get message() {
    return 'Hello from Glimmer!';
  }

  handleClick = () => {
    console.log('Button clicked!');
  }
}
