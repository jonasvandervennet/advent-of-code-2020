use crate::util::{print_part_1, print_part_2};
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::time::Instant;

struct Ingredient {
    name: String,
    allergen: String,
}

#[derive(Debug)]
struct Food {
    allergens: Vec<String>,
    ingredients: Vec<String>,
}

fn match_allergens(input: &str, part: usize) -> String {
    let mut foods: Vec<Food> = Vec::new();

    let mut all_allergens = HashSet::new();
    let mut ingredient_usage = HashMap::new();
    
    let mut fixed_allergens: Vec<String> = Vec::new();

    for line in input.lines() {
        let parts = line.split(" (contains ").collect::<Vec<&str>>();
        let ingredients = parts[0]
            .split_whitespace()
            .map(|s| {
                ingredient_usage
                    .entry(s.to_string())
                    .and_modify(|e| *e += 1)
                    .or_insert(1);

                s.to_string()
            })
            .collect::<Vec<String>>();
        let allergens: Vec<String> = parts[1].split(")").collect::<Vec<&str>>()[0]
            .split(", ")
            .map(|allergen| {
                all_allergens.insert(allergen.to_string());
                allergen.to_string()
            })
            .collect();

        foods.push(Food {
            allergens: allergens,
            ingredients: ingredients,
        });
    }

    let mut fixed_ingredients: Vec<Ingredient> = Vec::new();
    while fixed_allergens.len() != all_allergens.len() {
        for allergen in all_allergens.iter() {
            let food_with_allergen = foods
                .iter()
                .filter(|&x| x.allergens.contains(allergen))
                .collect::<Vec<&Food>>();
            let mutual_ingredients = food_with_allergen[0]
                .ingredients
                .iter()
                .filter(|&x| {
                    !&food_with_allergen[1..]
                        .iter()
                        .any(|f| !f.ingredients.contains(x))
                        && !fixed_ingredients.iter().any(|ing| ing.name == *x)
                })
                .collect::<Vec<&String>>();
            if mutual_ingredients.len() == 1 {
                // fixed
                fixed_allergens.push(allergen.to_string());
                fixed_ingredients.push(Ingredient {
                    name: mutual_ingredients[0].to_string(),
                    allergen: allergen.to_string(),
                });
            }
        }
    }

    if part == 1 {
        let mut unused_counter = 0;
        for (ing, usage) in ingredient_usage {
            if !fixed_ingredients.iter().any(|i| i.name == ing) {
                unused_counter += usage;
            }
        }

        return unused_counter.to_string();
    }

    // sort ascending by allergen name
    fixed_ingredients.sort_by(|a, b| a.allergen.cmp(&b.allergen));
    // join ingredient names with a ',' seperator
    fixed_ingredients
        .iter()
        .map(|ing| ing.name.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

pub fn main() {
    let input = read_to_string("inputs/day21.txt").expect("Input not found..");

    // PART 1
    let start = Instant::now();
    let known_answer = "2315";
    let part_1: String = match_allergens(&input, 1);
    let duration = start.elapsed();
    print_part_1(&part_1.to_string(), &known_answer, duration);

    // PART 2
    let start = Instant::now();
    let known_answer = "cfzdnz,htxsjf,ttbrlvd,bbbl,lmds,cbmjz,cmbcm,dvnbh";
    let part_2: String = match_allergens(&input, 2);
    let duration = start.elapsed();
    print_part_2(&part_2.to_string(), &known_answer, duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input: &str = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)\ntrh fvjkl sbzzf mxmxvkd (contains dairy)\nsqjhc fvjkl (contains soy)\nsqjhc mxmxvkd sbzzf (contains fish)";
        let answer: String = match_allergens(&input, 1);
        assert_eq!(answer, "5");
    }
    #[test]
    fn test_example_2() {
        let input: &str = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)\ntrh fvjkl sbzzf mxmxvkd (contains dairy)\nsqjhc fvjkl (contains soy)\nsqjhc mxmxvkd sbzzf (contains fish)";
        let answer: String = match_allergens(&input, 2);
        assert_eq!(answer, "mxmxvkd,sqjhc,fvjkl");
    }
}
