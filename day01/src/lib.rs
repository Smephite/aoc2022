#![no_std]

use soroban_sdk::{contractimpl, vec, Bytes, Env, Vec, bytes};

pub struct Day01;

#[contractimpl]
impl Day01 {
    #[allow(unused_mut)]
    pub fn haupt(e: Env, mut inp: Bytes) -> (u32, u32) {

        // force to be suffixed by new line such that parser works
        if inp.iter().last().unwrap() != 0x0a {
            inp.append(&bytes!(&e, 0x0a));
        }

        let mut input = inp
            .iter()
            .fold(
                (vec![&e], 0),
                |(mut list, mut tmp): (Vec<u32>, u32), curr| {
                    if curr == 0x0a {
                        list.push_back(tmp);
                        tmp = 0;
                    } else {
                        tmp = tmp * 10 + (curr as u32) - 0x30; // ascii to decimal
                    }
                    (list, tmp)
                },
            )
            .0;

            input.push_back(0); // postfix with 0 to denote end


            let added_elves = input.iter().fold((vec![&e], 0), |(mut list, mut tmp): (Vec<u32>, u32), curr| {
                let val = curr.unwrap();
                if val == 0 {
                    list.push_back(tmp);
                    tmp = 0;
                } else {
                    tmp += val;
                }
                (list, tmp)
            }).0;

            let mut top_3 : (u32, u32, u32) = (0, 0, 0);

            for elve in added_elves {
                let elve = elve.unwrap();
                if elve > top_3.0 {
                    top_3.2 = top_3.1;
                    top_3.1 = top_3.0;
                    top_3.0 = elve;
                } else if elve > top_3.1 {
                    top_3.2 = top_3.1;
                    top_3.1 = elve;
                } else if elve > top_3.2 {
                    top_3.2 = elve;
                }
            }


        (top_3.0, top_3.0 + top_3.1 + top_3.2)
    }

    // the input must end in 0x0A0A !!
    pub fn efficient(_: Env, inp: Bytes) -> (u32, u32) {
        let mut elf_tmp = 0u32;
        let mut line_tmp = 0u32;
        let mut first = 0u32;
        let mut scnd = 0u32;
        let mut third = 0u32;


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
                line_tmp = line_tmp*10 +  ((char as u32)-0x30);
            }
        }

        (first, first + scnd + third)
    }
}
