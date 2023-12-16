use std::cmp::Ordering;
use itertools;

#[derive(PartialEq)]
struct Problem {
    games: Vec<Game>,
}

impl Problem {
    fn get_solution(&self, red_max: u8, green_max: u8, blue_max: u8) -> u32 {
        self.games.iter().filter(|g| {
            g.is_valid_for(red_max, green_max, blue_max)
        }).map(|g| { g.id }).sum()
    }

    fn get_max_per_color(&self) -> (u8, u8, u8) {
        let l :Vec<_> = self.games.iter().flat_map(|g|{&g.picks}).map(|p|{(p.red,p.green,p.blue)}).collect();
        let (r,g,b): (Vec<_>, Vec<_>, Vec<_>) = itertools::multiunzip(l);
        (*r.iter().max().expect(""), *g.iter().max().expect(""), *b.iter().max().expect(""))
    }
}

#[derive(PartialEq, Debug)]
struct Game {
    id: u32,
    picks: Vec<Pick>,
}

impl Game {
    fn is_valid_for(&self, red_max: u8, green_max: u8, blue_max: u8) -> bool {
        self.picks.iter().all(|p| { p.is_valid_for(red_max, green_max, blue_max) })
    }
}

#[derive(PartialEq, Debug)]
struct Pick {
    red: u8,
    green: u8,
    blue: u8,
}

impl Pick {
    fn is_valid_for(&self, red_max: u8, green_max: u8, blue_max: u8) -> bool {
        if red_max < self.red {
            return false;
        }
        if green_max < self.green {
            return false;
        }
        if blue_max < self.blue {
            return false;
        }

        return true;
    }
}


fn read_file_into_problem(full_data: &String) -> Problem {
    let games: Vec<_> = full_data.lines().map(|s| {
        read_line_into_game(s)
    }).collect();

    Problem {
        games
    }
}

fn read_line_into_game(line: &str) -> Game {
    let game_id_and_data: Vec<_> = line.split(":").collect();
    let game_id = game_id_and_data[0];
    let game_data = game_id_and_data[1];

    let game_id = game_id.split_ascii_whitespace().collect::<Vec<_>>()[1];
    let picks: Vec<_> = game_data.split(";").collect();

    let mut parsed_picks: Vec<Pick> = Vec::new();

    for pick in picks {
        let mut red_count: u8 = 0;
        let mut green_count: u8 = 0;
        let mut blue_count: u8 = 0;
        let cubes: Vec<_> = pick.split(",").collect();
        for cube in cubes {
            let cr = cube.split_ascii_whitespace().collect::<Vec<_>>();
            let number = cr[0].parse::<u8>().expect("a valid number");
            match cr[1] {
                "red" => { red_count += number; }
                "green" => { green_count += number; }
                "blue" => { blue_count += number; }
                &_ => { panic!("unknown color"); }
            }
        }
        parsed_picks.push(Pick {
            red: red_count,
            green: green_count,
            blue: blue_count,
        })
    }

    Game { id: game_id.parse::<u32>().expect("u32 id"), picks: parsed_picks }
}

fn cube_conundrum(input: String, red: u8, green: u8, blue: u8) -> u32 {
    read_file_into_problem(&input).get_solution(red, green, blue)
}

#[cfg(test)]
mod tests {
    use crate::test::get_day_input;
    use super::{Game, Pick, Problem, read_file_into_problem, read_line_into_game};

    #[test]
    fn pick_validation_max_reached() {
        let p = Pick {
            red: 1,
            green: 1,
            blue: 1,
        };

        assert_eq!(true, p.is_valid_for(1, 1, 1))
    }

    #[test]
    fn pick_validation_zeros() {
        let p = Pick {
            red: 0,
            green: 0,
            blue: 0,
        };

        assert_eq!(true, p.is_valid_for(0, 0, 0))
    }

    #[test]
    fn pick_validation_simple() {
        let p = Pick {
            red: 50,
            green: 20,
            blue: 1,
        };

        assert_eq!(false, p.is_valid_for(49, 21, 1))
    }

    #[test]
    fn game_validation_simple() {
        let picks = vec![Pick {
            red: 49,
            green: 20,
            blue: 1,
        }];

        let g = Game { id: 0, picks };

        assert_eq!(true, g.is_valid_for(50, 20, 1))
    }

    #[test]
    fn game_validation_simple_false() {
        let picks = vec![Pick {
            red: 50,
            green: 20,
            blue: 1,
        }];

        let g = Game { id: 0, picks };

        assert_eq!(false, g.is_valid_for(49, 21, 1))
    }

