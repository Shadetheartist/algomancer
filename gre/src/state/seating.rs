
/// distributes the various players as evenly as possible amongst the seats at the table
/// output is a vec where idx = seat and the value is which team the player at that seat is on
pub(crate) fn interlace_players(teams_of_players: &[u8]) -> Vec<u8> {
    // fleshes out the vec of u8 into a more formal structure so
    // we don't lose track of what team each player is on during the interlacing process
    let teams: Vec<(u8, u8)> = teams_of_players.iter().enumerate().map(|(i, &t)| (i as u8, t)).collect();
    interlace_evenly(&teams)
}

/// distributes the various items as evenly as possible amongst indices of the resulting vector
/// by minimising the 'gap' for each element, which is calculated as the difference between the
/// desired and actual positions for the next occurrence of each character.
/// It selects the character with the smallest gap to place next in the result.
///
/// tuple is `[(team_number, num_players)]`
fn interlace_evenly(elements: &[(u8, u8)]) -> Vec<u8> {
    let total_count: u8 = elements.iter().map(|&(_, count)| count).sum();
    let mut result = Vec::with_capacity(total_count as usize);
    let mut indices = vec![0; elements.len()];

    for _ in 0..total_count {
        let mut min_gap = f64::INFINITY;
        let mut next_idx = 0;

        for (i, &(_, count)) in elements.iter().enumerate() {
            let gap = (total_count as f64) * (indices[i] as f64 + 1.0) / (count as f64) - result.len() as f64;
            if gap < min_gap {
                min_gap = gap;
                next_idx = i;
            }
        }

        let (ch, _) = elements[next_idx];
        result.push(ch);
        indices[next_idx] += 1;
    }

    result.into_iter().collect()
}