#[cfg(feature = "with-ring")]
extern crate ring;


fn main() {
    #[cfg(feature = "with-ring")]
        {
            use ring::rand::{SecureRandom, SystemRandom};

            println!("with ring");

            let mut buf = [0u8; 16];
            SystemRandom::new().fill(&mut buf).unwrap();

        }

    #[cfg(not(feature = "with-ring"))]
        {
            println!("without ring");
        }
}
