

pub fn evaluate_tournament(rounds: &Vec<&str>) -> u64
{
    let mut total: u64 = 0;

    for round in rounds
    {
        total += evaluate_round(round);
    }

    return total;
}

pub fn evaluate_tournament_the_second(rounds: &Vec<&str>) -> u64
{
    let mut total: u64 = 0;

    for round in rounds
    {
        total += evaluate_round_the_second(round);
    }

    return total;
}

pub fn evaluate_round_the_second(round: &str) -> u64
{
    let (their_play, my_play) = interpret_play_line(round);
    let play_score = score_play(&their_play, &my_play);

    return play_score + my_play as u64;
}



pub fn evaluate_round(round: &str) -> u64
{

    let (opponent, me) = translate_round_str(round);
    let play_score = score_play(&opponent, &me);

    return play_score + me as u64;
}

pub fn interpret_play_line(round: &str) -> (Play, Play)
{
    if let Some((their, mine)) = round.split_once(" ")
    {
        match their
        {
            "A" =>
            {
                match mine 
                {
                    "X" => (Play::Rock, Play::Scissor),
                    "Y" => (Play::Rock, Play::Rock),
                    "Z" => (Play::Rock, Play::Paper),
                    _ => {panic!("Unknown win/lose/draw column type");}
                }
            }
            "B" =>
            {
                match mine
                {
                    "X" => (Play::Paper, Play::Rock),
                    "Y" => (Play::Paper, Play::Paper),
                    "Z" => (Play::Paper, Play::Scissor),
                    _ => {panic!("Unknown win/lose/draw column type");}
                }
            }
            "C" =>
            {
                match mine
                {
                    "X" => (Play::Scissor, Play::Paper),
                    "Y" => (Play::Scissor, Play::Scissor),
                    "Z" => (Play::Scissor, Play::Rock),
                    _ => {panic!("Unknown win/lose/draw column type");}
                }
            }
            _ => {panic!("Unknown type")}
        }
    }
    else 
    {
        panic!("Unknown play format");
    }
}

fn score_play(opponent: &Play, mine: &Play) -> u64
{
    match opponent
    {
        Play::Rock => 
        {
            match mine
            {
                Play::Rock => 3,
                Play::Paper => 6,
                Play::Scissor => 0,
            }
        },
        Play::Paper => 
        {
            match mine
            {
                Play::Rock => 0,
                Play::Paper => 3,
                Play::Scissor => 6,
            }
        },
        Play::Scissor => 
        {
            match mine
            {
                Play::Rock => 6,
                Play::Paper => 0,
                Play::Scissor => 3,
            }
        },
    }
}

fn translate_round_str(round: &str) -> (Play, Play)
{
    let opponent: Play;
    let mine: Play;

    if let Some((opp_str, my_str)) = round.split_once(" ")
    {
        match opp_str
        {
            "A" => {opponent = Play::Rock},
            "B" => {opponent = Play::Paper},
            "C" => {opponent = Play::Scissor},
            _ => {panic!("It's late, I want tea, I'm not building some big error handling system for code challenges.")}
        }
        match my_str
        {
            "X" => {mine = Play::Rock}
            "Y" => {mine = Play::Paper}
            "Z" => {mine = Play::Scissor}
            _ => {panic!("Still not doing a big error handling system.")}
        }
    }
    else
    {
        panic!("This should be a simple space separated string.");
    }


    return (opponent, mine);
}

#[derive(Debug, PartialEq)]
pub enum Play
{
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}

#[cfg(test)]
mod tests
{
    use crate::day2::advent::{evaluate_round, evaluate_tournament, interpret_play_line};

    use super::Play;

    #[test]
    pub fn when_player_must_lose_and_opponent_plays_a_interpret_play_line_returns_scissor()
    {
        let play = "A X";
        let (_their_play, my_play) = interpret_play_line(play);
        assert_eq!(my_play, Play::Scissor);
    }

    #[test]
    pub fn when_player_must_lose_and_my_opponent_plays_b_interpret_play_line_returns_rock()
    {
        let play = "B X";
        let (_their_play, my_play) = interpret_play_line(play);
        assert_eq!(my_play, Play::Rock);
    }

