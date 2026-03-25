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
    assert_eq!(-Up, Down);
    assert_eq!(-Down, Up);
    assert_eq!(-Right, Left);
    assert_eq!(-Left, Right);
}

#[test]
fn flip_horizontal() {
    use Direction::*;
    assert_eq!(Up.flip_horizontal(), Up);
    assert_eq!(Down.flip_horizontal(), Down);
    assert_eq!(Right.flip_horizontal(), Left);
    assert_eq!(Left.flip_horizontal(), Right);
}

#[test]
fn flip_vertical() {
    use Direction::*;
    assert_eq!(Up.flip_vertical(), Down);
    assert_eq!(Down.flip_vertical(), Up);
    assert_eq!(Right.flip_vertical(), Right);
    assert_eq!(Left.flip_vertical(), Left);
}
