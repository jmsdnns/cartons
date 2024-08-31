enum Genre {
    Punk,
    Metal,
    Emo,
    Country,
}

struct Band {
    genre: Genre,
    name: String,
}

fn guess(band: &Band) -> String {
    match band.genre {
        Genre::Emo => String::from("emo is pretty good!"),
        Genre::Punk => String::from("punk is wild and amazing"),
        Genre::Metal => String::from("metal requires focus"),
        _ => String::from("not my thing"),
    }
}

fn guess_opinions() {
    let p = Band {
        genre: Genre::Punk,
        name: "Propagandhi".to_string(),
    };
    let mm = Band {
        genre: Genre::Metal,
        name: "Mutoid Man".to_string(),
    };
    let pr = Band {
        genre: Genre::Emo,
        name: "Promise Ring".to_string(),
    };

    [pr, mm, p]
        .iter()
        .for_each(|b| println!("{}: {}", b.name, guess(b)));
}

#[derive(Debug)]
struct Kick {
    brand: String,
}

#[derive(Debug)]
struct Snare {
    brand: String,
}

#[derive(Debug)]
struct RackTom {
    brand: String,
}

#[derive(Debug)]
struct FloorTom {
    brand: String,
}

#[derive(Debug)]
struct HighHat {
    brand: String,
}

#[derive(Debug)]
struct Ride {
    brand: String,
}

#[derive(Debug)]
struct Cymbal {
    brand: String,
}

#[derive(Debug)]
enum Drums {
    Punk(
        Kick,
        Snare,
        RackTom,
        FloorTom,
        HighHat,
        Ride,
        Cymbal,
        Cymbal,
    ),
    Metal(
        Kick,
        Snare,
        RackTom,
        RackTom,
        RackTom,
        FloorTom,
        HighHat,
        Ride,
        Cymbal,
        Cymbal,
    ),
    Emo(Kick, Snare, RackTom, FloorTom, HighHat, Ride),
}

fn explore_drums() {
    let my_drums = Drums::Punk(
        Kick {
            brand: "DW".to_string(),
        },
        Snare {
            brand: "Ludwig".to_string(),
        },
        RackTom {
            brand: "DW".to_string(),
        },
        FloorTom {
            brand: "DW".to_string(),
        },
        HighHat {
            brand: "Zildjian".to_string(),
        },
        Ride {
            brand: "Zildjian".to_string(),
        },
        Cymbal {
            brand: "Zildjian".to_string(),
        },
        Cymbal {
            brand: "Zildjian".to_string(),
        },
    );

    // extract just the snare brand
    if let Drums::Punk(_, Snare { brand: snare }, ..) = my_drums {
        println!("My snare: {:?}", snare);
    }
}

pub fn run() {
    guess_opinions();
    explore_drums();
}
