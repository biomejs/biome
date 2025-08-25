// should not generate diagnostics

try { /* ... */ } catch { }


try { /* ... */ } catch (used) { console.error(used); }

try { /* ... */ } catch ({ used }) { console.error(used); }

try { /* ... */ } catch ({ used, unused }) { console.error(used); }


try { /* ... */ } catch (used) { const log = () => console.error(used); log(); }

try { /* ... */ } catch ({ used }) { const log = () => console.error(used); log(); }

try { /* ... */ } catch ({ used, unused }) { const log = () => console.error(used); log(); }


try { /* ... */ } catch ({ used: alias }) { console.error(alias); }

try { /* ... */ } catch ({ nested: { unused } }) { console.error(unused); }

try { /* ... */ } catch ({ ...rest }) { console.error(rest); }
