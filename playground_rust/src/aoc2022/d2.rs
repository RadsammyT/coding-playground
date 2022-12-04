use super::input_parse;

pub fn p1(fp: String) {
    let input = input_parse::parse_to_vec_d2(fp);
    // OPPONENT | PLAYER LETTER TABLE
    // A, X = ROCK/ LOSE, 1
    // B, Y = PAPER/ DRAW, 2
    // C, Z = SCISSORS/ WIN, 3
    // LOSE = 0, TIE = 3, WIN = 6.
    let mut total: i64 = 0;


    for i in input {
        let op = *i.get(0).unwrap_or_else(move || {
            println!("ERROR! OPPONENT CHAR IS NOT AVAILABLE!");
            panic!("SEE ABOVE!");
        });

        let pl = *i.get(1).unwrap_or_else(move || {
            println!("ERROR! PLAYER CHAR IS NOT AVAILABLE!");
            panic!("SEE ABOVE!");
        });

        // hot single ifs in your area

        if op == 'A' { // rock
            if pl == 'X' { //rock
                total += 4; // 3 for tie, 1 for rock.
            }
            
            if pl == 'Y' {
                total += 8; // 6 for win. 2 for paper.
            }

            if pl == 'Z' {
                total += 3; // 0 for lose, 3 for scissors.z
            }
        }

        if op == 'B' { // paper
            if pl == 'X' { // rock
                total += 1; // 0 for lose, 1 for rock.
            }
            
            if pl == 'Y' {
                total += 5; // 3 for tie, 2 for paper. 
            }

            if pl == 'Z' {
                total += 9; // 6 for win, 3 for scissors. 
            }
        }

        if op == 'C' { // scissors
            if pl == 'X' { // rock
                total += 7; // 6 for win, 1 for rock. 
            }
            
            if pl == 'Y' {
                total += 2; // 0 for lose, 2 for paper. 
            }

            if pl == 'Z' {
                total += 6; // 3 for tie, 3 for scissors
            }
        }
        
    }

    println!("TOTAL = {}", total);
}

pub fn p2(fp: String) {
    let input = input_parse::parse_to_vec_d2(fp);
    // OPPONENT | PLAYER LETTER TABLE
    // A, X = ROCK/ LOSE, 1
    // B, Y = PAPER/ DRAW, 2
    // C, Z = SCISSORS/ WIN, 3
    // LOSE = 0, TIE = 3, WIN = 6.
    let mut total: i64 = 0;


    for i in input {
        let op = *i.get(0).unwrap_or_else(move || {
            println!("ERROR! OPPONENT CHAR IS NOT AVAILABLE!");
            panic!("SEE ABOVE!");
        });

        let pl = *i.get(1).unwrap_or_else(move || {
            println!("ERROR! PLAYER CHAR IS NOT AVAILABLE!");
            panic!("SEE ABOVE!");
        });

        if op == 'A' { // rock
            if pl == 'X' { // scissors
                total += 3; // 0 for lose, 3 for scissors
            }
            
            if pl == 'Y' { // rock
                total += 4; // 3 for tie, 1 for rock..
            }

            if pl == 'Z' { // paper
                total += 8; //6 for win, 2 for paper.
            }
        }

        if op == 'B' { // paper
            if pl == 'X' { // rock
                total += 1; // 0 for lose, 1 for rock.
            }
            
            if pl == 'Y' { // paper
                total += 5; // 3 for tie, 2 for paper. 
            }

            if pl == 'Z' { // scissors
                total += 9; // 6 for win, 3 for scissors. 
            }
        }

        if op == 'C' { // scissors
            if pl == 'X' { // paper
                total += 2; // 0 for lose, 2 for paper. 
            }
            
            if pl == 'Y' { // scissors
                total += 6; // 3 for tie, 3 for scissors
            }

            if pl == 'Z' { // rock
                total += 7; // 6 for win, 1 for rock.
            }
        }
        
    }

    println!("TOTAL = {}", total);
}
