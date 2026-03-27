use soukoban::{Action, direction::Direction};

#[test]
fn from_char() {
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
fn to_char() {
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

#[test]
fn rotate() {
    use {Action::*, Direction::*};

    assert_eq!(Move(Up).rotate(), Move(Right));
    assert_eq!(Move(Right).rotate(), Move(Down));
    assert_eq!(Move(Down).rotate(), Move(Left));
    assert_eq!(Move(Left).rotate(), Move(Up));

    assert_eq!(Push(Up).rotate(), Push(Right));
    assert_eq!(Push(Right).rotate(), Push(Down));
    assert_eq!(Push(Down).rotate(), Push(Left));
    assert_eq!(Push(Left).rotate(), Push(Up));
}

#[test]
fn is_move() {
    use {Action::*, Direction::*};

    assert!(Move(Up).is_move());
    assert!(Move(Down).is_move());
    assert!(Move(Left).is_move());
    assert!(Move(Right).is_move());
    assert!(!Push(Up).is_move());
    assert!(!Push(Down).is_move());
    assert!(!Push(Left).is_move());
    assert!(!Push(Right).is_move());
}

#[test]
fn is_push() {
    use {Action::*, Direction::*};

    assert!(Push(Up).is_push());
    assert!(Push(Down).is_push());
    assert!(Push(Left).is_push());
    assert!(Push(Right).is_push());
    assert!(!Move(Up).is_push());
    assert!(!Move(Down).is_push());
    assert!(!Move(Left).is_push());
    assert!(!Move(Right).is_push());
}
