pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut annotated_minefield: Vec<String> = Vec::new();
    let x_length = if minefield.is_empty() { 0 } else { minefield[0].len() };
    let y_length = minefield.len();

    for _ in 0..y_length {
        annotated_minefield.push(' '.to_string().repeat(x_length));
    }

    for y in 0..y_length {
        for x in 0..x_length {
            match minefield[y].chars().nth(x).unwrap() {
                '*' => {
                    annotated_minefield[y].replace_range(x..x + 1, "*");
                },
                _ => {
                    let mut count = 0;

                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            let new_y = y as isize + dy;
                            let new_x = x as isize + dx;

                            if new_y >= 0 && new_y < y_length as isize && new_x >= 0 && new_x < x_length as isize {
                                if minefield[new_y as usize].chars().nth(new_x as usize).unwrap() == '*' {
                                    count += 1;
                                }
                            }
                        }
                    }

                    if count > 0 {
                        annotated_minefield[y].replace_range(x..x+1, &count.to_string());
                    }
                }
            }
        }
    }
    
    return annotated_minefield;
}

#[cfg(test)]
fn remove_annotations(board: &[&str]) -> Vec<String> {
    board.iter().map(|r| remove_annotations_in_row(r)).collect()
}

#[cfg(test)]
fn remove_annotations_in_row(row: &str) -> String {
    row.chars()
        .map(|ch| match ch {
            '*' => '*',
            _ => ' ',
        })
        .collect()
}

#[cfg(test)]
fn run_test(test_case: &[&str]) {
    let cleaned = remove_annotations(test_case);
    let cleaned_strs = cleaned.iter().map(|r| &r[..]).collect::<Vec<_>>();
    let expected = test_case.iter().map(|&r| r.to_string()).collect::<Vec<_>>();
    assert_eq!(expected, annotate(&cleaned_strs));
}

#[test]
fn no_rows() {
    #[rustfmt::skip]
    run_test(&[
    ]);
}

#[test]
fn no_columns() {
    #[rustfmt::skip]
    run_test(&[
        "",
    ]);
}

#[test]
fn no_mines() {
    #[rustfmt::skip]
    run_test(&[
        "   ",
        "   ",
        "   ",
    ]);
}

#[test]
fn board_with_only_mines() {
    #[rustfmt::skip]
    run_test(&[
        "***",
        "***",
        "***",
    ]);
}

#[test]
fn mine_surrounded_by_spaces() {
    #[rustfmt::skip]
    run_test(&[
        "111",
        "1*1",
        "111",
    ]);
}

#[test]
fn space_surrounded_by_mines() {
    #[rustfmt::skip]
    run_test(&[
        "***",
        "*8*",
        "***",
    ]);
}

#[test]
fn horizontal_line() {
    #[rustfmt::skip]
    run_test(&[
        "1*2*1",
    ]);
}

#[test]
fn horizontal_line_mines_at_edges() {
    #[rustfmt::skip]
    run_test(&[
        "*1 1*",
    ]);
}

#[test]
fn vertical_line() {
    #[rustfmt::skip]
    run_test(&[
        "1",
        "*",
        "2",
        "*",
        "1",
    ]);
}

#[test]
fn vertical_line_mines_at_edges() {
    #[rustfmt::skip]
    run_test(&[
        "*",
        "1",
        " ",
        "1",
        "*",
    ]);
}

#[test]
fn cross() {
    #[rustfmt::skip]
    run_test(&[
        " 2*2 ",
        "25*52",
        "*****",
        "25*52",
        " 2*2 ",
    ]);
}

#[test]
fn large_board() {
    #[rustfmt::skip]
    run_test(&[
        "1*22*1",
        "12*322",
        " 123*2",
        "112*4*",
        "1*22*2",
        "111111",
    ]);
}
