/********************************************************
*   __  _         _  _           __  _        _      _  *
*  / _|(_)       (_)| |         / _|(_)      | |    | | *
* | |_  _  _ __   _ | |_  ___  | |_  _   ___ | |  __| | *
* |  _|| || '_ \ | || __|/ _ \ |  _|| | / _ \| | / _` | *
* | |  | || | | || || |_|  __/ | |  | ||  __/| || (_| | *
* |_|  |_||_| |_||_| \__|\___| |_|  |_| \___||_| \__,_| *
*                                                       *
*  _                               _                    *
* (_)                             (_)                   *
* _  _ __ __   __ ___  _ __  ___  _   ___   _ __        *
* | || '_ \\ \ / // _ \| '__|/ __|| | / _ \ | '_ \      *
* | || | | |\ V /|  __/| |   \__ \| || (_) || | | |     *
* |_||_| |_| \_/  \___||_|   |___/|_| \___/ |_| |_|     *
*                                                       *
********************************************************/

/**
 * $ cargo run
 *
 **/
fn main() {
    let f = 0b10001_1011;
    let a = 0b0000_0110; // 0x86 = 134
    let mut u = 0b0000_0110;
    let mut v = 0b10001_1011;
    let mut g1 = 0b0000_0001;
    let mut g2 = 0b0000_0000;

    loop {
        println!("################################");
        println!(
            "Stampo i parametri attuali\n u vale: {:b}\n v vale: {:b}\n g1 vale: {:b}\n g2 vale: {:b}",
            u, v, g1, g2
        );

        // finché u è divisibile per \alpha shifto e modifico anche g1 come prescritto dall' algoritmo
        while u & 1 == 0 {
            u = u >> 1;

            if g1 & 1 == 0 {
                g1 = g1 >> 1;
            } else {
                g1 = (g1 ^ f) >> 1;
            }
        }

        if u == 1 {
            println!("################################");
            println!(
                "l' inverso di {:b} = {:#02X} = {} è {:b} = {:#02X} = {}",
                a, a, a, g1, g1, g1
            );
            return;
        }

        // XOR inversion, cioè scambio u con v e g1 con g2 se v ha grado maggiore di u
        if v > u {
            u ^= v;
            v ^= u;
            u ^= v;

            g1 ^= g2;
            g2 ^= g1;
            g1 ^= g2;
        }

        // Sommo in GF(2^8) u con v e g1 con g2
        u ^= v;
        g1 ^= g2;
    }
}
