use std::collections::HashSet;

/**
 * This class might have been interesting in the past, now seems like a very trivial wrapper over hashset...maybe
 * not really needed anymore.
 */
pub struct AssignmentConstraint {
    allowed_values: HashSet<u32>,
}

impl AssignmentConstraint {
    pub fn new() -> AssignmentConstraint {
        AssignmentConstraint {
            allowed_values: HashSet::new(),
        }
    }    


    pub fn assign_value(&mut self, value: u32) -> bool {
        self.allowed_values.remove(&value)
    }

    pub fn can_assign_value(&self, value: u32) -> bool {
        self.allowed_values.contains(&value)
    }

    pub fn clear(&mut self) {
        self.allowed_values.clear()
    }

    pub fn assign(& mut self, other: &AssignmentConstraint) {
        self.allowed_values.clone_from(&other.allowed_values)
    }

    pub fn intersect_with(& mut self, other: &AssignmentConstraint) {
        self.allowed_values.retain(|value| other.allowed_values.contains(value))
    }

    pub fn get_allowed_values(& self) -> &HashSet<u32>{
        &self.allowed_values
    }
  
    pub fn init_with_values(&mut self, values: &[u32]) {
        self.allowed_values.clear();
        for value in values {
            self.allowed_values.insert(*value);
        }
    }    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interset_only_keeps_common_values() {
        let expected_result = HashSet::from([2,4]);

        let mut this = AssignmentConstraint {
            allowed_values: HashSet::from([1, 2, 3, 4])
        };

        let that = AssignmentConstraint {
            allowed_values: HashSet::from([2,4,5,6])
        };

        this.intersect_with(&that);

        assert_eq!(this.allowed_values, expected_result);
    }
}