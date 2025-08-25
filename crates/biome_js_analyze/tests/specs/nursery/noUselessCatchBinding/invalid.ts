// should generate diagnostics

try { /* ... */ } catch (unused: any) { }

try { /* ... */ } catch (_unused: any) { }


try { /* ... */ } catch ({ unused }: any) { }

try { /* ... */ } catch ({ _unused }: any) { }

try { /* ... */ } catch ({ unused, _unused }: any) { }


try { /* ... */ } catch (/* leading inner */ unused: any /* trailing inner */) { }

try { /* ... */ } catch /* leading outer */ (unused: any) /* trailing outer */ { }

try { /* ... */ } catch /* leading outer */ (/* leading inner */ unused: any /* trailing inner */) /* trailing outer */ { }

try { /* ... */ } catch /* leading outer */ (/* leading inner 1 */ { /* leading inner 2 */ unused: any /* trailing inner 2 */ } /* trailing inner 1 */) /* trailing outer */ { }


try { /* ... */ } catch ({ used: alias }: any) { }

try { /* ... */ } catch ({ nested: { unused } }: any) { }

try { /* ... */ } catch ({ ...rest }: any) { }
