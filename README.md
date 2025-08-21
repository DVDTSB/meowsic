# meowsic

notes -> chord name in rust (`no-std` friendly too) :)

## use

```rust
use meowsic::get_chords;

fn main() {
    println!("{:?}", get_chords(&vec![60, 64, 67, 71]));
    // ["Cmaj7", "Em(b6)", "Em/C", "G6/11", "G6/C", "Bsus4(b6,b9)", "Bsus4(b6)/C"]
}
```
