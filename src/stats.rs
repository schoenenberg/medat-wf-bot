#[derive(Default,Copy,Clone)]
pub(crate) struct Stats {
    correct_answered: u32,
    total_answered: u32,
}

impl Stats {
    pub(crate) fn add_correct(&mut self) {
        self.correct_answered += 1;
        self.total_answered += 1;
    }

    pub(crate) fn add_wrong(&mut self) {
        self.total_answered += 1;
    }

    pub(crate) fn reset(&mut self) {
        self.correct_answered = 0;
        self.total_answered = 0;
    }

    pub(crate) fn stats(&self) -> String {
        let percentage = if self.total_answered == 0 {
            100.0
        } else {
            self.correct_answered as f32 / self.total_answered as f32 * 100.0
        };
        format!("Richtig beantwortet: {}\nInsgesamt beantwortet: {}\nProzentsatz: {:.2}%",
                self.correct_answered,
                self.total_answered,
                percentage
        )
    }
}