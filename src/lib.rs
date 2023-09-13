use arbitrary::Arbitrary;

#[derive(Debug, Arbitrary, PartialEq)]
struct ArbitraryTest(u32);

#[cfg(test)]
mod tests {
    use super::{Arbitrary, ArbitraryTest};
    use arbitrary::Unstructured;

    #[test]
    fn test_arbitrary() {
        let raw_data: &mut [u8] = &mut [42, 2, 157, 7, 22, 35, 2, 4, 5, 6, 7, 8, 9, 10];

        let mut unstruct = Unstructured::new(raw_data);

        let mut out = vec![];

        for _ in 0..256 {
            let arbitrary_test = ArbitraryTest::arbitrary(&mut unstruct).unwrap();
            out.push(arbitrary_test);
        }

        for el in out.iter() {
            println!("{:?}", el);
        }
    }
}
