import { Controller } from "./decorators";
import { MethodService } from "./service";

@Controller()
class AppController {
	method(service: MethodService) {}
}
