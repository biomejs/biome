<!-- should not generate diagnostics -->
<div class={'bg-pink-500' === state ? 'bg-primary' : 'text-white'}></div>
<div class={'bg-pink-500' && 'bg-primary'}></div>
<div class={{ 'bg-primary': 'bg-pink-500' }}></div>
<div class={{ 'bg-primary': active ? 'bg-pink-500' : 'hover:text-red-500/80' }}></div>
<div class={lookup['bg-pink-500']}></div>
<div class={lookup('bg-pink-500')}></div>
<div class={lookup(`bg-pink-500`)}></div>
<div class={(() => 'bg-pink-500')()}></div>
<div class={`bg-primary ${state === 'bg-pink-500' ? 'text-white' : 'border-transparent'}`}></div>
<div class={('bg-pink-500', 'bg-primary')}></div>
<div id={'bg-pink-500'}></div>
<div class:active={'bg-pink-500'}></div>
{'bg-pink-500'}
<div class={[...'bg-pink-500']}></div>
<div class={{...['bg-pink-500']}}></div>
<div class={clsx(...'bg-pink-500')}></div>
<div class={`${{ 'bg-pink-500': active }}`}></div>
<div class={({ 'bg-pink-500': active }) + ''}></div>
<div class={{ [({ 'bg-pink-500': active })]: true }}></div>
<div class={`${['bg-pink-500', 'text-white']}`}></div>
<div class={cached -= 'bg-pink-500'}></div>
<div class={cached['bg-pink-500'] = 'bg-primary'}></div>
<div class={["bg-white text-black", active && "border-transparent fill-current stroke-inherit", "bg-[#ff00aa]"]}></div>
<div class={{ "bg-primary": active, "text-white": selected }}></div>
