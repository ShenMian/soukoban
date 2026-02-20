use soukoban::{ForwardAction, direction::Direction};

#[test]
fn action_from_char() {
    assert_eq!(
        ForwardAction::try_from('u'),
        Ok(ForwardAction::Move(Direction::Up))
    );
    assert_eq!(
        ForwardAction::try_from('d'),
        Ok(ForwardAction::Move(Direction::Down))
    );

    assert_eq!(
        ForwardAction::try_from('U'),
        Ok(ForwardAction::Push(Direction::Up))
    );
    assert_eq!(
        ForwardAction::try_from('D'),
        Ok(ForwardAction::Push(Direction::Down))
    );

    assert!(ForwardAction::try_from('x').is_err());
}

#[test]
fn action_to_char() {
    assert_eq!(char::from(ForwardAction::Move(Direction::Up)), 'u');
    assert_eq!(char::from(ForwardAction::Push(Direction::Up)), 'U');
}
