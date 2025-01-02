use soukoban::direction::*;

#[test]
fn rotate() {
    use Direction::*;
    assert_eq!(Up.rotate(), Right);
    assert_eq!(Right.rotate(), Down);
    assert_eq!(Down.rotate(), Left);
    assert_eq!(Left.rotate(), Up);
}

#[test]
fn flip() {
    use Direction::*;
    assert_eq!(Up.flip(), Down);
    assert_eq!(Down.flip(), Up);
    assert_eq!(Right.flip(), Left);
    assert_eq!(Left.flip(), Right);
}
