use std::fmt;

pub enum Action {
    // Install,
    // Delete,
    Act(Option<String>)
}

impl fmt::Debug for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self).expect("TODO: panic message");
        return Ok(());
    }
}

// impl Action {
//
// }

// impl Clone for Action {
//     fn clone(&self) -> Self {
//         todo!()
//     }
//
//     fn clone_from(&mut self, source: &Self) {
//         todo!()
//     }
// }