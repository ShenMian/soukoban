use soukoban::{Action, direction::Direction};

#[test]
fn action_from_char() {
    use {Action::*, Direction::*};

    assert_eq!(Action::try_from('u'), Ok(Move(Up)));
    assert_eq!(Action::try_from('U'), Ok(Push(Up)));
    assert_eq!(Action::try_from('d'), Ok(Move(Down)));
    assert_eq!(Action::try_from('D'), Ok(Push(Down)));
    assert_eq!(Action::try_from('l'), Ok(Move(Left)));
    assert_eq!(Action::try_from('L'), Ok(Push(Left)));
    assert_eq!(Action::try_from('r'), Ok(Move(Right)));
    assert_eq!(Action::try_from('R'), Ok(Push(Right)));

    assert!(Action::try_from('x').is_err());
}

#[test]
fn action_to_char() {
    use {Action::*, Direction::*};

    assert_eq!(char::from(Move(Up)), 'u');
    assert_eq!(char::from(Push(Up)), 'U');
    assert_eq!(char::from(Move(Down)), 'd');
    assert_eq!(char::from(Push(Down)), 'D');
    assert_eq!(char::from(Move(Left)), 'l');
    assert_eq!(char::from(Push(Left)), 'L');
    assert_eq!(char::from(Move(Right)), 'r');
    assert_eq!(char::from(Push(Right)), 'R');
}
