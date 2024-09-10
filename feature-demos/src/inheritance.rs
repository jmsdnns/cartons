trait Drummer {
    // just the interface
    // fn play_drums(&self);

    // default imlpementation
    fn play_drums(&self) {
        println!("an amazing beat is played");
    }
}

struct JoshFreese;
impl Drummer for JoshFreese {}

struct JonahFalco;
impl Drummer for JonahFalco {
    fn play_drums(&self) {
        println!("falco is ripping those drums");
    }
}

fn needs_drummer(musician: &impl Drummer) {
    musician.play_drums();
}

trait Guitarist {
    fn play_guitar(&self) {
        println!("a heavy guitar riff is played");
    }
}

struct JamesHetfield;
impl Guitarist for JamesHetfield {}

// Super trait
trait MultiMusician: Guitarist + Drummer {}

struct DaveGrohl;
impl MultiMusician for DaveGrohl {}
impl Drummer for DaveGrohl {}
impl Guitarist for DaveGrohl {
    fn play_guitar(&self) {
        println!("DaveGrohl is floating");
    }
}

fn record_song(musician: &impl MultiMusician) {
    musician.play_drums();
    musician.play_guitar();
}

pub fn main() {
    let jf = JoshFreese;
    needs_drummer(&jf);

    let jf = JonahFalco;
    needs_drummer(&jf);

    let jh = JamesHetfield;
    jh.play_guitar();

    let dg = DaveGrohl;
    record_song(&dg);
}
