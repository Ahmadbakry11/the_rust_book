use crate::shoes::Shoe;

pub fn get_shoes(shoes: Vec<Shoe>, size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|shoe| shoe.size == size).collect()
}

/*
    This method can be implemented using the iter() which produces imuable refs of the main collection
    which is the shoes vector here.
    
    pub fn get_shoes(shoes: Vec<Shoe>, size: u32) -> Vec<Shoe> {
        let shoes_refs = shoes.iter().filter(|shoe| shoe.size == size);
        shoes_refs.map(|s| *s).collect()
    }
*/

