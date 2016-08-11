/* Stones esoteric programming language
 * (c) 2016 Zack Hixon - see LICENSE.txt */

extern crate argparse;

use argparse::{ArgumentParser, StoreTrue, Store};

use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::error::Error;
use std::process;

#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
    Invis
}

fn main() {
    let mut debug = false;
    let mut show_field = false;
    let mut show_stack = false;
    let mut filename: String = "".into();

    {
        let mut args = ArgumentParser::new();
        args.set_description("Run a stones language file");
        args.refer(&mut debug)
            .add_option(&["-d", "--debug"], StoreTrue, "Run debugging");
        args.refer(&mut show_field)
            .add_option(&["-f", "--field"], StoreTrue, "Show field");
        args.refer(&mut show_stack)
            .add_option(&["-s", "--stack"], StoreTrue, "Show stack");
        args.refer(&mut filename)
            .add_argument("file", Store, "File to run")
            .required();
        args.parse_args_or_exit();
    }

    /* open file */
    let path = Path::new(&filename);
    let display = path.display();

    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(_) => {
                println!("File doesn't exist: {}", display);
                std::process::exit(1);
            }
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Ok(_) => {},
        Err(e) => panic!("Couldn't read {}: {}", display, e.description()),
    }

    /* split file into tokens */
    let tokens: Vec<&str> = s.split_whitespace().collect();

    let mut field: Vec<Vec<Color>> = /* so much for 80 columns */
        vec![vec![Color::Blue,  Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Orange,Color::Invis, Color::Invis, Color::Invis, Color::Invis ],
             vec![Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis ],
             vec![Color::Invis, Color::Invis, Color::Red,   Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Green, Color::Invis, Color::Invis ],
             vec![Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis ],
             vec![Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Yellow,Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Purple],
             vec![Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis ]];

    let mut stack: Vec<i64> = vec![];

    /* execute program */
    let mut count = 0;
    let mut current_stone = Color::Invis;
    let mut current_direction = "none";
    let mut current_number = "none";
    let colors: Vec<&str> = vec!["red", "orange", "yellow", "green", "blue", "purple"];

    while count < tokens.len() {
        if show_field {
            for row in &field {
                for color in row {
                    match color {
                        &Color::Red => print!("{:?}... ", color),
                        &Color::Orange => print!("{:?} ", color),
                        &Color::Yellow => print!("{:?} ", color),
                        &Color::Green => print!("{:?}. ", color),
                        &Color::Blue => print!("{:?}.. ", color),
                        &Color::Purple => print!("{:?} ", color),
                        &Color::Invis => print!("...... "), /* oh. */
                    }
                }
                println!("");
            }
        }

        match tokens[count] {
            /* colors */
            "red" => {
                    current_stone = Color::Red;
                    current_direction = "none";
                    current_number = "none";
                    if count != 0 {
                        if is_color(tokens[count - 1]) {
                            println!("Unexpected color!");
                        }
                    }
                },
            "orange" => {
                    current_stone = Color::Orange;
                    current_direction = "none";
                    current_number = "none";
                    if count != 0 {
                        if is_color(tokens[count - 1]) {
                            println!("Unexpected color!");
                        }
                    }
                },
            "yellow" => {
                    current_stone = Color::Yellow;
                    current_direction = "none";
                    current_number = "none";
                    if count != 0 {
                        if is_color(tokens[count - 1]) {
                            println!("Unexpected color!");
                        }
                    }
                },
            "green" => {
                    current_stone = Color::Green;
                    current_direction = "none";
                    current_number = "none";
                    if count != 0 {
                        if is_color(tokens[count - 1]) {
                            println!("Unexpected color!");
                        }
                    }
                },
            "blue" => {
                    current_stone = Color::Blue;
                    current_direction = "none";
                    current_number = "none";
                    if count != 0 {
                        if is_color(tokens[count - 1]) {
                            println!("Unexpected color!");
                        }
                    }
                },
            "purple" => {
                    current_stone = Color::Purple;
                    current_direction = "none";
                    current_number = "none";
                    if count != 0 {
                        if is_color(tokens[count - 1]) {
                            println!("Unexpected color!");
                        }
                    }
                },

            /* directions */
            "up" => {
                    current_direction = "up";
                    if current_stone != Color::Red {
                        match current_stone {
                            //Color::Orange => {},
                            Color::Yellow => {
                                    let tmp1 = stack.pop().unwrap();
                                    let tmp2 = stack.pop().unwrap();
                                    stack.push(tmp1 * tmp2);
                                },
                            //Color::Green => {},
                            Color::Blue => {
                                    print!("{}\n", stack.pop().unwrap());
                                },
                            //Color::Purple => {},
                            _ => {}
                        }

                        //field[]
                        /* oh boy! I just found a shitty thing. fuck. I need
                         * to figure out how to move the stones around. */
                    }
                },
            "down" => {
                    current_direction = "down";
                    if current_stone != Color::Red {
                        match current_stone {
                            //Color::Orange => {},
                            Color::Yellow => {
                                    let tmp1 = stack.pop().unwrap();
                                    let tmp2 = stack.pop().unwrap();
                                    stack.push(tmp1 + tmp2);
                                },
                            //Color::Green => {},
                            //Color::Blue => {
                            //        let tmp = stack.pop().unwrap();
                            //        stack.push(&tmp); /* might cause problemos... */
                            //        stack.push(&tmp);
                            //    },
                            //Color::Purple => {},
                            _ => {}
                        }
                    }
                },
            "left" => {
                    current_direction = "left";
                    if current_stone != Color::Red {
                        match current_stone {
                            //Color::Orange => {},
                            Color::Yellow => {
                                    let tmp1 = stack.pop().unwrap();
                                    let tmp2 = stack.pop().unwrap();
                                    stack.push(tmp1 - tmp2);
                                },
                            //Color::Green => {},
                            Color::Blue => {
                                    /* ewwww */
                                    print!("{}", to_char(stack.pop().unwrap()));
                                },
                            //Color::Purple => {},
                            _ => {}
                        }
                    }
                },
            "right" => {
                    current_direction = "right";
                    if current_stone != Color::Red {
                        match current_stone {
                            //Color::Orange => {},
                            Color::Yellow => {
                                    let tmp1 = stack.pop().unwrap();
                                    let tmp2 = stack.pop().unwrap();
                                    stack.push(tmp1 / tmp2);
                                },
                            //Color::Green => {},
                            //Color::Blue => {
                            //        print!("{}", stack.pop().unwrap());
                            //    },
                            //Color::Purple => {},
                            _ => {}
                        }
                    }
                },

            /* numbers */
            "1" => {
                    current_number = "1";
                    if current_stone == Color::Red {
                        match current_direction {
                            "up" => stack.push(0),
                            "down" => stack.push(1),
                            "left" => stack.push(2),
                            "right" => stack.push(3),
                            _ => panic!("Unexpected reserved word!"),
                        }
                    } else {
                        println!("That {:?} stone is too heavy!", &current_stone);
                    }
                },
            "2" => {
                    current_number = "2";
                    if current_stone == Color::Red {
                        match current_direction {
                            "up" => stack.push(4),
                            "down" => stack.push(5),
                            "left" => stack.push(6),
                            "right" => stack.push(7),
                            _ => panic!("Unexpected reserved word!"),
                        }
                    } else {
                        println!("That {:?} stone is too heavy!", &current_stone);
                    }
                },
            "3" => {
                    current_number = "3";
                    if current_stone == Color::Red {
                        match current_direction {
                            "up" => stack.push(8),
                            "down" => stack.push(9),
                            "left" => stack.push(1),
                            "right" => stack.push(0),
                            _ => panic!("Unexpected reserved word!"),
                        }
                    } else {
                        println!("That {:?} stone is too heavy!", &current_stone);
                    }
                },
            _ => { }
        }

        if debug {
            println!("");
            println!("Token:     {}", tokens[count]);
            println!("Color:     {:?}", current_stone);
            println!("Direction: {}", current_direction.trim());
            println!("Number:    {}", current_number.trim());
        }

        if show_stack {
            println!("-------");
            for item in &stack {
                println!("{}", item);
            }
        }

        count = count + 1;
    }
}

