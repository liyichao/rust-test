#[macro_export]
macro_rules! m1 { () => ((println!("m1"))) }

// visible here: m1

pub mod foo {
    // visible here: m1

    #[macro_export]
    macro_rules! m2 { () => (println!("m2")) }

    pub fn test2() {
        m2!();
    }
    // visible here: m1, m2
}

pub fn test3() {
    m1!();
}
// visible here: m1

macro_rules! m3 { () => (()) }

// visible here: m1, m3

#[macro_use]
mod bar {
    // visible here: m1, m3

    macro_rules! m4 { () => (()) }

    // visible here: m1, m3, m4
}