    #[test]
    fn game_validation_mulitple_picks_true() {
        let picks = vec![Pick {
            red: 50,
            green: 20,
            blue: 1,
        }, Pick {
            red: 1,
            green: 1,
            blue: 1,
        }, Pick {
            red: 1,
            green: 1,
            blue: 1,
        }, Pick {
            red: 1,
            green: 1,
            blue: 1,
        }];

        let g = Game { id: 0, picks };

        assert_eq!(true, g.is_valid_for(50, 20, 1))
    }

    #[test]
    fn game_validation_mulitple_picks_false() {
        let picks = vec![Pick {
            red: 50,
            green: 20,
            blue: 1,
        }, Pick {
            red: 1,
            green: 1,
            blue: 1,
        }, Pick {
            red: 1,
            green: 1,
            blue: 1,
        }, Pick {
            red: 1,
            green: 1,
            blue: 1,
        }];

        let g = Game { id: 0, picks };

        assert_eq!(false, g.is_valid_for(1, 2, 1))
    }


    #[test]
    fn problem_solution_1() {
        let p = Problem {
            games: vec![Game {
                id: 20,
                picks: vec![Pick {
                    red: 50,
                    green: 20,
                    blue: 1,
                }, Pick {
                    red: 1,
                    green: 1,
                    blue: 1,
                }, Pick {
                    red: 1,
                    green: 1,
                    blue: 1,
                }, Pick {
                    red: 1,
                    green: 1,
                    blue: 1,
                }],
            }]
        };

        assert_eq!(0, p.get_solution(1, 1, 1))
    }

    #[test]
    fn problem_solution_2() {
        let p = Problem {
            games: vec![Game {
                id: 20,
                picks: vec![Pick {
                    red: 50,
                    green: 20,
                    blue: 1,
                }, Pick {
                    red: 1,
                    green: 1,
                    blue: 1,
                }, Pick {
                    red: 1,
                    green: 1,
                    blue: 1,
                }, Pick {
                    red: 1,
                    green: 1,
                    blue: 1,
                }],
            },
                        Game {
                            id: 30,
                            picks: vec![Pick {
                                red: 50,
                                green: 20,
                                blue: 1,
                            }, Pick {
                                red: 1,
                                green: 1,
                                blue: 1,
                            }, Pick {
                                red: 1,
                                green: 1,
                                blue: 1,
                            }, Pick {
                                red: 1,
                                green: 1,
                                blue: 1,
                            }],
                        },
            ]
        };

        assert_eq!(0, p.get_solution(1, 1, 1))
    }

    #[test]
    fn problem_solution_3() {
        let p = Problem {
            games: vec![Game {
                id: 20,
                picks: vec![Pick {
                    red: 50,
                    green: 20,
                    blue: 1,
                }, Pick {
                    red: 1,
                    green: 1,
                    blue: 1,
                }, Pick {
                    red: 1,
                    green: 1,
                    blue: 1,
                }, Pick {
                    red: 1,
                    green: 1,
                    blue: 1,
                }],
            },
                        Game {
                            id: 30,
                            picks: vec![Pick {
                                red: 50,
                                green: 20,
                                blue: 1,
                            }, Pick {
                                red: 1,
                                green: 1,
                                blue: 1,
                            }, Pick {
                                red: 1,
                                green: 1,
                                blue: 1,
                            }, Pick {
                                red: 1,
                                green: 1,
                                blue: 1,
                            }],
                        }, Game {
                    id: 7,
                    picks: vec![Pick {
                        red: 1,
                        green: 1,
                        blue: 1,
                    }, Pick {
                        red: 1,
                        green: 1,
                        blue: 1,
                    }, Pick {
                        red: 1,
                        green: 1,
                        blue: 1,
                    }, Pick {
                        red: 1,
                        green: 1,
                        blue: 1,
                    }],
                },
            ]
        };

        assert_eq!(7, p.get_solution(1, 1, 1))
    }

    #[test]
    fn read_line_into_game_test() {
        let data = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let result = read_line_into_game(data);
        let expected = Game {
            id: 1,
            picks: vec![Pick {
                red: 4,
                green: 0,
                blue: 3,
            }, Pick {
                red: 1,
                green: 2,
                blue: 6,
            }, Pick {
                red: 0,
                green: 2,
                blue: 0,
            }],
        };

        assert_eq!(result, expected)
    }

    #[test]
    fn solution() {
        let data = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string();

        let p = read_file_into_problem(&data);
        assert_eq!(8, p.get_solution(12, 13, 14))
    }

    #[test]
    fn day2_part_1_answer() {
        let data = get_day_input(2);
        let p = read_file_into_problem(&data);
        let result = p.get_solution(12, 13, 14);
        println!("{result}");
        // assert_eq!(8, p.get_solution(12,13,14))
    }
}