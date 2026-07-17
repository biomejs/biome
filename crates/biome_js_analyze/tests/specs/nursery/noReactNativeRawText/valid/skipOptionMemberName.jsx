/* should not generate diagnostics */

const Title = ({ children }) => <Title.Text>{children}</Title.Text>;
Title.Text = ({ children }) => <Text>{children}</Text>;
export const A = () => <Title.Text>This is the title</Title.Text>;
