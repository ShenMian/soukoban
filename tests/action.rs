use soukoban::{direction::Direction, Action};

#[test]
fn action_from_char() {
    assert_eq!(Action::try_from('u'), Ok(Action::Move(Direction::Up)));
    assert_eq!(Action::try_from('d'), Ok(Action::Move(Direction::Down)));

    assert_eq!(Action::try_from('U'), Ok(Action::Push(Direction::Up)));
    assert_eq!(Action::try_from('D'), Ok(Action::Push(Direction::Down)));

    assert!(Action::try_from('x').is_err());
}

#[test]
fn action_to_char() {
    assert_eq!(char::from(Action::Move(Direction::Up)), 'u');
    assert_eq!(char::from(Action::Push(Direction::Up)), 'U');
}
