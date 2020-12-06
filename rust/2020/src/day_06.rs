use itertools::{join, Itertools};
use std::collections::HashSet;

// Day 6 - Custom Customs
//
// This is the alternate, slower version

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<Vec<String>> {
    let data: Vec<&str> = input.lines().collect();

    data.iter()
        .group_by(|&&l| l.is_empty())
        .into_iter()
        .map(|(_, g)| join(g, " "))
        .filter(|l| !l.is_empty())
        .map(|l| l.split_whitespace().map(|l| l.to_string()).collect())
        .collect()
}

/* Part One
 * As your flight approaches the regional airport where you'll switch to a much larger plane,
 * customs declaration forms are distributed to the passengers.
 *
 * The form asks a series of 26 yes-or-no questions marked a through z.
 * All you need to do is identify the questions for which anyone in your group answers "yes".
 * Since your group is just you, this doesn't take very long.
 *
 * However, the person sitting next to you seems to be experiencing
 * a language barrier and asks if you can help. For each of the people in their group,
 * you write down the questions for which they answer "yes", one per line. For example:
 *
 * abcx
 * abcy
 * abcz
 *
 * In this group, there are 6 questions to which anyone answered "yes": a, b, c, x, y, and z.
 * (Duplicate answers to the same question don't count extra; each question counts at most once.)
 *
 * Another group asks for your help, then another, and eventually you've collected answers from
 * every group on the plane (your puzzle input). Each group's answers are separated by a blank line,
 * and within each group, each person's answers are on a single line. For example:
 *
 * abc
 *
 * a
 * b
 * c
 *
 * ab
 * ac
 *
 * a
 * a
 * a
 * a
 *
 * b
 *
 * This list represents answers from five groups:
 *
 * The first group contains one person who answered "yes" to 3 questions: a, b, and c.
 * The second group contains three people; combined, they answered "yes" to 3 questions: a, b, and c.
 * The third group contains two people; combined, they answered "yes" to 3 questions: a, b, and c.
 * The fourth group contains four people; combined, they answered "yes" to only 1 question, a.
 * The last group contains one person who answered "yes" to only 1 question, b.
 * In this example, the sum of these counts is 3 + 3 + 3 + 1 + 1 = 11.
 *
 * For each group, count the number of questions to which anyone answered "yes". What is the sum of those counts?
 */
///your puzzle answer was.
/// ```
/// use advent_of_code_2020::day_06::*;
/// let input = include_str!("../input/2020/day6.txt");
/// assert_eq!(solve_part_01(&input_generator(input)), 6778);
/// ```
#[aoc(day6, part1)]
pub fn solve_part_01(input: &[Vec<String>]) -> usize {
    input
        .iter()
        .map(|group| join(group, "").chars().collect::<HashSet<_>>().len())
        .sum()
}

/* Part Two
 * As you finish the last group's customs declaration, you notice that you misread one word in the instructions:
 *
 * You don't need to identify the questions to which anyone answered "yes";
 * you need to identify the questions to which everyone answered "yes"!
 *
 * Using the same example as above:
 *
 * abc
 *
 * a
 * b
 * c
 *
 * ab
 * ac
 *
 * a
 * a
 * a
 * a
 *
 * b
 * This list represents answers from five groups:
 *
 * In the first group, everyone (all 1 person) answered "yes" to 3 questions: a, b, and c.
 * In the second group, there is no question to which everyone answered "yes".
 * In the third group, everyone answered yes to only 1 question, a. Since some people did not answer "yes" to b or c, they don't count.
 * In the fourth group, everyone answered yes to only 1 question, a.
 * In the fifth group, everyone (all 1 person) answered "yes" to 1 question, b.
 * In this example, the sum of these counts is 3 + 0 + 1 + 1 + 1 = 6.
 *
 * For each group, count the number of questions to which everyone answered "yes". What is the sum of those counts?
 */
///your puzzle answer was.
/// ```
/// use advent_of_code_2020::day_06::*;
/// let input = include_str!("../input/2020/day6.txt");
/// assert_eq!(solve_part_02(&input_generator(input)), 3406);
/// ```
#[aoc(day6, part2)]
pub fn solve_part_02(input: &[Vec<String>]) -> usize {
    input
        .iter()
        .map(|group| {
            join(group, "")
                .chars()
                .sorted()
                .group_by(|&c| c)
                .into_iter()
                .map(|(_, g)| join(g, ""))
                .fold(0, |acc, answers| match (answers.len(), group.len()) {
                    (a, g) if a == g => acc + 1,
                    _ => acc,
                })
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test example data on part 1
    #[test]
    fn sample_01() {
        let data = "
abc

a
b
c

ab
ac

a
a
a
a

b
";

        assert_eq!(solve_part_01(&input_generator(data)), 11)
    }
    /// Test example data on part 2
    #[test]
    fn sample_02() {
        let data = "
abc

a
b
c

ab
ac

a
a
a
a

b
";

        assert_eq!(solve_part_02(&input_generator(data)), 6)
    }
}
