// VIOLATION SHOWCASE: dom-helpers.js
// Demonstrates global jQuery violations

export function setupTooltips() {
  // ❌ VIOLATION: noEmberGlobalJquery
  // Using global $ is discouraged - import jQuery explicitly or use native DOM
  $('.tooltip').tooltip();

  // ❌ VIOLATION: noEmberGlobalJquery
  // Global jQuery reference
  jQuery('.modal').modal('show');
}

export function getFormData(formId) {
  // ❌ VIOLATION: noEmberGlobalJquery
  return $('#' + formId).serialize();
}

export function animateElement(selector) {
  // ❌ VIOLATION: noEmberGlobalJquery
  $(selector).fadeIn(300);
}

export function bindEvents() {
  // ❌ VIOLATION: noEmberGlobalJquery (multiple)
  $('.btn').on('click', function() {
    alert('Clicked!');
  });

  $('input').on('change', function() {
    console.log('Changed:', $(this).val());
  });
}

/* ✅ FIXED VERSION:

// Use native DOM APIs
export function setupTooltips() {
  document.querySelectorAll('.tooltip').forEach(el => {
    // Initialize tooltip with native JS or proper library
    el.setAttribute('data-tooltip', 'initialized');
  });
}

export function getFormData(formId) {
  const form = document.getElementById(formId);
  return new FormData(form);
}

export function animateElement(selector) {
  const element = document.querySelector(selector);
  element.style.opacity = '0';
  element.style.display = 'block';

  requestAnimationFrame(() => {
    element.style.transition = 'opacity 300ms';
    element.style.opacity = '1';
  });
}

export function bindEvents() {
  document.querySelectorAll('.btn').forEach(btn => {
    btn.addEventListener('click', () => {
      alert('Clicked!');
    });
  });

  document.querySelectorAll('input').forEach(input => {
    input.addEventListener('change', (e) => {
      console.log('Changed:', e.target.value);
    });
  });
}
*/
