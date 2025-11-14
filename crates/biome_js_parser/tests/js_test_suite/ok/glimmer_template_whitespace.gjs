// Template with whitespace after 'template' keyword
// This tests the edge case where there's whitespace before the '>'
const WithSpace = <template >Hello World</template>;

// Template with tab
const WithTab = <template	>Hello Tab</template>;

// Template with newline
const WithNewline = <template
>Hello Newline</template>;

// Multiple spaces
const MultipleSpaces = <template    >Hello Spaces</template>;
