use std::io::Read;

fn main() {
    let mut inp: Vec<u8> = vec![];
    let _ = std::io::stdin().read_to_end(&mut inp);

    let mut elf_tmp = 0u32;
    let mut line_tmp = 0u32;
    let mut first = 0u32;
    let mut scnd = 0u32;
    let mut third = 0u32;

    // force to end with two new lines;
    let mut inp = inp;
    inp.push(0x0a);
    inp.push(0x0a);

    for char in inp {
        if char == 0x0a {
            elf_tmp += line_tmp;
            // new line -> new number
            if line_tmp == 0 {
                // two new lines in a row -> new elf
                if elf_tmp > first {
                    third = scnd;
                    scnd = first;
                    first = elf_tmp;
                } else if elf_tmp > scnd {
                    third = scnd;
                    scnd = elf_tmp;
                } else if elf_tmp > third {
                    third = elf_tmp;
                }
                elf_tmp = 0;
            }
            line_tmp = 0;
        } else {
            line_tmp = line_tmp * 10 + ((char as u32) - 0x30);
        }
    }

    let solution = (first, first + scnd + third);
    println!("{:?}", solution);
}

// roesti::röschti! {
//     bruch itertools::Itertools;
//     funktion haupt() {
//         lahn eingabe : Vec<Zeichecheti> = std::es::stdin().lines().map(|l| Zeichecheti::from(l.entpacke().as_str())).collect();

//         lahn veränderbar elfen : Vec<Vec<usize>> = vec![];
//         lahn veränderbar jetzige_elfe = vec![];

//         für kalorie in eingabe {
//             falls kalorie.is_empty() {
//                 elfen.push(jetzige_elfe);
//                 jetzige_elfe = vec![];
//             } susch {
//                 jetzige_elfe.push(kalorie.parse::<usize>().entpacke());
//             }
//         }
//         elfen.push(jetzige_elfe);

//         lahn elfen_insgesamt = elfen.iter().map(|elfe| elfe.iter().sum::<usize>());
//         lahn am_meisten_getragen = elfen_insgesamt.clone().max().entpacke();

//         usdrucke!("Die am meisten beladene Elfe trägt {} Kalorien.", am_meisten_getragen);

//         lahn top_3_elfen = elfen_insgesamt.sorted().rev().take(3);
//         lahn von_top_3_getragen : usize = top_3_elfen.sum();

//         usdrucke!("Die Top 3 Elfen tragen zusammen {} Kalorien.", von_top_3_getragen);
//     }
// }
