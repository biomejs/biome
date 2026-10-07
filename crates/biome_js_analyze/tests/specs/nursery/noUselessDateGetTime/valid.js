/* should not generate diagnostics */
new Date(date);
date.getTime();
new Date(...date.getTime());
new Date(getTime());
new Date(date.getTime(), extraArgument);
new Date(date.not_getTime());
new Date(date?.getTime());
new Date(foo?.bar.getTime());
new Date(date.getTime?.());
new Date((date?.getTime)());
new NotDate(date.getTime());
new Date(date[getTime]());
new Date(date["getTime"]());
new Date(date.getTime(extraArgument));
new Date(date.#getTime());
Date(date.getTime());
new Date(
	date.getFullYear(),
	date.getMonth(),
	date.getDate(),
	date.getHours(),
	date.getMinutes(),
	date.getSeconds(),
	date.getMilliseconds(),
);
function shadowed(Date) {
	return new Date(date.getTime());
}
