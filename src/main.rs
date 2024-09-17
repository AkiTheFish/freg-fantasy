use rand::*;
use core::time;
use std::{io, thread};

#[allow(non_camel_case_types)]
#[derive(Debug)]
enum characters {
    frog,
    frob,
    henry,
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
enum enemies {
    guy,
    doge,
    dooge,
    freiren,
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
struct character {
    name: String,
    ap: i32,
    hp: i32,
    desc: String,
}
impl character {
    fn new(name: &str, ap: i32, hp: i32, desc: &str) -> Self {
        Self {
            name: name.to_owned(),
            ap,
            hp,
            desc: desc.to_owned(),
        }
    }
    fn display(&self) {
        println!(
            "{}:  health: {} | attack power: {} | description: {}",
            self.name, self.hp, self.ap, self.desc
        )
    }
}
fn main() {
    let mut character: character;
    let mut enemy: character;

    let freg = character::new("freg", 18, 60, "very freg");
    let frob = character::new("frob", 12, 120, "gyatt");
    let henry = character::new("henry", 15, 80, "hacker guy yes");

    println!("choose your freg:");
    freg.display();
    frob.display();
    henry.display();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    character = match input.trim() {
        "freg" => freg,
        "frob" => frob,
        "henry" => henry,
        _ => {
            println!("character not found. choose an actual character");
            return
        },
    };
    println!("u chose '{:?}' as a character", character);
    enemy = spawn_enemy();
    fight(character, enemy);
}

fn spawn_enemy() -> character {
    match rand::thread_rng().gen_range(1..=5) {
        1 => {
            println!("a wild doge appeared");
            character::new("doge", 100, 60, "dogus")
        },
        2 => {
            println!("a wild dooge appeared");
            character::new("dooge", 100, 60, "doogus")
        }
        3 => {
            println!("a wild freiren appeared");
            character::new("freiren", 100, 20, "smashington")
        }
        4..=5 => {
            println!("ur safe bluddington");
            panic!("freg");
        }
        _ => panic!("blud"),
    }
}

fn fight(mut player: character, mut enemy: character) {
    loop {
        if enemy.hp <= 0 {
            println!("yuo won against {} with {} health left", enemy.name, player.hp);
            break;
        } else if player.hp <= 0 {
            println!("yuo lost against {} with {} health left", enemy.name, enemy.hp);
            break;
        } else {
            enemy.hp -= player.ap;
            println!("you did {} damage to {}!", player.ap, enemy.name);
            thread::sleep(time::Duration::from_millis(500));
            player.hp -= enemy.ap;
            println!("{} did {} damage to you!", enemy.name, enemy.ap);
            println!("your health: {} \n enemys health: {}", player.hp, enemy.hp);
            thread::sleep(time::Duration::from_millis(500));
        }
    }
}
