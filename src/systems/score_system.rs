use crate::resources::{
    dice::{Dice, Selected},
    game::{Score, ScoreUI},
};
use bevy::{
    color::palettes::css::{BLACK, BLUE, RED, WHITE},
    prelude::*,
    utils::hashbrown::HashMap,
};

pub fn display_score_system(score: Res<Score>, mut query: Query<&mut Text, With<ScoreUI>>) {
    let mut text = query.single_mut();
    text.0 = format!("{}/{}", score.0, score.1);
}

pub fn farkle_system(
    mut button_query: Query<(&Interaction, &mut BorderColor, &Children), With<Button>>,
    text_query: Query<&Text>,
) {
    for (i, mut border, child) in &mut button_query {
        let text = text_query.get(child[0]).unwrap();
        match *i {
            Interaction::Pressed => {
                if text.0 == "END" {
                    border.0 = BLACK.into();
                } else {
                    border.0 = RED.into();
                }
            }
            Interaction::Hovered => {
                if text.0 == "END" {
                    border.0 = WHITE.into();
                } else {
                    border.0 = RED.into();
                }
            }
            Interaction::None => {
                if text.0 == "END" {
                    border.0 = BLUE.into();
                } else {
                    border.0 = RED.into();
                }
            }
        }
    }
}

pub fn scoring_system(selected: Res<Selected>, dice: Res<Dice>, mut score: ResMut<Score>) {
    let mut d = Vec::new();
    for (e, i) in dice.0.clone() {
        if selected.0.contains(&e) {
            d.push(i);
        }
    }
    score.0 = calc_score(&d);
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