fn is_color(c: &str) -> bool {
    match c {
        "red" | "orange" | "yellow" | "green" | "blue" | "purple" => true,
        _ => false
    }
}

/* Copied from superfish.rs */
fn to_char(i: i64) -> String {
    let a = match i {
        0 => "NUL",
        1 => "SOH",
        2 => "STX",
        3 => "ETX",
        4 => "EOT",
        5 => "ENQ",
        6 => "ACK",
        7 => "BEL",
        8 => "BS",
        9 => "\t",
        10 => "\n",
        11 => "VT",
        12 => "TT",
        13 => "\r",
        14 => "SO",
        15 => "SI",
        16 => "DLE",
        17 => "DC1",
        18 => "DC2",
        19 => "DC3",
        20 => "DC4",
        21 => "NAK",
        22 => "SYN",
        23 => "ETB",
        24 => "CAN",
        25 => "EM",
        26 => "SUB",
        27 => "ESC",
        28 => "FS",
        29 => "GS",
        30 => "RS",
        31 => "US",
        32 => " ",
        33 => "!",
        34 => "\"",
        35 => "#",
        36 => "$",
        37 => "%",
        38 => "&",
        39 => "\'",
        40 => "(",
        41 => ")",
        42 => "*",
        43 => "+",
        44 => ",",
        45 => "-",
        46 => ".",
        47 => "/",
        48 => "0",
        49 => "1",
        50 => "2",
        51 => "3",
        52 => "4",
        53 => "5",
        54 => "6",
        55 => "7",
        56 => "8",
        57 => "9",
        58 => ":",
        59 => ";",
        60 => "<",
        61 => "=",
        62 => ">",
        63 => "?",
        64 => "@",
        65 => "A",
        66 => "B",
        67 => "C",
        68 => "D",
        69 => "E",
        70 => "F",
        71 => "G",
        72 => "H",
        73 => "I",
        74 => "J",
        75 => "K",
        76 => "L",
        77 => "M",
        78 => "N",
        79 => "O",
        80 => "P",
        81 => "Q",
        82 => "R",
        83 => "S",
        84 => "T",
        85 => "U",
        86 => "V",
        87 => "W",
        88 => "X",
        89 => "Y",
        90 => "Z",
        91 => "[",
        92 => "\\",
        93 => "]",
        94 => "^",
        95 => "_",
        96 => "`",
        97 => "a",
        98 => "b",
        99 => "c",
        100 => "d",
        101 => "e",
        102 => "f",
        103 => "g",
        104 => "h",
        105 => "i",
        106 => "j",
        107 => "k",
        108 => "l",
        109 => "m",
        110 => "n",
        111 => "o",
        112 => "p",
        113 => "q",
        114 => "r",
        115 => "s",
        116 => "t",
        117 => "u",
        118 => "v",
        119 => "w",
        120 => "x",
        121 => "y",
        122 => "z",
        123 => "{",
        124 => "|",
        125 => "}",
        126 => "~",
        127 => "DEL",
        _ => "",
    };
    String::from(a)
}
