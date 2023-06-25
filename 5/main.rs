/*
[V]     [B]                     [C]
[C]     [N] [G]         [W]     [P]
[W]     [C] [Q] [S]     [C]     [M]
[L]     [W] [B] [Z]     [F] [S] [V]
[R]     [G] [H] [F] [P] [V] [M] [T]
[M] [L] [R] [D] [L] [N] [P] [D] [W]
[F] [Q] [S] [C] [G] [G] [Z] [P] [N]
[Q] [D] [P] [L] [V] [D] [D] [C] [Z]
 1   2   3   4   5   6   7   8   9
*/

fn main() {

    let binding = "\
        QFMRLWCV\n\
        DQL\n\
        PSRGWCNB\n\
        LCDHBQG\n\
        VGLFZS\n\
        DGNP\n\
        DZPVFCW\n\
        CPDMS\n\
        ZNWTVMPC\n";

    let mut ship: Vec<Vec<_>> = binding
        .split("\n")
        .map(|x| x.trim())
        .filter(|y| !y.is_empty())
        .map(|_crate_name| {
            _crate_name
                .split("")
                .filter(|x| !x.is_empty())
                .collect::<Vec<_>>()
        })
        .collect();
            
    let raw_data = std::fs::read_to_string("input.txt")
        .expect("the file's absence! ");

    raw_data
        .lines()
        .skip(10)
        .for_each(|cmd| {
            let cmd: Cmd = cmd.parse().unwrap();
            let cargo_size = ship[cmd.from].len();

            for i in 0..cmd.number_items {
                let item = ship[cmd.from][cargo_size - cmd.number_items + i].clone();
                ship[cmd.to].push(item);
            }

            for _ in 0..cmd.number_items {
                ship[cmd.from].pop();
            }

        });

    for crane in ship {
        print!("{}", crane[crane.len() - 1]);
    }

}

struct Cmd {
    from: usize,
    to: usize,
    number_items: usize
}

impl std::str::FromStr for Cmd {
    type Err = ();
    fn from_str(cmd: &str) -> Result<Self, Self::Err> {
        let cmd = cmd.split(" ").collect::<Vec<_>>();

        let number_items = cmd[1].parse::<usize>().map_err(|_| ())?;
        let from = cmd[3].parse::<usize>().map_err(|_| ())? - 1;  // subtract 1 because indices start from 0
        let to = cmd[5].parse::<usize>().map_err(|_| ())? - 1;

        Ok(Self { from, to, number_items })
    }
}

