<form onsubmit={(e)=>{const x={a:1,b:2}; e.preventDefault(); save(x) }}>
  <button on:click={(e)=>{e.preventDefault(); run("save")}}>Go</button>
</form>
