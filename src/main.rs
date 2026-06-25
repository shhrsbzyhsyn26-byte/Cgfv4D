use std::io;
use std::collections::HashMap;
fn main() {
    #[derive(Debug, Clone)]
    enum Blox {
        Empty,
        Particle,
    }
    let (mut x,mut y,mut z,mut w) = (0, 0, 0, 0);
    let mut world: HashMap<(i32, i32, i32, i32), Blox> = HashMap::new();
    world.insert((0, 0, 0, 1), Blox::Particle);
    world.insert((0, 1, 1, 1), Blox::Particle);
    println!("({x}, {y}, {z}, {w})");
    loop {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    match input {
        "+x" => x += 1,
        "-x" => x -= 1,
        "+y" => y += 1,
        "-y" => y -= 1,
        "+z" => z += 1,
        "-z" => z -= 1,
        "+w" => w += 1,
        "-w" => w -= 1,
        _ => println!("unknown command: {input}")
    };
    println!("({x}, {y}, {z}, {w})");
    let block = world.get(&(x, y, z, w)).cloned().unwrap_or(Blox::Empty);
    println!("{:?}", block);
}
    }