use std::collections::{HashMap, HashSet};

fn load_demo() -> String {
    r#"mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)"#.to_string()
}

fn load_data() -> String {
    std::fs::read_to_string("./input.txt").unwrap()
}

#[derive(Debug)]
struct Food {
    ingredients: Vec<String>,
    allergens: Vec<String>,    
}

impl Food {
    fn from_line(line: &str) -> Food {
        if line.contains("(contains") {
            let parts: Vec<&str> = line.split("(contains").collect();
            let ingredients: Vec<String> = parts[0]
                .split(" ")
                .map(|w| w.to_string())
                .filter(|w| !w.is_empty())
                .collect();
            let allergens: Vec<String> = parts[1]
                .split(",")
                .map(|w| w.trim().to_string().replace(")", ""))
                .filter(|w| !w.is_empty())
                .collect();
            Food{ingredients, allergens}
        } else {
            let ingredients: Vec<String> = line
                .split(" ")
                .map(|w| w.to_string())
                .filter(|w| !w.is_empty())
                .collect();
            Food{ingredients, allergens: vec![]}
        }
    }
}


#[derive(Debug)]
struct Allergen {
    name: String,
    foods: Vec<usize>,
    candidates: Vec<String>,
}

impl Allergen {
    fn list_from_food(food: &Food, id_food: &usize) -> Vec<Allergen> {
        let mut all = vec![];
        for a in food.allergens.iter() {
            all.push(Allergen{
                name: a.clone(),
                foods: vec![id_food.clone()],
                candidates: food.ingredients.clone(),
            });
        }
        all
    }

    fn trim_candidates(&mut self, food: &Food, id_food: &usize) {
        self.foods.push(id_food.clone());
        for idx in (0..self.candidates.len()).rev() {
            if !food.ingredients.contains(&self.candidates[idx]) {
                self.candidates.remove(idx);
            }
        }
    }

    fn purge(&mut self, word: &String) {
        for idx in (0..self.candidates.len()).rev() {
            if self.candidates[idx] == *word {
                self.candidates.remove(idx);
            }
        }
    }

    fn determined(&self) -> bool {
        self.candidates.len() == 1
    }

    fn all_word(&self) -> String {
        self.candidates[0].clone()
    }
}

struct Translation {
    foods: Vec<Food>,
    words: HashMap<String, Vec<usize>>,
    allergens: HashMap<String, Allergen>,    
}

impl Translation {
    fn from_data(data: String) -> Translation {
        let mut foods: Vec<Food> = vec![];
        let mut allergens: HashMap<String, Allergen> = HashMap::new();
        let mut words: HashMap<String, Vec<usize>> = HashMap::new();
        for (id_food, line) in data.lines().enumerate() {
            let f = Food::from_line(line);
            for word in f.ingredients.iter() {
                if words.contains_key(word) {
                    words.get_mut(word).unwrap().push(id_food.clone());
                } else {
                    words.insert(word.clone(), vec![id_food.clone()]);
                }
            }
            for all in Allergen::list_from_food(&f, &id_food) {
                if allergens.contains_key(&all.name) {
                    allergens.get_mut(&all.name).unwrap().trim_candidates(&f, &id_food);
                } else {
                    allergens.insert(all.name.clone(), all);
                }
            }

            foods.push(f);
        }
        Translation{foods, allergens, words}
    }

    fn resolve_allergens(&mut self) {
        let mut determined: HashSet<String> = HashSet::new();
        let keys: Vec<String> = self.allergens.keys().map(|k| k.clone()).collect();
        let mut trim_cycle = 0;
        loop {
            for all_name in keys.iter() {
                if determined.contains(all_name) { continue; }
                if self.allergens[all_name].determined() {
                    let all_word = self.allergens[all_name].all_word();
                    for food in self.words[&all_word].iter() {
                        for other_all_name in self.foods[*food].allergens.iter() {
                            if other_all_name == all_name || self.allergens[other_all_name].determined() { continue; }
                            self.allergens.get_mut(other_all_name).unwrap().purge(&all_word);
                        }
                    }
                    determined.insert(all_name.clone());
                }
            }
            trim_cycle += 1;
            println!("{}: {} determined of {}", trim_cycle, determined.len(), keys.len());
            if determined.len() == keys.len() {
                break;
            }
        }
    }

    fn non_allergenics(&self) -> HashSet<String> {
        let mut nons = HashSet::new();
        for word in self.words.keys() {
            let mut found = false;
            for (_, all) in self.allergens.iter() {
                if *word == all.all_word() {
                    found = true;
                    break;
                }
            }
            if !found {
                nons.insert(word.clone());
            }
        }
        nons
    }

    fn count_non_allergenics(&self) -> usize {
        let nons = self.non_allergenics();
        println!("Non-allergentics: {:?}", nons);
        self
            .foods
            .iter()
            .fold(
            0,
            |acc, f| acc + f
                .ingredients
                .iter()
                .filter(|w| nons.contains(*w))
                .count()
            )
    }

    fn alphabetical_alergens(&self) -> Vec<String> {
        let mut keys: Vec<String> = self.allergens.keys().map(|k| k.clone()).collect();
        keys.sort();
        keys
            .iter()
            .map(|k| self.allergens[k].all_word())
            .collect()
    }
}

fn main() {
    let is_demo = false;
    let data = match is_demo { true => load_demo(), false => load_data()};
    let mut trans = Translation::from_data(data);
    trans.resolve_allergens();
    let count = trans.count_non_allergenics();
    println!("Part 1: {}", count);
    let words = trans.alphabetical_alergens();
    println!("Part 2: {}", words.join(","));
}
