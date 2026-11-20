/* should not generate diagnostics */
let a = `Hello, ${name}`;
let a = templateFunction`Hello, ${name}`;
let a = `Hello, name`;
let a = 'Hello, name';
let a = 'Hello, ' + name;
let a = `Hello, ${index + 1}`;
let a = `Hello, ${name + " foo"}`;
let a = `Hello, ${name || "foo"}`;
let a = `Hello, ${{foo: "bar"}.foo}`;
let a = '$2';
let a = '${';
let a = '$}';
let a = '{foo}';
let a = '{foo: "bar"}';

// GitHub Actions expressions (double curly braces)
let a = "${{ inputs.abc }}";
let a = '${{ github.event.action }}';
let a = "environment: ${{ inputs.environment }}";
let a = "${{ secrets.MY_SECRET }}";

// GitHub Actions expressions with nested braces
let a = "${{ fromJSON('{\"key\": \"value\"}') }}";
let a = "${{ toJSON(github) }}";
let a = "${{ format('{0} {1}', foo, bar) }}";
let a = "${{ contains(github.event.head_commit.message, '[skip ci]') }}";