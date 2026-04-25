use soukoban::{Vector2, direction::*};

#[test]
fn rotate_cw() {
    use Direction::*;
    assert_eq!(Up.rotate_cw(), Right);
    assert_eq!(Right.rotate_cw(), Down);
    assert_eq!(Down.rotate_cw(), Left);
    assert_eq!(Left.rotate_cw(), Up);
}

#[test]
fn rotate_ccw() {
    use Direction::*;
    assert_eq!(Up.rotate_ccw(), Left);
    assert_eq!(Right.rotate_ccw(), Up);
    assert_eq!(Down.rotate_ccw(), Right);
    assert_eq!(Left.rotate_ccw(), Down);
}

#[test]
fn negate() {
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

#[test]
fn to_vector2() {
    use Direction::*;
    assert_eq!(Vector2::<i32>::from(Up), -Vector2::y());
    assert_eq!(Vector2::<i32>::from(Right), Vector2::x());
    assert_eq!(Vector2::<i32>::from(Down), Vector2::y());
    assert_eq!(Vector2::<i32>::from(Left), -Vector2::x());
}

#[test]
fn try_from_vector2() {
    use Direction::*;
    assert_eq!(Direction::try_from(-Vector2::<i32>::y()), Ok(Up));
    assert_eq!(Direction::try_from(Vector2::<i32>::x()), Ok(Right));
    assert_eq!(Direction::try_from(Vector2::<i32>::y()), Ok(Down));
    assert_eq!(Direction::try_from(-Vector2::<i32>::x()), Ok(Left));
    assert_eq!(Direction::try_from(Vector2::new(1, 1)), Err(()));
    assert_eq!(Direction::try_from(Vector2::new(0, 0)), Err(()));
}
