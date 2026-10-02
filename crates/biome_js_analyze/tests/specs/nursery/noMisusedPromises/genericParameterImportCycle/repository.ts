/* should not generate diagnostics */

import type { Entity } from "./entity";

export interface Repository<E extends Entity<any> = Entity<any>> {
	find(): Promise<E>;
}
