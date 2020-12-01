// https://aoc.infi.nl/
struct Octagon {
    sidelength: usize,
    area: usize,
    perimeter: usize,
}

fn get_shape_of_octagon(sidelength: usize) -> Octagon {
    if sidelength == 1 {
        // edge case, cannot be bothered
        return Octagon {
            sidelength: 1,
            area: 5,
            perimeter: 8,
        };
    }
    let mut area = sidelength; // start with bottom row
    let mut perimeter = sidelength + 2; // start with bottom and first side edges
    let mut current_width = sidelength;
    let max_width = sidelength + (sidelength) * 2; // start with sidelength, add 2 until middle is reached, which is after 'sidelength' steps

    while current_width != max_width {
        current_width += 2;
        area += current_width;
        perimeter += 2;
    }

    area *= 2;
    area += (sidelength - 2) * max_width;

    perimeter *= 2;
    perimeter += (sidelength - 2) * 2;

    Octagon {
        sidelength: sidelength,
        area: area,
        perimeter: perimeter,
    }
}

fn optimal_octagon_shape(min_content: usize) -> Octagon {
    let mut i: usize = 0;
    loop {
        i += 1;
        let octagon = get_shape_of_octagon(i);
        if octagon.area > min_content {
            return octagon;
        }
    }
}

pub fn main() {
    // PART 1 : 1581
    let population: usize = 17_491_446;
    println!("PART 1: {}", optimal_octagon_shape(population).sidelength);

    // PART 2 : 537816
    let continents: Vec<usize> = vec![
        4_541_690_743,
        1_340_880_189,
        747_680_368,
        430_827_677,
        368_940_588,
        42_734_618,
    ];
    let cloth_required: usize = continents
        .iter()
        .map(|&pop| optimal_octagon_shape(pop).perimeter)
        .sum();
    println!("PART 2: {}", cloth_required);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_area_1() {
        assert_eq!(get_shape_of_octagon(1).area, 5);
    }

    #[test]
    fn test_example_area_2() {
        assert_eq!(get_shape_of_octagon(2).area, 24);
    }

    #[test]
    fn test_example_area_3() {
        assert_eq!(get_shape_of_octagon(3).area, 57);
    }

    #[test]
    fn test_example_area_4() {
        assert_eq!(get_shape_of_octagon(4).area, 104);
    }

    #[test]
    fn test_example_area_10() {
        assert_eq!(get_shape_of_octagon(10).area, 680);
    }

    #[test]
    fn test_example_area_25() {
        assert_eq!(get_shape_of_octagon(25).area, 4_325);
    }

    #[test]
    fn test_example_perimeter_1() {
        assert_eq!(get_shape_of_octagon(1).perimeter, 8);
    }

    #[test]
    fn test_example_perimeter_2() {
        assert_eq!(get_shape_of_octagon(2).perimeter, 16);
    }

    #[test]
    fn test_example_perimeter_3() {
        assert_eq!(get_shape_of_octagon(3).perimeter, 24);
    }

    #[test]
    fn test_example_perimeter_4() {
        assert_eq!(get_shape_of_octagon(4).perimeter, 32);
    }

    #[test]
    fn test_example_perimeter_25() {
        assert_eq!(get_shape_of_octagon(25).perimeter, 200);
    }
}
