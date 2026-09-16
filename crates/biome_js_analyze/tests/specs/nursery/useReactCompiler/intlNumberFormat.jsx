// should not generate diagnostics

export function MyPage() {
    const percentageFormat = Intl.NumberFormat();
    return <p>{percentageFormat.format(123)}</p>;
}
