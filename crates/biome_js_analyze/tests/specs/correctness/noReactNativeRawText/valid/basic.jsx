/* should not generate diagnostics */

export const TextChild = () => <View><Text>some text</Text></View>;

export const TemplateLiteralInText = () => {
	const text = "some text";
	return <View><Text>{`${text}`}</Text></View>;
};

export const StringLiteralInText = () => <View><Text>{"some text"}</Text></View>;

export const MultilineLayout = () => (
	<View>
		<Text>some text</Text>
	</View>
);

export const NestedTSpan = () => (
	<Svg>
		<Text>
			<TSpan>some text</TSpan>
		</Text>
	</Svg>
);

export const DefaultAllowedStyledText = () => <StyledText>some text</StyledText>;

export const TextWithStyleProp = () => (
	<View>
		<Text style={{ color: "red" }}>some text</Text>
	</View>
);

export const FragmentInsideText = () => <Text>Some text <>More text</></Text>;

export const AnimatedText = () => <Animated.Text>animated text</Animated.Text>;

export const IdentifierExpression = ({ children }) => <View>{children}</View>;

export const EmptyWhitespaceWithNewline = () => (
	<View>
	</View>
);
