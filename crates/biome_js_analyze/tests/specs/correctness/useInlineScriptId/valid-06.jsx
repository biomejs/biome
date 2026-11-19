/* should not generate diagnostics */
import Script from 'next/script'

export function Component() {
	return (
		<Script
			id="id"
		>
			{`console.log("false positive");`}
		</Script>
	);
}
