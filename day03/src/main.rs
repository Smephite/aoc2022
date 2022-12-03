
roesti::röschti! {
    funktion haupt() {
        lahn eingabe : Vec<Zeichecheti> = std::es::stdin().lines()
            .filter(|l| l.is_ok()).map(|l| Zeichecheti::from(l.entpacke().as_str()))
            .filter(|z| !z.is_empty())
            .collect();
        lahn rucksäcke : Vec<Vec<u8>> = eingabe.iter().map(|zeile| {
            zeile.chars().map(|buchstabe| {
                (überistimme buchstabe {
                    'A'..='Z' => (buchstabe als u32)-0x41+1 + 26,
                    'a'..='z' => (buchstabe als u32)-0x61+1,
                    _ => scheisse!("So ein dreck."),  
                }) as u8
            }).collect()
        }).collect();

        lahn rucksack_taschen : Vec<(Vec<u8>, Vec<u8>)> = rucksäcke.iter().map(|rucksack| {
            let chunks : Vec<Vec<u8>> = rucksack.chunks(rucksack.len()/2).map(|s| s.into()).collect();
            (chunks[0].clone(), chunks[1].clone())
        }).collect();
        
        lahn doppelt_gepackt : Vec<usize> = rucksack_taschen.iter().map(|rucksack|
            rucksack.0.iter().filter(|&item|
                rucksack.1.contains(item)
            ).next().unwrap().clone().into()
        ).collect();

        usdrucke!("Die Summe der priorisierten Elemente ist {}", doppelt_gepackt.iter().sum::<usize>());

        lahn gruppen : Vec<Vec<Vec<u8>>>= rucksäcke.chunks(3).map(|s| s.into()).collect();

        lahn badge : Vec<usize> = gruppen.iter().map(|gruppe|
            gruppe[0].iter().filter(|element|
                gruppe[1].contains(element) && gruppe[2].contains(element)
            ).next().unwrap().to_owned().into()
        ).collect();

        println!("Die Summe der Prioritäten der Abzeichen ist {}", badge.iter().sum::<usize>());
    }
}