use rand::prelude::*;

pub struct Fact<T> {
    phantom: std::marker::PhantomData<T>,
}

impl<T> Fact<T> {
    pub fn new() -> Self {
        Fact {
            phantom: std::marker::PhantomData,
        }
    }

}

impl<T> Fact<Vec<T>> {
    fn name() -> &'static str {
        "Vec"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fact_for_string() {
        let fact_string = Fact::<String>::new();
    }
    #[test]
    fn test_name_for_vec() {
        
     
let name = Fact::<Vec<i32>>::name();
        assert_eq!(name, "Vec");
    }
 
}
