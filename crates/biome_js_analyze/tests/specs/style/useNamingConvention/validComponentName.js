/* should not generate diagnostics */
function loadWidgetComponent(widgetId) {
  const Component = getWidgetComponent(widgetId);
  if (!Component) return null;
  return <Component />;
}