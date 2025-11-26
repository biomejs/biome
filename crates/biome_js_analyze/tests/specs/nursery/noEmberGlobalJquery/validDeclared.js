// should not generate diagnostics
const $ = document.querySelectorAll.bind(document);
const jQuery = (selector) => document.querySelectorAll(selector);

// This is OK because $ and jQuery are locally declared, not global
$('.selector').forEach(el => el.style.display = 'none');

function withLocalJQuery() {
  // This is also OK because jQuery is locally scoped above
  jQuery('.another-selector');
}
