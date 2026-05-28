/* should not generate diagnostics */

import { Controller, Inject } from "./decorators";
import DefaultService, { NamedService } from "./service";
import * as Services from "./services";

@Controller()
class AppController {
	constructor(
		private readonly defaultService: DefaultService,
		private readonly namedService: NamedService,
		private readonly namespacedService: Services.NamespacedService,
	) {}
}

class ParameterDecoratorController {
	constructor(@Inject("service") private readonly namedService: NamedService) {}
}
