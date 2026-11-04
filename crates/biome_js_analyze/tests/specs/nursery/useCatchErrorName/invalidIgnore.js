/* should generate diagnostics */
try {} catch (notMatching) {}
try {} catch (unicorns) {}
promise.catch((notMatching) => {});
