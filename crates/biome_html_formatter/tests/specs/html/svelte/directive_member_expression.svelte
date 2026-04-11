<button use:obj.action>action</button>
<button use:obj.action={{leet: 1337}}>with params</button>
<div in:renderer.in|global out:renderer.out|global></div>
<div transition:lib.fade|local={{ duration: 200 }}>fading</div>
<div in:a.b.c|global>deep</div>
