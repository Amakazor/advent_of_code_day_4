#[cfg(test)]
mod tests {
    use std::fs::read_to_string;
    use crate::{convert_assignment_into_section, convert_line_into_sections, count_fully_overlapping_pairs, count_overlapping_pairs, Section};

    #[test]
    fn test_section_contains() {
        let a = Section {
            start: 1,
            end: 9,
        };

        let b = Section {
            start: 3,
            end: 7,
        };

        assert!( a.contains(&b));
        assert!(!b.contains(&a));
    }

    #[test]
    fn test_is_contained_by() {
        let a = Section {
            start: 1,
            end: 9,
        };

        let b = Section {
            start: 3,
            end: 7,
        };

        assert!(!a.is_contained_by(&b));
        assert!( b.is_contained_by(&a));
    }

    #[test]
    fn test_fully_overlaps_with() {
        let a = Section {
            start: 1,
            end: 9,
        };

        let b = Section {
            start: 3,
            end: 7,
        };

        let c = Section {
            start: 8,
            end: 9,
        };

        assert!( a.fully_overlaps_with(&b));
        assert!( b.fully_overlaps_with(&a));
        assert!( a.fully_overlaps_with(&c));
        assert!( c.fully_overlaps_with(&a));
        assert!(!b.fully_overlaps_with(&c));
        assert!(!c.fully_overlaps_with(&b));
    }

    #[test]
    fn test_overlaps() {
        let a = Section {
            start: 1,
            end: 9,
        };

        let b = Section {
            start: 3,
            end: 20,
        };

        let c = Section {
            start: 10,
            end: 21,
        };

        assert!( a.overlaps(&b));
        assert!( b.overlaps(&a));
        assert!(!a.overlaps(&c));
        assert!(!c.overlaps(&a));
        assert!( b.overlaps(&c));
        assert!( c.overlaps(&b));
    }

    #[test]
    fn test_convert_assignment_into_section() {
        assert_eq!(convert_assignment_into_section("1-3"), Section { start: 1, end: 3 });
        assert_eq!(convert_assignment_into_section("7-16"), Section { start: 7, end: 16 });
    }

    #[test]
    fn test_convert_line_into_sections() {
        assert_eq!(convert_line_into_sections("5-15,6-14"), (Section { start: 5, end: 15 }, Section { start: 6, end: 14 }));
    }

    #[test]
    fn test_count_fully_overlapping_pairs() {
        let data = read_to_string("test-data.txt").unwrap();
        let result = count_fully_overlapping_pairs(data.split_ascii_whitespace().map(convert_line_into_sections).collect::<Vec<_>>());
        assert_eq!(result, 2);
    }

    #[test]
    fn test_count_overlapping_pairs() {
        let data = read_to_string("test-data.txt").unwrap();
        let result = count_overlapping_pairs(data.split_ascii_whitespace().map(convert_line_into_sections).collect::<Vec<_>>());
        assert_eq!(result, 4);
    }
}