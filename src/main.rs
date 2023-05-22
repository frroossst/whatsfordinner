use clap::Parser;
use ::recipes::Recipe;

#[derive(Parser, Debug)]
struct Args
    {
    }

fn main() 
    {
    let r = Recipe::load("recipes/cookies.toml");
    }
