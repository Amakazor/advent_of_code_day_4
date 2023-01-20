use std::fs::read_to_string;

mod test;

#[derive(Eq, PartialEq, Debug)]
struct Section {
    start: usize,
    end: usize,
}

impl Section {
    fn contains(&self, other: &Section) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn is_contained_by(&self, other: &Section) -> bool {
        other.contains(self)
    }

    fn fully_overlaps_with(&self, other: &Section) -> bool {
       self.contains(other) || self.is_contained_by(other)
    }

    fn overlaps(&self, other: &Section) -> bool {
        self.fully_overlaps_with(other)
            || (other.start..other.end).contains(&self.start)
            || (other.start..other.end).contains(&self.end)
            || (self.start..self.end).contains(&other.start)
            || (self.start..self.end).contains(&other.end)
    }
}

fn convert_line_into_sections(line: &str) -> (Section, Section) {
    let parts = line.split(",").collect::<Vec<_>>();
    (convert_assignment_into_section(parts[0]), convert_assignment_into_section(parts[1]))
}

fn convert_assignment_into_section(assignment: &str) -> Section {
    let parts = assignment.split("-").map(|number| number.parse::<usize>().unwrap()).collect::<Vec<_>>();
    Section { start: parts[0], end: parts[1], }
}

fn count_fully_overlapping_pairs(pairs: Vec<(Section, Section)>) -> usize {
    pairs.iter().map(|pair| (*pair).0.fully_overlaps_with(&(*pair).1) as usize).sum()
}

fn count_overlapping_pairs(pairs: Vec<(Section, Section)>) -> usize {
    pairs.iter().map(|pair| (*pair).0.overlaps(&(*pair).1) as usize).sum()
}

fn main() {
    let data = read_to_string("final-data.txt").unwrap();
    let fully_overlapping = count_fully_overlapping_pairs(data.split_ascii_whitespace().map(convert_line_into_sections).collect::<Vec<_>>());
    let overlapping = count_overlapping_pairs(data.split_ascii_whitespace().map(convert_line_into_sections).collect::<Vec<_>>());
    println!("PART 1: {}", fully_overlapping);
    println!("PART 2: {}", overlapping);
}
