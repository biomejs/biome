<>
	<span className={`map-panel-basemap-map-a${isActive ? ' active' : ''}`}>a</span>
	<span className={`card${isActive ? ' p-4 flex' : ''}`}>b</span>
	<span className={`${isActive ? 'p-4 flex ' : ''}base`}>c</span>
	<span className={`wrap${isActive ? ' p-4 flex ' : ''}end`}>d</span>
	<span className={`${someFunc('p-4 flex')}base`}>e</span>
	<span className={`${isActive ? 'p-4 flex m-2' : ''}base`}>f</span>
	<span className={`${prefix}${isActive ? ' p-4 flex' : ''}`}>g</span>
	<span className={`card${isActive && ' p-4 flex'}`}>h</span>
</>
