use whatsfordinner::grocery;

fn main() 
    {
    // load all recipes into memory
    let mut db = grocery::load_recipes();

    println!("{:#?}", db);

    db.sort();
    }
