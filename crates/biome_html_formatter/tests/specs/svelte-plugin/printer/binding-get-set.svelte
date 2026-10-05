<input bind:value={() => value, (v) => (value = v)} />

<input bind:value={get, set} />

<input
    bind:value={binding_on_a_separate_line123, binding_on_a_separate_line123}
/>

<input
    bind:value={
        binding_value_on_a_separate_line123, binding_value_on_a_separate_line123
    }
/>

<input
    bind:value={
        () => getter_setter_each_need_own_line,
        (v) => (getter_setter_each_need_own_line = v)
    }
/>
