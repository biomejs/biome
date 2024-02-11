import prettierIcon from "@/assets/svg/prettier-icon-dark.svg";

export default function PrettierHeader() {
	return (
		<>
			<img alt="Prettier logo" src={prettierIcon.src} />
			<span>Prettier</span>
		</>
	);
}
