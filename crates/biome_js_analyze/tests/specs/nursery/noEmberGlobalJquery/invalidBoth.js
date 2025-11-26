// Using both global $ and jQuery in the same file
$('.selector').hide();
jQuery('#element').show();

function mixed() {
  $('.foo').addClass('bar');
  jQuery('.baz').removeClass('qux');
}
