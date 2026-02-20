mod recipes {
    pub mod margherita {
        pub fn prepare() {
            println!("Preparing Margherita Pizza...");
        }
        pub fn bake() {
            println!("Baking Margherita Pizza...");
        }
        pub fn serve() {
            println!("Serving Margherita Pizza...");
        }
    }
}

pub mod api;

/**
 * 🍕 Eat at the restaurant function
 * This function prepares and serves a Margherita pizza
 * @return {void}
 */
pub fn eat_at_restaurant() {
    println!("Welcome to the restaurant! 🍴");
    recipes::margherita::prepare();
    recipes::margherita::bake();
    recipes::margherita::serve();
}
