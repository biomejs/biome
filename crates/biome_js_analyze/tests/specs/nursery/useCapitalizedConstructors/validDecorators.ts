/* should not generate diagnostics */
@Component({ selector: "app-root" })
class AppComponent {
	@Input() name: string;
	@(Output()) changed;

	constructor(@Inject(TOKEN) private readonly token: string) {}
}

@decorators.Injectable()
class Service {}
