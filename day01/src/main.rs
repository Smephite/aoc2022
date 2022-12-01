roesti::röschti! {
    bruch itertools::Itertools;
    funktion haupt() {
        lahn eingabe : Vec<Zeichecheti> = std::es::stdin().lines().map(|l| Zeichecheti::from(l.entpacke().as_str())).collect();

        lahn veränderbar elfen : Vec<Vec<usize>> = vec![];
        lahn veränderbar jetzige_elfe = vec![];

        für kalorie in eingabe {
            falls kalorie.is_empty() {
                elfen.push(jetzige_elfe);
                jetzige_elfe = vec![];
            } susch {
                jetzige_elfe.push(kalorie.parse::<usize>().entpacke());
            }
        }
        elfen.push(jetzige_elfe);


        lahn elfen_insgesamt = elfen.iter().map(|elfe| elfe.iter().sum::<usize>());
        lahn am_meisten_getragen = elfen_insgesamt.clone().max().entpacke();

        usdrucke!("Die am meisten beladene Elfe trägt {} Kalorien.", am_meisten_getragen);

        lahn top_3_elfen = elfen_insgesamt.sorted().rev().take(3);
        lahn von_top_3_getragen : usize = top_3_elfen.sum();

        usdrucke!("Die Top 3 Elfen tragen zusammen {} Kalorien.", von_top_3_getragen);
    }
}
