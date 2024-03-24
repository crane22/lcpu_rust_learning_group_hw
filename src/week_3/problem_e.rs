pub fn quiz() {
    let mut total = 0;
    let mut curr = 0;
    let mut cards = [0; 10];
    loop {
        let line = read_line();
        let judge_card = match line.as_str() {
            "+2" => {
                for _ in 0..2 {
                    let drawn_card = draw_card();
                    cards[drawn_card] += 1;
                    curr += 1;
                    total += 1;
                }
                continue;
            },
            "+4" => {
                for _ in 0..4 {
                    let drawn_card = draw_card();
                    cards[drawn_card] += 1;
                    curr += 1;
                    total += 1;
                }
                continue;
            },
            _ => line.parse::<usize>().unwrap(),
        };
        if cards[judge_card] > 0 {
            cards[judge_card] -= 1;
            curr -= 1;
        } else {
            loop {
                let drawn_card = draw_card();
                cards[drawn_card] += 1;
                curr += 1;
                total += 1;
                if drawn_card == judge_card {
                    cards[drawn_card] -= 1;
                    curr -= 1;
                    break;
                }
            }
        }
        if curr == 1 {
            println!("UNO!");
            println!("{}", total);
            break;
        }
    }
}

fn read_line() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}

/// A magic deck for you :D
fn draw_card() -> usize {
    // 3, 6, 9, 2, 5, 8, 1, 4, 7, 0, then repeat
    use std::cell::Cell;
    thread_local! {
        static SEED: Cell<i32> = const { Cell::new(0) };
    }
    SEED.with(|seed| {
        let val = seed.get();
        let next = (val * 71 + 3) % 100;
        seed.set(next);
        (next % 10) as usize
    })
}