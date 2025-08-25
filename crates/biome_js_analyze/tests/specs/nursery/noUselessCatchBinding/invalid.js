// should generate diagnostics

try { /* ... */ } catch (unused) { }

try { /* ... */ } catch (_unused) { }


try { /* ... */ } catch ({ unused }) { }

try { /* ... */ } catch ({ _unused }) { }

try { /* ... */ } catch ({ unused, _unused }) { }


try { /* ... */ } catch (/* leading inner */ unused /* trailing inner */) { }

try { /* ... */ } catch /* leading outer */ (unused) /* trailing outer */ { }

try { /* ... */ } catch /* leading outer */ (/* leading inner */ unused /* trailing inner */) /* trailing outer */ { }

try { /* ... */ } catch /* leading outer */ (/* leading inner 1 */ { /* leading inner 2 */ unused /* trailing inner 2 */ } /* trailing inner 1 */) /* trailing outer */ { }


try { /* ... */ } catch ({ used: alias }) { }

try { /* ... */ } catch ({ nested: { unused } }) { }

try { /* ... */ } catch ({ ...rest }) { }
