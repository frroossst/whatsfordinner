use whatsfordinner::grocery;

fn main() 
    {
    // load all recipes into memory
    let mut db = grocery::load_recipes();

    println!("{:#?}", db);

    let mut picked = grocery::pick_recipes(&mut db, 10);
    picked.sort();

    println!("{:#?}", picked);
    println!("picked {} recipes", picked.len());

    }
