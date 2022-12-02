roesti::röschti! {
    funktion haupt() {
        lahn eingabe : Vec<Zeichecheti> = std::es::stdin().lines().filter(|l| l.is_ok()).map(|l| Zeichecheti::from(l.entpacke().as_str())).collect();

        lahn spiel_wertung : (isize, isize) = eingabe.iter().fold((0, 0), |(score1, score2), zeili| {

            if zeili.is_empty() {
                zruggäh (score1, score2);
            }
            lahn veränderbar data = zeili.split(" ");
            lahn gegner = data.next().unwrap();
            lahn zweites_arg = data.next().unwrap();
            
            lahn gegner: isize  = überistimme gegner {
                "A" => 0,
                "B" => 1,
                "C" => 2,
                _ => scheisse!("huere schafseckel")
            };

            lahn ich1 : isize = überistimme zweites_arg {
                "X" => 0,
                "Y" => 1,
                "Z" => 2,
                _ => scheisse!("huere schafseckel")
            };

            lahn spiel_wert1 =  3 
            + 3 * falls ich1 == gegner {
                    0
                } susch falls (ich1 + 1) % 3 == gegner % 3 {
                    -1
                } susch {
                    1
                };

            lahn spiel_wertung_2 = überistimme zweites_arg {
                "X" => 0,
                "Y" => 3,
                "Z" => 6,
                _ => scheisse!("huere schafseckel")
            };

            lahn mein_zug_2 = überistimme zweites_arg {
                "X" => (gegner - 1).rem_euclid(3),
                "Y" => gegner,
                "Z" => (gegner + 1 ) % 3,
                _ => scheisse!("huere schafseckel")
            };



            (score1 + (ich1 + 1) + spiel_wert1, score2 + spiel_wertung_2 + (mein_zug_2 + 1))
        });

        usdrucke!("Die gesamte Wertung wäre {:?}", spiel_wertung);

    }
}