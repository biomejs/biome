import biomeIcon from "../../assets/svg/logomark.svg";

export default function BiomeHeader() {
	console.log(biomeIcon);
	return (
		<>
			<img alt="Biome logo" src={biomeIcon.src} />
			<span>Biome</span>
		</>
	);
}
