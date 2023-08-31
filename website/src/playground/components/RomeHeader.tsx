import biomeIcon from "../../assets/svg/logomark.svg";

export default function RomeHeader() {
	console.log(biomeIcon);
	return (
		<>
			<img alt="Biome logo" src={biomeIcon.src} />
			<span>Biome</span>
		</>
	);
}
