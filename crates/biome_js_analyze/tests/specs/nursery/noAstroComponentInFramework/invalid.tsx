// should generate diagnostics
import Card, { helper } from "./Card.astro";
import { default as Other } from "./Card.astro";
import { Image, Picture as Photo, getImage } from "astro:assets";
export default () => <Card><Other /><Image /><Photo /></Card>;
