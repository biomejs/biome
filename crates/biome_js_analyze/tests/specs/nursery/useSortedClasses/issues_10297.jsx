function Example({ isActive }) {
	return (
		<>
			<span className={`map-panel-basemap-map-a${isActive ? ' active' : ''}`} />
			<span className={`map-panel-basemap-map-a${isActive ? ' active ' : ''}map-panel-overlay`} />
			<span className={`${isActive ? 'active ' : ''}map-panel-basemap-map-a`} />
			<span className={`map-panel-basemap-map-a ${isActive ? ' active ' : ''} map-panel-overlay`} />
		</>
	);
}
