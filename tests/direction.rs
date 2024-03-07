use soukoban::direction::*;

#[test]
fn test_rotate() {
    use Direction::*;
    assert_eq!(Up.rotate(), Right);
    assert_eq!(Right.rotate(), Down);
    assert_eq!(Down.rotate(), Left);
    assert_eq!(Left.rotate(), Up);
}

#[test]
fn test_flip() {
    use Direction::*;
    assert_eq!(Up.flip(), Down);
    assert_eq!(Down.flip(), Up);
    assert_eq!(Right.flip(), Left);
    assert_eq!(Left.flip(), Right);
}
