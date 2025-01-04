use itertools::Itertools;
use solver::SolverBase;

#[derive(Debug, Clone)]
struct Ingredient {
    name: &'static str,
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

pub struct Solver {
    ingredients: Vec<Ingredient>,
}

impl Solver {
    pub fn new(input: &'static str) -> Self {
        let mut ingredients = Vec::new();
        for line in input.lines() {
            let tokens: Vec<_> = line.split([':', ' ', ',']).collect();
            // Sprinkles: capacity 5, durability -1, flavor 0, texture 0, calories 5
            if let [name, "", "capacity", capacity, "", "durability", durability, "", "flavor", flavor, "", "texture", texture, "", "calories", calories] =
                &tokens[..]
            {
                ingredients.push(Ingredient {
                    name,
                    capacity: capacity.parse().unwrap(),
                    durability: durability.parse().unwrap(),
                    flavor: flavor.parse().unwrap(),
                    texture: texture.parse().unwrap(),
                    calories: calories.parse().unwrap(),
                });
            }
        }
        Solver { ingredients }
    }

    /// Function to calculate the score for a combination of items
    fn score_function(items: &[(Ingredient, usize)]) -> i64 {
        let mut capacity_sum = 0;
        let mut durability_sum = 0;
        let mut flavor_sum = 0;
        let mut texture_sum = 0;
        for (ingredient, teaspoons) in items.iter() {
            capacity_sum += ingredient.capacity * *teaspoons as i64;
            durability_sum += ingredient.durability * *teaspoons as i64;
            flavor_sum += ingredient.flavor * *teaspoons as i64;
            texture_sum += ingredient.texture * *teaspoons as i64;
        }

        capacity_sum.max(0) * durability_sum.max(0) * flavor_sum.max(0) * texture_sum.max(0)
    }

    fn generate_partitions(total: usize, parts: usize) -> Vec<Vec<usize>> {
        if parts == 1 {
            return vec![vec![total]];
        }

        let mut partitions = vec![];
        for i in 0..=total {
            let sub_partitions = Solver::generate_partitions(total - i, parts - 1);
            for mut sub_partition in sub_partitions {
                sub_partition.push(i);
                partitions.push(sub_partition);
            }
        }
        partitions
    }

    fn maximize_score(&self) -> (i64, Vec<(Ingredient, usize)>) {
        // All possible combinations of items (power set)
        let mut max_score = 0;
        let mut best_combination: Vec<(Ingredient, usize)> = vec![];

        // Iterate over all combinations (using power set)
        for combination in (1..=self.ingredients.len())
            .flat_map(|size| self.ingredients.iter().cloned().combinations(size))
        {
            //println!("{:?}", combination);
            let partitions = Solver::generate_partitions(100, combination.len());
            for partition in partitions {
                //println!("{partition:?}");
                let combination_partition = combination
                    .clone()
                    .into_iter()
                    .zip(partition)
                    .collect::<Vec<_>>();
                let score = Solver::score_function(combination_partition.as_slice());

                if score > max_score {
                    max_score = score;
                    best_combination = combination_partition;
                }
            }
        }

        (max_score, best_combination)
    }
}

impl SolverBase for Solver {
    fn solve_part_one(&self) -> String {
        let (max_score, _best_combination) = self.maximize_score();
        //println!("{max_score:?}, {best_combination:?}");
        max_score.to_string()
    }

    fn solve_part_two(&self) -> String {
        "".to_string()
    }

    fn day_number(&self) -> usize {
        15
    }

    fn description(&self) -> &'static str {
        "Cookie igredients"
    }
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solver::new(
            r"Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3",
        )
        .solve_part_one();
        assert_eq!(result, "62842880x");
    }
}

// #[cfg(test)]
// mod part2_tests {
//     use super::*;

//     #[test]
//     fn test_1() {
//         let result = Solver::new("abc").solve_part_two();
//         assert_eq!(result, "0");
//     }
// }
