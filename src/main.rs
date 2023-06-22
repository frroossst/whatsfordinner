use whatsfordinner::grocery;

fn main() 
    {
    // load all recipes into memory
    let mut db = grocery::load_recipes();

    db.sort();

    println!("{:#?}", db);
    }
