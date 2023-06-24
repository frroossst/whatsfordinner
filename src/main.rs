use whatsfordinner::grocery;

fn main() {
    // load all recipes into memory
    let mut db = grocery::load_recipes();

    let mut picked = grocery::pick_recipes(&mut db, 10);
    picked.sort();

    println!("{:#?}", picked);
    println!("picked {}/{} recipes", picked.len(), db.len());

    for i in picked.iter().map(|x| x.ingredients.iter()).flatten() {
        println!("{:?}", i);
    }
}
