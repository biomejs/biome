import { Controller } from "./decorators";
import { Service } from "./service";

@Controller()
class AppController {
	constructor(private readonly service: Service) {}
}
