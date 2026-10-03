/* should generate diagnostics */

export const RawText = () => <View>some text</View>;

export const TemplateLiteralInView = () => {
	const text = "some text";
	return <View>{`${text}`}</View>;
};

export const StringLiteralInView = () => <View>{"some text"}</View>;

export const WhitespaceInView = () => <View>  </View>;

export const TextInsideUnknownComponent = () => (
	<MyComponent myFunctionProp={() => {}}>my children</MyComponent>
);

export const AnimatedTextSibling = () => (
	<View>
		<Animated.View>some text</Animated.View>
	</View>
);
