/* should not generate diagnostics */
try {} catch (skipThisNameCheck) {}
try {} catch (error1) {}
try {} catch (error1__) {}
promise.catch((unicorn) => {});
try {} catch (_) {
	console.log(_);
}
