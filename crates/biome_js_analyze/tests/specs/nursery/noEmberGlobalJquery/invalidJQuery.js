// Using global jQuery should trigger the rule
jQuery('.selector').addClass('active');

function anotherFunction() {
  jQuery('#element').remove();
}
