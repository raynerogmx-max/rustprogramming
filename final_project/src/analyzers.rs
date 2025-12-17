use crate::types::FileStats;

pub trait Analyzer: Send + Sync {
    fn analyze(&self, content: &str, stats: &mut FileStats);
}

pub struct WordCount;
pub struct LineCount;
pub struct CharFrequency;

impl Analyzer for WordCount {
    fn analyze(&self, content: &str, stats: &mut FileStats) {
        stats.word_count = content.split_whitespace().count();
    }
}

impl Analyzer for LineCount {
    fn analyze(&self, content: &str, stats: &mut FileStats) {
        stats.line_count = content.lines().count();
    }
}

impl Analyzer for CharFrequency {
    fn analyze(&self, content: &str, stats: &mut FileStats) {
        for c in content.chars() {
            *stats.char_frequencies.entry(c).or_insert(0) += 1;
        }
    }
}
