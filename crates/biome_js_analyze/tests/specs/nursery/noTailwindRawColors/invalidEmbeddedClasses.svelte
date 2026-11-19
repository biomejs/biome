<!-- should generate diagnostics -->
<div class={["bg-pink-500", selected && "hover:text-red-500/80"]}></div>
<div class={{ "border-slate-950": active }}></div>
<div class={active ? `bg-pink-500` : "hover:text-red-500/80"}></div>
<div class={['bg-pink-500', [active && 'hover:text-red-500/80']]}></div>
<div class={{ ['border-slate-950']: active }}></div>
<div class={`bg-pink-500 ${active ? 'hover:text-red-500/80' : 'border-slate-950'}`}></div>
<div class={classes || 'bg-pink-500'}></div>
<div class={classes ?? 'hover:text-red-500/80'}></div>
<div class={(ignored, 'bg-pink-500')}></div>
<div class={'bg-pink-500 ' + extra}></div>
<div class={[...['bg-pink-500']]}></div>
<div class={{...{'hover:text-red-500/80': active}}}></div>
<div class={cached = 'bg-pink-500'}></div>
<div class={cached += ' bg-pink-500'}></div>
<div class={cached ??= 'bg-pink-500'}></div>
