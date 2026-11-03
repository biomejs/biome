use super::*;

const FIRST: ClientId = ClientId(Some(1));
const SECOND: ClientId = ClientId(Some(2));

#[test]
fn clients_read_their_own_editor_view_or_the_disk_view() {
    let views = Views::from_disk("disk")
        .with_editor(FIRST, "first")
        .with_editor(SECOND, "second");

    assert_eq!(views.current(), &"first");
    assert_eq!(views.for_client(FIRST), Some(&"first"));
    assert_eq!(views.for_client(SECOND), Some(&"second"));
    assert_eq!(views.for_client(ClientId(Some(3))), Some(&"disk"));
}

#[test]
fn closing_the_primary_editor_view_promotes_the_next_one() {
    let views = Views::from_editor(FIRST, "first").with_editor(SECOND, "second");

    let views = views.without_editor(FIRST).unwrap();
    assert_eq!(views.current(), &"second");
    assert_eq!(views.for_client(FIRST), None);

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

    assert_eq!(views.current(), &"first, changed");
    assert_eq!(views.for_client(SECOND), Some(&"second"));
}

/// Opens and closes `closed` files in a map from [new_views_map] the way the
/// workspace does, then returns the shortest time of several rounds of
/// lookups of a file that is open.
fn lookup_time_after_closing(closed: usize) -> std::time::Duration {
    use papaya::Operation;

    let map = new_views_map(rustc_hash::FxBuildHasher);
    let open = |path: &camino::Utf8Path, index: usize| {
        map.pin()
            .update_or_insert_with(path.to_path_buf(), |index| *index, || index);
    };
    for index in 0..closed {
        let path = camino::Utf8PathBuf::from(format!("/project/src/file{index}.ts"));
        open(&path, index);
        let pinned = map.pin();
        let result = pinned.compute(path, |entry| match entry {
            Some(_) => Operation::Remove,
            None => Operation::Abort(()),
        });
        assert!(matches!(result, papaya::Compute::Removed(..)));
    }
    let path = camino::Utf8PathBuf::from("/project/src/open.ts");
    open(&path, closed);

    (0..5)
        .map(|_| {
            let start = std::time::Instant::now();
            for _ in 0..100 {
                assert!(std::hint::black_box(map.pin().get(path.as_path())).is_some());
            }
            start.elapsed()
        })
        .min()
        .unwrap()
}

#[test]
fn views_map_lookups_do_not_slow_down_as_files_are_closed() {
    let after_few = lookup_time_after_closing(100);
    let after_many = lookup_time_after_closing(20_000);
    assert!(
        after_many < after_few * 5,
        "100 lookups took {after_few:?} after closing 100 files, and {after_many:?} after closing 20,000"
    );
}
