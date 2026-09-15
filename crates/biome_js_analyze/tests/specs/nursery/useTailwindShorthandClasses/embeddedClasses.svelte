<!-- should generate diagnostics -->
<div class={["w-4 h-4", selected && "px-2 py-2"]}></div>
<div class={{ "mr-3 ml-3": active }}></div>
<div class={active ? `w-4 h-4` : "px-2 py-2"}></div>
<div class={['w-4 h-4', [active && 'px-2 py-2']]}></div>
<div class={{ ['mr-3 ml-3']: active }}></div>
<div class={`w-4 h-4 ${active ? 'px-2 py-2' : 'mr-3 ml-3'}`}></div>
<div class={classes || 'w-4 h-4'}></div>
<div class={classes ?? 'px-2 py-2'}></div>
<div class={(ignored, 'w-4 h-4')}></div>
<div class={'w-4 h-4 ' + extra}></div>
<div class={[...['w-4 h-4']]}></div>
<div class={{...{'px-2 py-2': active}}}></div>
<div class={cached = 'w-4 h-4'}></div>
<div class={cached += ' w-4 h-4'}></div>
<div class={cached ??= 'w-4 h-4'}></div>
