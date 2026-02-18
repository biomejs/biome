/* should not generate diagnostics */
<>
	<App class="bar" />
	<App for="bar" />
	<App someProp="bar" />
	<Foo.bar for="bar" />
	<App accept-charset="bar" />
	<App http-equiv="bar" />

	<App xlink:href="bar" />
	<App clip-path="bar" />
	<div className="bar"></div>
	<div onMouseDown={this._onMouseDown}></div>
	<a href="someLink" download="foo">
		Read more
	</a>
	<area download="foo" />
	<img
		src="cat_keyboard.jpeg"
		alt="A cat sleeping on a keyboard"
		align="top"
		fetchPriority="high"
	/>
	<input type="password" required />
	<input ref={this.input} type="radio" />
	<input type="file" webkitdirectory="" />
	<input type="file" webkitDirectory="" />
	<div inert children="anything" />
	<iframe scrolling="?" onLoad={a} onError={b} align="top" />
	<input key="bar" type="radio" />
	<button disabled>You cannot click me</button>
	<svg
		key="lock"
		viewBox="box"
		fill={10}
		d="d"
		stroke={1}
		strokeWidth={2}
		strokeLinecap={3}
		strokeLinejoin={4}
		transform="something"
		clipRule="else"
		x1={5}
		x2="6"
		y1="7"
		y2="8"
	></svg>
	<svg>
		<path
			d="M11.293 8H5.57c-.528 0-.771.79-.37 1.205l2.406 2.481z"
			fill="currentColor"
			opacity="0.5"
		/>
	</svg>
	<g fill="#7B82A0" fillRule="evenodd"></g>
	<mask fill="#7B82A0"></mask>
	<symbol fill="#7B82A0"></symbol>
	<meta property="og:type" content="website" />
	<input type="checkbox" checked={checked} disabled={disabled} id={id} onChange={onChange} />
	<video playsInline />
	<img onError={foo} onLoad={bar} />
	<picture inert={false} onError={foo} onLoad={bar} />
	<iframe onError={foo} onLoad={bar} />
	<script onLoad={bar} onError={foo} />
	<source onLoad={bar} onError={foo} />
	<link onLoad={bar} onError={foo} />
	<link
		rel="preload"
		as="image"
		href="someHref"
		imageSrcSet="someImageSrcSet"
		imageSizes="someImageSizes"
	/>
	<object onLoad={bar} />
	<video allowFullScreen webkitAllowFullScreen mozAllowFullScreen />
	<iframe allowFullScreen webkitAllowFullScreen mozAllowFullScreen />
	<table border="1" />
	<th abbr="abbr" />
	<td abbr="abbr" />
	<div onPointerDown={this.onDown} onPointerUp={this.onUp} />
	<input type="checkbox" defaultChecked={this.state.checkbox} />
	<div
		onTouchStart={this.startAnimation}
		onTouchEnd={this.stopAnimation}
		onTouchCancel={this.cancel}
		onTouchMove={this.move}
		onMouseMoveCapture={this.capture}
		onTouchCancelCapture={this.log}
	/>
	<meta charset="utf-8" />
	<meta charSet="utf-8" />
	<div class="foo" is="my-elem"></div>
	<div {...this.props} class="foo" is="my-elem"></div>
	<atom-panel class="foo"></atom-panel>
	<div data-foo="bar"></div>
	<div data-foo-bar="baz"></div>
	<div data-parent="parent"></div>
	<div data-index-number="1234"></div>
	<div data-e2e-id="5678"></div>
	<div data-testID="bar" data-under_sCoRe="bar" />
	<button aria-haspopup="true">Click me to open pop up</button>
	<button aria-label="Close" onClick={someThing.close} />
	<script crossOrigin noModule />
	<audio crossOrigin />
	<svg focusable>
		<image crossOrigin />
	</svg>
	<details onToggle={this.onToggle}>Some details</details>
	<path
		fill="pink"
		d="M 10,30 A 20,20 0,0,1 50,30 A 20,20 0,0,1 90,30 Q 90,60 50,90 Q 10,60 10,30 z"
	></path>
	<line fill="pink" x1="0" y1="80" x2="100" y2="20"></line>
	<link as="audio">Audio content</link>
	<video
		controlsList="nodownload"
		controls={this.controls}
		loop={true}
		muted={false}
		src={this.videoSrc}
		playsInline={true}
		onResize={this.onResize}
	></video>
	<audio
		controlsList="nodownload"
		controls={this.controls}
		crossOrigin="anonymous"
		disableRemotePlayback
		loop
		muted
		preload="none"
		src="something"
		onAbort={this.abort}
		onDurationChange={this.durationChange}
		onEmptied={this.emptied}
		onEnded={this.end}
		onError={this.error}
		onResize={this.onResize}
	></audio>
	<marker
		id={markerId}
		viewBox="0 0 2 2"
		refX="1"
		refY="1"
		markerWidth="1"
		markerHeight="1"
		orient="auto"
	/>
	<pattern id="pattern" viewBox="0,0,10,10" width="10%" height="10%" />
	<symbol id="myDot" width="10" height="10" viewBox="0 0 2 2" />
	<view id="one" viewBox="0 0 100 100" />
	<hr align="top" />
	<applet align="top" />
	<marker fill="#000" />
	<dialog closedby="any" onClose={handler} open id="dialog" returnValue="something" onCancel={handler2} />

	<table align="top">
		<caption align="top">Table Caption</caption>
		<colgroup valign="top" align="top">
			<col valign="top" align="top" />
		</colgroup>
		<thead valign="top" align="top">
			<tr valign="top" align="top">
				<th valign="top" align="top">
					Header
				</th>
				<td valign="top" align="top">
					Cell
				</td>
			</tr>
		</thead>
		<tbody valign="top" align="top" />
		<tfoot valign="top" align="top" />
	</table>

	<fbt desc="foo" doNotExtract />
	<fbs desc="foo" doNotExtract />
	<math displaystyle="true" />

	<div
		className="App"
		data-crash-crash-crash-crash-crash-crash-crash-crash-crash-crash-crash-crash-crash-crash-crash-crash-crash-crash-crash-crash-crash-crash-crash-crash-crash-crash-crash-crash-crash-crash-crash-crash-crash-crash-crash-crash-crash-crash-crash-crash-crash-crash-crash-crash-crash-crash="customValue"
	>
		Hello, world!
	</div>

	<div>
		<button popovertarget="my-popover" popovertargetaction="toggle">
			Open Popover
		</button>

		<div popover id="my-popover">
			Greetings, one and all!
		</div>
	</div>
</>;
