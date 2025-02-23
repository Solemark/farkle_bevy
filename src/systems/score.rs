use crate::resources::{
    dice::SelectedDice,
    game::{Score, ScoreUI},
};
use bevy::{prelude::*, utils::HashMap};

use super::types::DBQuery;

pub fn display_score(score: Res<Score>, mut score_query: Query<&mut Text, With<ScoreUI>>) {
    let mut text = score_query.single_mut();
    text.0 = format!("{}/10000", score.0);
}

pub fn scoring_system(
    selected_dice: Res<SelectedDice>,
    mut score: ResMut<Score>,
    mut button_query: Query<(Entity, &Children), DBQuery>,
    text_query: Query<&mut Text>,
) {
    let mut dice = Vec::new();
    for (e, c) in &mut button_query {
        if selected_dice.0.contains(&e) {
            let d = text_query.get(c[0]).unwrap();
            dice.push(d.0.parse().unwrap_or_default());
        }
    }
    dice.sort();
    score.0 = calc_score(&dice);
}

fn calc_score(dice: &Vec<u8>) -> usize {
    let mut d: HashMap<u8, u8> = HashMap::new();
    for i in dice {
        *d.entry(*i).or_default() += 1;
    }
    // only retain, 1's, 5's or at least 3 of a kind
    d.retain(|k, v| *k == 1 || *k == 5 || *v >= 3);
    d.shrink_to_fit();

    let mut score = 0;
    // Six of a Kind
    for i in d
        .clone()
        .into_iter()
        .filter(|i| i.1 == 6)
        .collect::<Vec<(u8, u8)>>()
    {
        score += 3000;
        d.remove(&i.0);
    }
    // Two Triples
    let j = d
        .clone()
        .into_iter()
        .filter(|i| i.1 == 3)
        .collect::<Vec<(u8, u8)>>();
    if j.len() == 2 {
        score += 2500;
        for i in j {
            d.remove(&i.0);
        }
    }
    // Five of a Kind
    for i in d
        .clone()
        .into_iter()
        .filter(|i| i.1 == 5)
        .collect::<Vec<(u8, u8)>>()
    {
        score += 2000;
        d.remove(&i.0);
    }
    // Three Pairs
    let j = d
        .clone()
        .into_iter()
        .filter(|i| i.1 == 2)
        .collect::<Vec<(u8, u8)>>();
    if j.len() == 3 {
        score += 1500;
        for i in j {
            d.remove(&i.0);
        }
    }
    // Four of a Kind
    for i in d
        .clone()
        .into_iter()
        .filter(|i| i.1 == 4)
        .collect::<Vec<(u8, u8)>>()
    {
        score += 1000;
        d.remove(&i.0);
    }
    // Three of a Kind
    for i in d
        .clone()
        .into_iter()
        .filter(|i| i.1 == 3)
        .collect::<Vec<(u8, u8)>>()
    {
        score += if i.0 == 1 { 1000 } else { i.0 as usize * 100 };
        d.remove(&i.0);
    }
    // Single Ones
    for i in d
        .clone()
        .into_iter()
        .filter(|i| i.0 == 1)
        .collect::<Vec<(u8, u8)>>()
    {
        score += 100 * i.1 as usize;
        d.remove(&i.0);
    }
    // Single Fives
    for i in d
        .clone()
        .into_iter()
        .filter(|i| i.0 == 5)
        .collect::<Vec<(u8, u8)>>()
    {
        score += 50 * i.1 as usize;
        d.remove(&i.0);
    }
    score
}
