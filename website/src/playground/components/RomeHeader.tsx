import biomeIcon from "../../svg/logomark.svg";
import biomeDarkIcon from "../../svg/logomark_white_yellow.svg";
import { useTheme } from "../utils";

export default function RomeHeader() {
	const theme = useTheme();

	return (
		<>
			<img
				alt="Biome logo"
				src={theme === "dark" ? biomeDarkIcon : biomeIcon}
			/>
			<span>Biome</span>
		</>
	);
}
