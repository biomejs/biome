/* should not generate diagnostics */

interface InjectionKey<T> extends Symbol {}

declare function inject<T>(key: InjectionKey<T> | string): T | undefined;
declare function inject<T>(
	key: InjectionKey<T> | string,
	defaultValue: T,
	treatDefaultAsFactory?: false,
): T;

type UiContainerLoading = {
	isLoading: boolean;
};

declare const UiContainerLoadingKey: InjectionKey<UiContainerLoading>;

export function useContainerLoading(): UiContainerLoading {
	const loading = inject(UiContainerLoadingKey, undefined);

	if (loading) {
		return loading;
	}

	return { isLoading: false };
}
