pub struct Calculator {
    pub memory: Vec<f32>,
}

impl Calculator {
    pub fn new() -> Calculator {
        Calculator { memory: Vec::new() }
    }

    /// preforms a + b, returns and stores the result in calculators memory
    /// # Example
    /// ```rust
    /// use doc_example::{Calculator};
    ///
    /// let mut calc = Calculator::new();
    ///
    /// assert_eq!(calc.memory, Vec::new());
    ///
    /// let result = calc.add(6.0, 0.7);
    ///
    /// assert_eq!(result, 6.9);
    /// assert_eq!(calc.memory, vec![6.7]);
    /// ```
    pub fn add(&mut self, a: f32, b: f32) -> f32 {
        let result = a + b;

        self.memory.push(result);

        result
    } //
}
