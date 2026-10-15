#[derive(Clone, Debug, Default)]
pub struct Delta<T: Default + Copy + std::ops::Sub<Output = T>> {
    old: T,
    new: T
}

impl<T> Delta<T>
where
    T: Copy + std::ops::Sub<Output = T> + Default
{
    /// Starts with old == new so the first delta is zero rather than the full value
    pub fn new(initial: T) -> Self {
        Self { old: initial, new: initial }
    }

    pub fn push(&mut self, new: T) {
        self.old = self.new;
        self.new = new;
    }

    pub fn delta(&self) -> T {
        self.new - self.old
    }
}