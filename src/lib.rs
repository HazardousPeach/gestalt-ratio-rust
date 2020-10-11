#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// This test and it's expected output are taken from the
    /// Wikipedia page on gestalt pattern matching.
    fn wikipedia_example() {
        let score = gestalt_ratio("Wikimedia", "Wikimania");
        assert!(score > 0.7777, "{}", score);
        assert!(score < 0.7778, "{}", score);
    }

    #[test]
    /// This test and it's expected output are taken from this stack
    /// overflow post:
    /// https://stackoverflow.com/questions/35517353/how-does-pythons-sequencematcher-work
    fn stack_overflow_example() {
        let s1 = "Ebojfm Mzpm";
        let s2 = "Ebfo ef Mfpo";
        let score1 = gestalt_ratio(s1, s2);
        let score2 = gestalt_ratio(s2, s1);
        assert_eq!(score1, 0.6086956521739131, "{}", score1);
        assert_eq!(score2, 0.5217391304347826, "{}", score2);
    }
}

/// Ratcliff-Obershelp String Matching, otherwise known as Gestalt
/// Pattern Matching. This function computes a similarity score
/// between two strings, based on recursively looking at longest
/// common substrings. It is described in this wikipedia page:
/// https://en.wikipedia.org/wiki/Gestalt_Pattern_Matching
pub fn gestalt_ratio(s1: &str, s2: &str) -> f64 {
    fn longest_common_substring_idxs(s1: &str, s2: &str) -> ((usize, usize), (usize, usize)) {
        let mut max_length = 0;
        let mut ending_index_1 = s1.len();
        let mut ending_index_2 = s2.len();
        let mut lookup = vec![vec![0; s2.len() + 1]; s1.len() + 1];

        for (i, c1) in s1.chars().enumerate() {
            for (j, c2) in s2.chars().enumerate() {
                if c1 == c2 {
                    lookup[i + 1][j + 1] = lookup[i][j] + 1;
                    if lookup[i + 1][j + 1] > max_length {
                        max_length = lookup[i + 1][j + 1];
                        ending_index_1 = i + 1;
                        ending_index_2 = j + 1;
                    }
                }
            }
        }
        (
            (ending_index_1 - max_length, ending_index_1),
            (ending_index_2 - max_length, ending_index_2),
        )
    }
    fn matching_characters(s1: &str, s2: &str) -> usize {
        let ((l1, r1), (l2, r2)) = longest_common_substring_idxs(s1, s2);
        assert_eq!(r1 - l1, r2 - l2);
        if l1 == r1 {
            0
        } else {
            let left_rec = if l1 > 0 && l2 > 0 {
                matching_characters(&s1[..l1], &s2[..l2])
            } else {
                0
            };
            let right_rec = if r1 < s1.len() && r2 < s2.len() {
                matching_characters(&s1[r1..], &s2[r2..])
            } else {
                0
            };
            left_rec + (r1 - l1) + right_rec
        }
    }
    (2.0 * matching_characters(s1, s2) as f64) / ((s1.len() + s2.len()) as f64)
}
