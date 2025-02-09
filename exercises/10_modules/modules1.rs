// TODO: Fix the compiler error about calling a private function.
mod sausage_factory {
    use crate::sample;

    // Don't let anybody outside of this module see this!
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    pub fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
        sample::test();
    }
}

mod sample {
    pub fn test() {
        println!("Just test")
    }
}

fn main() {
    sausage_factory::make_sausage();
}