    #[test]
    pub fn when_player_must_lose_and_my_opponent_plays_c_interpret_play_line_returns_paper()
    {
        let play = "C X";
        let (_their_play, my_play) = interpret_play_line(play);
        assert_eq!(my_play, Play::Paper);
    }

    #[test]
    pub fn when_player_must_draw_and_my_opponent_plays_a_interpret_play_line_returns_rock()
    {
        let play = "A Y";
        let (_their_play, my_play) = interpret_play_line(play);
        assert_eq!(my_play, Play::Rock);
    }

    #[test]
    pub fn when_player_must_draw_and_my_opponent_plays_b_interpret_play_line_returns_paper()
    {
        let play = "B Y";
        let (_their_play, my_play) = interpret_play_line(play);
        assert_eq!(my_play, Play::Paper);
    }

    #[test]
    pub fn when_player_must_draw_and_my_opponent_plays_c_interpret_play_line_returns_scissors()
    {
        let play = "C Y";
        let (_their_play, my_play) = interpret_play_line(play);
        assert_eq!(my_play, Play::Scissor)
    }

    #[test]
    pub fn when_player_must_win_and_my_opponent_plays_a_interpret_play_line_returns_paper()
    {
        let play = "A Z";
        let (_their_play, my_play) = interpret_play_line(play);
        assert_eq!(my_play, Play::Paper)
    }

    #[test]
    pub fn when_player_must_win_and_my_opponent_plays_b_interpret_play_line_returns_scissors()
    {
        let play = "B Z";
        let (_their_play, my_play) = interpret_play_line(play);
        assert_eq!(my_play, Play::Scissor)
    }
    
    #[test]
    pub fn when_player_must_win_and_opponent_plays_c_interpret_play_line_returns_rock()
    {
        let play = "C Z";
        let (__their_play, my_play) = interpret_play_line(play);
        assert_eq!(my_play, Play::Rock)
    }

    #[test]
    pub fn given_a_tourney_of_plays_rock_paper_rock_scissors_paper_rock_your_score_should_evaluate_to_12()
    {
        let plays = vec!["A Y", "A Z", "B X"];

        let score = evaluate_tournament(&plays);

        assert_eq!(score, 12);
    }

    #[test]
    pub fn given_a_single_round_with_plays_rock_scissors_your_score_should_evaluate_to_3()
    {
        let round = "A Z";

        let score = evaluate_round(round);

        assert_eq!(score, 3);
    }

    #[test]
    pub fn given_a_single_round_with_plays_rock_paper_your_score_should_evaluate_to_8()
    {
        let round = "A Y";

        let score = evaluate_round(round);

        assert_eq!(score, 8);
    }

    #[test]
    pub fn given_a_single_round_with_plays_rock_rock_your_score_should_evaluate_to_4()
    {
        let round = "A X";

        let score = evaluate_round(round);

        assert_eq!(score, 4);
    }

    #[test]
    pub fn given_a_single_round_with_plays_paper_scissors_your_score_should_evaluate_to_9()
    {
        let round = "B Z";
        let score = evaluate_round(round);
        assert_eq!(score, 9);
    }

    #[test]
    pub fn given_a_single_round_with_plays_paper_paper_your_score_should_evaluate_to_5()
    {
        let round = "B Y";
        let score = evaluate_round(round);
        assert_eq!(score, 5);
    }

    #[test]
    pub fn given_a_single_round_with_plays_paper_rock_your_score_should_evaluate_to_1()
    {
        let round = "B X";
        let score = evaluate_round(round);
        assert_eq!(score, 1);
    }

    #[test]
    pub fn given_a_single_round_with_plays_scissors_scissors_your_score_should_evaluate_to_6()
    {
        let round = "C Z";
        let score = evaluate_round(round);
        assert_eq!(score, 6);
    }

    #[test]
    pub fn given_a_single_round_with_plays_scissors_paper_your_score_should_evaluate_to_2()
    {
        let round = "C Y";
        let score = evaluate_round(round);
        assert_eq!(score, 2);
    }

    #[test]
    pub fn given_a_single_round_with_plays_scissors_rock_your_score_should_evaluate_to_7()
    {
        let round = "C X";
        let score = evaluate_round(round);
        assert_eq!(score, 7);
    }
}