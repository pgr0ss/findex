use color_eyre::eyre;
use findex;

#[test]
fn add_and_query() -> eyre::Result<()> {
    findex::add(false, &"tests/test_data".to_string())?;

    let entries = findex::query()?;

    let found = entries
        .iter()
        .find(|entry| entry.filename.ends_with("hello.txt"))
        .unwrap();

    assert!(found
        .filename
        .ends_with("/findex/tests/test_data/hello.txt"));

    assert_eq!(
        found.sha256,
        "a948904f2f0f479b8f8197694b30184b0d2ed1c1cd2a1ec0fb85d299a192a447"
    );

    assert_eq!(found.size, 12);

    Ok(())
}
