<button onclick={(event)=>{
  /* before */ run(  event );
  finish(); // after
}} aria-label="Run">Go</button>
<button onclick={(event)=>{
  // first
  run(event)
  // last
}} title="other">Go</button>
<button onclick={/* leading */ (event) => doThing( event ) /* trailing */}>Go</button>
<Component value={((value))} bind:value={(value)} />
<Component value={/* keep */ value} />
<Component value={value /* trailing */} />
<Component value={value // line comment
} />
<!-- biome-ignore format: preserve this element -->
<button onclick={(e)=>{ run( e ) }}>Go</button>
