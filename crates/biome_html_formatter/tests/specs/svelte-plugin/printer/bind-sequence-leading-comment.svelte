<Foo
    bind:value={
        // eslint-disable-next-line @typescript-eslint/no-unsafe-return -- ignore
        () => bar ?? "",
        (v) => {
            bar = v;
        }
    }
/>

<Foo
    bind:value={
        /* interesting
        comment */
        () => bar ?? "",
        (v) => {
            bar = v;
        }
    }
/>
