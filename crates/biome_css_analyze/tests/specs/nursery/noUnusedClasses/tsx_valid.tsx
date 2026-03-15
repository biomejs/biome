import "./tsx_valid.css";

interface Props {
	label: string;
}

export function Layout({ label }: Props) {
	return <div className="container">{label}</div>;
}
