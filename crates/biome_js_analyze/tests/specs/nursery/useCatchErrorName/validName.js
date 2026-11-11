/* should not generate diagnostics */
try {} catch (exception) {}
try {} catch (exception__) {}
try {} catch (descriptiveException) {}
try {} catch (descriptive_exception__) {}
obj.catch((exception) => {});
obj.then(undefined, function (exception) {});
