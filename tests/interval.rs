use babbit::interval::Interval;

#[test]
fn interval_from() {
    let some_interval = Interval::from(3, 2);

    assert_eq!(some_interval.interval, 11);
}

#[test]
fn interval_class() {
    let some_interval = Interval::from(3, 2);

    assert_eq!(some_interval.class(), 1);
}

#[test]
fn interval_peq() {
    let some_interval = Interval::from(3, 2);
    let another_interval = Interval::from(2, 3);

    assert_eq!(some_interval, another_interval);
}
