#[derive(Debug, Clone)]
pub struct NoveltyItem {
    added: bool,
    generation: u32,
    id: u32,
    fitness: f64,
    novelty: f64,
    age: f64,
}

impl NoveltyItem {
    pub fn new(added: bool, generation: u32, id: u32) -> NoveltyItem {
        NoveltyItem {
            added,
            generation,
            id,
            fitness: 0.0,
            novelty: 0.0,
            age: 0.0,
        }
    }
}

/*pub struct ItemsDistance<'s> {
    distance: f64,
    from: &'s NoveltyItem,
    to: &'s NoveltyItem,
}

pub struct ItemDistancesList<'s> {
    items: Vec<ItemsDistance<'s>>
}

impl ItemDistancesList<'_> {
    pub fn new() -> ItemDistancesList {
        ItemDistancesList {
            items: vec![]
        }
    }
}*/

#[derive(Debug, Clone)]
pub struct NoveltyItemsByFitness {
    items: Vec<NoveltyItem>,
}

impl NoveltyItemsByFitness {
    pub fn new() -> NoveltyItemsByFitness {
        NoveltyItemsByFitness { items: vec![] }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn swap(&mut self, i: usize, j: usize) {
        let copy = self.items[i].clone();
        self.items[i] = self.items[j].clone();

        self.items[j] = copy;
    }

    pub fn less(&self, i: usize, j: usize) -> bool {
        if self.items[i].fitness < self.items[j].fitness {
            return true;
        } else if self.items[i].fitness == self.items[j].fitness {
            if self.items[i].novelty < self.items[j].novelty {
                return true; // less novel is less
            }
        }

        false
    }
}
