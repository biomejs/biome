// should not generate diagnostics

try { /* ... */ } catch { }


try { /* ... */ } catch (used: any) { console.error(used); }

try { /* ... */ } catch ({ used }: any) { console.error(used); }

try { /* ... */ } catch ({ used, unused }: any) { console.error(used); }


try { /* ... */ } catch (used: any) { const log = () => console.error(used); log(); }

try { /* ... */ } catch ({ used }: any) { const log = () => console.error(used); log(); }

try { /* ... */ } catch ({ used, unused }: any) { const log = () => console.error(used); log(); }


try { /* ... */ } catch ({ used: alias }: any) { console.error(alias); }

try { /* ... */ } catch ({ nested: { used } }: any) { console.error(used); }

try { /* ... */ } catch ({ ...rest }: any) { console.error(rest); }
