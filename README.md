# `whatsfordinner`

This is to help me with grocery and meal prepping without having to go through some serious decision fatigue.

Find 30 recipes that I like, are fairly easy to cook, and are affordable, have a pretty basic algorithm pick out recipes for week-ish and generate a grocery list

## How it works

Every dish has a `genre` a week is made up of 
- pasta
- noodles
- sandwich
- rice
- wrap
- oven baked

## Schema

In a folder called `database` each file is a recipe in itself like `caesar_salad.recipe`, this follows the struct `Recipe` as defined in `recipes.rs`. 

`pantry.store` keeps track of what is there in the pantry and plans accordingly. 


