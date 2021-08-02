pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    input
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(j, x)| {
                    match row.iter().all(|ele| *x >= *ele)
                        && input.iter().map(|e| e[j]).all(|z| z >= *x)
                    {
                        true => Some((i, j)),
                        false => None,
                    }
                })
                .collect::<Vec<(usize, usize)>>()
        })
        .flatten()
        .collect()
}
