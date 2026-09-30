/* should not generate diagnostics */

import type { Repository } from "./repository";

export interface Entity<R extends Repository<any> = Repository<any>> {
	repository: R;
}
