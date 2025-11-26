// Template in TypeScript file
const MyTemplate: TemplateFactory = <template>
  <div>Hello TypeScript</div>
</template>;

// Class with template in TypeScript
class TypedComponent {
  private value: number = 42;

  <template>
    <span>{{this.value}}</span>
  </template>
}
