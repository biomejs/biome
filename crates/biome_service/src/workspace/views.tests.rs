use super::*;

const FIRST: ClientId = ClientId(Some(1));
const SECOND: ClientId = ClientId(Some(2));

#[test]
fn clients_read_their_own_editor_view_or_the_disk_view() {
    let views = Views::from_disk("disk")
        .with_editor(FIRST, "first")
        .with_editor(SECOND, "second");

    assert_eq!(views.current().value(), &"first");
    assert_eq!(views.for_client(FIRST).map(View::value), Some(&"first"));
    assert_eq!(views.for_client(SECOND).map(View::value), Some(&"second"));
    assert_eq!(
        views.for_client(ClientId(Some(3))).map(View::value),
        Some(&"disk")
    );
}

#[test]
fn closing_the_primary_editor_view_promotes_the_next_one() {
    let views = Views::from_editor(FIRST, "first").with_editor(SECOND, "second");

    let views = views.without_editor(FIRST).unwrap();
    assert_eq!(views.current().value(), &"second");
    assert_eq!(views.for_client(FIRST).map(View::value), None);

    assert_eq!(views.without_editor(SECOND), None);
}

#[test]
fn closing_the_last_editor_view_falls_back_to_the_disk_view() {
    let views = Views::from_disk("disk").with_editor(FIRST, "first");

    let views = views.without_editor(FIRST).unwrap();
    assert_eq!(views, Views::from_disk("disk"));
}

#[test]
fn updating_an_editor_view_keeps_its_position() {
    let views = Views::from_editor(FIRST, "first")
        .with_editor(SECOND, "second")
        .with_editor(FIRST, "first, changed");

    assert_eq!(views.current().value(), &"first, changed");
    assert_eq!(views.for_client(SECOND).map(View::value), Some(&"second"));
}
