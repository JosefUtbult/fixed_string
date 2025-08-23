use crate::{FixedString, FixedStringRef};

#[test]
fn check_default_paramteters() {
    let mut fixed_string = FixedString::<128>::new();
    fixed_string.assign("Hello World!").unwrap();

    assert_eq!(128, fixed_string.capacity());
    assert_eq!(12, fixed_string.length());
}

#[test]
fn check_format() {
    let fixed_string = FixedString::<128>::format(format_args!("Hello {}", "World!")).unwrap();
    assert_eq!("Hello World!", fixed_string.as_str());
}

#[test]
fn concatinate() {
    let mut fixed_string1 = FixedString::<12>::new();
    let mut fixed_string2 = FixedString::<6>::new();
    fixed_string1.assign("Hello ").unwrap();
    fixed_string2.assign("World!").unwrap();

    fixed_string1.concatinate(&fixed_string2).unwrap();
    assert_eq!("Hello World!", fixed_string1.as_str());
}

#[test]
#[should_panic]
fn double_assign() {
    let mut fixed_string = FixedString::<128>::new();
    fixed_string.assign("Hello World!").unwrap();
    fixed_string.assign("Hello Again!").unwrap();
}

#[test]
#[should_panic]
fn assign_overflow() {
    let mut fixed_string = FixedString::<11>::new();
    fixed_string.assign("Hello World!").unwrap();
}

#[test]
#[should_panic]
fn push_overflow() {
    let mut fixed_string = FixedString::<11>::new();
    fixed_string.assign("Hello ").unwrap();
    fixed_string.push("World!").unwrap();
}

#[test]
#[should_panic]
fn format_overflow() {
    FixedString::<11>::format(format_args!("Hello {}", "World!")).unwrap();
}

#[test]
#[should_panic]
fn concatinate_overflow() {
    let mut fixed_string1 = FixedString::<11>::new();
    let mut fixed_string2 = FixedString::<6>::new();
    fixed_string1.assign("Hello ").unwrap();
    fixed_string2.assign("World!").unwrap();

    fixed_string1.concatinate(&fixed_string2).unwrap()
}

#[test]
fn from_raw() {
    let raw = ['H', 'e', 'l', 'l', 'o', ' ', 'W', 'o', 'r', 'l', 'd', '!'];
    let raw_as_u8 = raw.map(|character| character as u8);
    let fixed_string = FixedString::from_raw(&raw_as_u8).unwrap();
    assert_eq!("Hello World!", fixed_string.as_str());
}

#[test]
fn to_raw() {
    let mut fixed_string = FixedString::<128>::new();
    fixed_string.assign("Hello World!").unwrap();

    let res = fixed_string.raw();
    let target = ['H', 'e', 'l', 'l', 'o', ' ', 'W', 'o', 'r', 'l', 'd', '!'];

    for i in 0..12 {
        assert_eq!(target[i], res[i] as char);
    }
}

#[test]
fn iter() {
    let mut fixed_string = FixedString::<128>::new();
    fixed_string.assign("Hello World!").unwrap();

    let target = [
        'H', 'e', 'l', 'l', 'o', ' ', 'W', 'o', 'r', 'l', 'd', '!', '\0',
    ];
    let mut counter = 0;

    for value in fixed_string.iter() {
        assert_eq!(target[counter], value);
        counter += 1;
    }
}
