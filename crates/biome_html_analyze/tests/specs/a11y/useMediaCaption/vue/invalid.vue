<!-- should generate diagnostics -->
<video></video>
<audio></audio>
<video>
    <source src="video.webm" type="video/webm" />
</video>
<audio>
    <source src="audio.ogg" type="audio/ogg" />
</audio>
<!-- track without kind="captions" -->
<video><track kind="subtitles" /></video>
<!-- track with missing kind attribute -->
<video><track src="captions.vtt" /></video>
<!-- track with empty kind attribute -->
<video><track kind="" /></video>
<!-- muted audio still requires captions -->
<audio muted></audio>
<!-- static Vue bindings with non-captions kind should also be flagged -->
<video><track :kind="'subtitles'" /></video>
<audio><track v-bind:kind="'metadata'" /></audio>
