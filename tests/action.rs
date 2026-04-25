use soukoban::prelude::*;

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
fn rotate_cw() {
    use {Action::*, Direction::*};

    assert_eq!(Move(Up).rotate_cw(), Move(Right));
    assert_eq!(Move(Right).rotate_cw(), Move(Down));
    assert_eq!(Move(Down).rotate_cw(), Move(Left));
    assert_eq!(Move(Left).rotate_cw(), Move(Up));

    assert_eq!(Push(Up).rotate_cw(), Push(Right));
    assert_eq!(Push(Right).rotate_cw(), Push(Down));
    assert_eq!(Push(Down).rotate_cw(), Push(Left));
    assert_eq!(Push(Left).rotate_cw(), Push(Up));
}

#[test]
fn rotate_ccw() {
    use {Action::*, Direction::*};

    assert_eq!(Move(Up).rotate_ccw(), Move(Left));
    assert_eq!(Move(Right).rotate_ccw(), Move(Up));
    assert_eq!(Move(Down).rotate_ccw(), Move(Right));
    assert_eq!(Move(Left).rotate_ccw(), Move(Down));

    assert_eq!(Push(Up).rotate_ccw(), Push(Left));
    assert_eq!(Push(Right).rotate_ccw(), Push(Up));
    assert_eq!(Push(Down).rotate_ccw(), Push(Right));
    assert_eq!(Push(Left).rotate_ccw(), Push(Down));
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
fn is_shift() {
    use {Action::*, Direction::*};

    assert!(Push(Up).is_shift());
    assert!(Push(Down).is_shift());
    assert!(Push(Left).is_shift());
    assert!(Push(Right).is_shift());

    assert!(!Move(Up).is_shift());
    assert!(!Move(Down).is_shift());
    assert!(!Move(Left).is_shift());
    assert!(!Move(Right).is_shift());
}
