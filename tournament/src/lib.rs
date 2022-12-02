use std::collections::HashMap;

#[derive(Clone)]
struct Result {
    team_name: String,
    played: u64,
    points: u64,
    won: u64,
    drawn: u64,
    lost: u64,
}

impl Result {
    fn new_empty(team_name: String) -> Self {
        Result {
            team_name,
            played: 0,
            points: 0,
            won: 0,
            drawn: 0,
            lost: 0,
        }
    }
}

fn format_output(results: &Vec<Result>) -> String {
    let mut output = String::new();
    output.push_str(
        format!(
            "{:<31}| {:^3}| {:^3}| {:^3}| {:^3}|{:>3}",
            "Team", "MP", "W", "D", "L", "P"
        )
        .as_str(),
    );
    for result in results {
        output.push_str(
            format!(
                "\n{:<31}| {:^3}| {:^3}| {:^3}| {:^3}|{:>3}",
                result.team_name,
                result.played,
                result.won,
                result.drawn,
                result.lost,
                result.points
            )
            .as_str(),
        );
    }
    output
}

pub fn tally(match_results: &str) -> String {
    let mut results: HashMap<&str, Result> = HashMap::new();
    for result in match_results.lines() {
        let result_parts: Vec<_> = result.split(';').collect();
        match result_parts.as_slice() {
            [first_team, second_team, verdict] => {
                let mut first_team_stats = results
                    .get(first_team)
                    .cloned()
                    .unwrap_or_else(|| Result::new_empty(first_team.to_string()));
                let mut second_team_stats = results
                    .get(second_team)
                    .cloned()
                    .unwrap_or_else(|| Result::new_empty(second_team.to_string()));
                match *verdict {
                    "win" => {
                        first_team_stats.won += 1;
                        second_team_stats.lost += 1;
                        first_team_stats.points += 3;
                    }
                    "loss" => {
                        second_team_stats.won += 1;
                        first_team_stats.lost += 1;
                        second_team_stats.points += 3;
                    }
                    "draw" => {
                        first_team_stats.drawn += 1;
                        second_team_stats.drawn += 1;
                        first_team_stats.points += 1;
                        second_team_stats.points += 1;
                    }
                    _ => panic!("Invalid value for verdict: {}", verdict),
                }
                first_team_stats.played += 1;
                second_team_stats.played += 1;
                results.insert(first_team, first_team_stats);
                results.insert(second_team, second_team_stats);
            }
            _ => panic!("The input is not in the expected format"),
        }
    }
    let mut results = results.into_values().collect::<Vec<_>>();
    results.sort_by(|a, b| {
        if a.points != b.points {
            a.points.cmp(&b.points).reverse()
        } else {
            a.team_name.cmp(&b.team_name)
        }
    });
    format_output(&results)
}
