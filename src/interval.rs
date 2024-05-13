/// An interval between two notes.
#[derive(Debug, Clone, Copy)]
pub struct Interval {
    /// The number of semitones between the two notes.
    pub interval: u8,
}

impl Interval {
    /// Create an interval from two notes.
    ///
    /// Finds the interval between the two notes where `a` is the lower note and `b` is the higher note, and `a` and `b` are both within an octave.
    ///
    /// # Examples
    /// ```rust
    /// use babbit::interval::Interval;
    ///
    /// let some_interval = Interval::from(0, 2); // 2 semitones
    /// let another_interval = Interval::from(3, 2); // 11 semitones
    /// ```
    pub fn from(a: i8, b: i8) -> Interval {
        Interval {
            interval: (b - a).rem_euclid(12) as u8,
        }
    }

    /// Get the interval class.
    ///
    /// Returns the [interval class] of the interval.
    ///
    /// # Examples
    /// ```rust
    /// use babbit::interval::Interval;
    ///
    /// let some_interval = Interval::from(3, 2); // 11 semitones
    /// let interval_class = some_interval.class(); // ic 1
    /// ```
    ///
    /// [interval class]: https://en.wikipedia.org/wiki/Interval_class
    pub fn class(&self) -> u8 {
        if self.interval > 6 {
            6 - (self.interval % 6)
        } else {
            self.interval
        }
    }
}

impl PartialEq for Interval {
    fn eq(&self, other: &Interval) -> bool {
        self.class() == other.class()
    }
}

impl Eq for Interval {}
