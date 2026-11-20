// should not generate diagnostics
import type CardProps from "./Card.astro";
import { type Image, getImage } from "astro:assets";
import { helper } from "./Card.astro";
import source from "./Card.astro?raw";
import Card from "./Card.jsx";
export default () => <Card />;
