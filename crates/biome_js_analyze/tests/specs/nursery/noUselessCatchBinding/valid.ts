// should not generate diagnostics

try { /* ... */ } catch (used: any) { console.error(used); }

try { /* ... */ } catch ({ used }: any) { console.error(used); }

try { /* ... */ } catch ({ used, unused }: any) { console.error(used); }


try { /* ... */ } catch (used: any) { const log = () => console.error(used); log(); }

try { /* ... */ } catch ({ used }: any) { const log = () => console.error(used); log(); }

try { /* ... */ } catch ({ used, unused }: any) { const log = () => console.error(used); log(); }


try { /* ... */ } catch { }
