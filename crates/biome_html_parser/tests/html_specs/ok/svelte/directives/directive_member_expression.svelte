<!-- simple member expression -->
<button use:obj.action>action</button>
<!-- member expression with params -->
<button use:obj.action={{leet: 1337}}>with params</button>
<!-- in/out with member expression and modifier -->
<div in:renderer.in|global out:renderer.out|global></div>
<!-- transition with member expression, modifier, and params -->
<div transition:lib.fade|local={{ duration: 200 }}>fading</div>
<div transition:renderer.slide|global>sliding</div>
<!-- deep member expression -->
<div in:a.b.c|global>deep</div>
