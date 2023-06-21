use whatsfordinner::grocery;

fn main() 
    {
    // load all recipes into memory
    let db = grocery::load_recipes();

    println!("{:#?}", db);
    }
