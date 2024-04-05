export function formatNumber1(
	value: string,
	{
		a,
		b,
		c,
		formatNumber,
		...props
	}: Omit<NumberFormatterProps, 'value' | 'defaultFractionDigits'> & {
		useGrouping?: boolean;
	}
): string {}

export function formatNumber2(
	value: string,
	{ a }: Omit<NumberFormatterProps, 'value' | 'defaultFractionDigits'> & {
		useGrouping?: boolean;
	}
): string {}


export const findByDatefindByDatefindByDatefindByDate =
	(_, { date }, { req } ) => findByDatefindByDatefindByDatefindByDate;

export const queryAuditLog = async ({
		startDate,
		endDate,
		jobId,
		src,
		type,
	}: Filter): Promise<DBAuditLog[]> => {

};
