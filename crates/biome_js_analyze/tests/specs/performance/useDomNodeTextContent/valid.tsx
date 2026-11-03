/* should not generate diagnostics */
export const CreateAccountLinkAnchor = ({
    innerText,
    children,
}: {
    innerText: string;
    children: React.ReactNode;
}) => <a onClick={() => emitClick({ innerText })}>{children}</a>;
