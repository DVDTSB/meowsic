use meowsic::get_chords;

fn main() {
    println!("{:?}", get_chords(&vec![60, 64, 67, 71]));
    //["Cmaj7", "Emb6", "Em/C", "G6/11", "G6/C", "Bb6sus4(b9)", "Bb6sus4/C"]
}
