// should not generate diagnostics

try { /* ... */ } catch (used) { console.error(used); }

try { /* ... */ } catch ({ used }) { console.error(used); }

try { /* ... */ } catch ({ used, unused }) { console.error(used); }


try { /* ... */ } catch (used) { const log = () => console.error(used); log(); }

try { /* ... */ } catch ({ used }) { const log = () => console.error(used); log(); }

try { /* ... */ } catch ({ used, unused }) { const log = () => console.error(used); log(); }


try { /* ... */ } catch { }
