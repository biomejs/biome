/* should not generate diagnostics */
<>
    {/* default "render" prop still works */}
    <Button render={<a href="/home" aria-label="Home" />}>Home</Button>
    {/* additional prop names from options */}
    <Button as={<a href="/about" aria-label="About" />}>About</Button>
    <Button component={<a href="/docs" aria-label="Docs" />}>Docs</Button>
</>
