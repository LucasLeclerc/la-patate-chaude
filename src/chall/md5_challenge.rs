use crate::chall::base::*;
use md5;
use md5::Digest;

pub struct Md5chall {
    input: StructInput,
}

impl Challenge for Md5chall {
    type Input = StructInput;
    type Output = StructOutput;

    fn name() -> String {
        "MD5HashCash".to_owned()
    }

    fn new(input: Self::Input) -> Self {
        Md5chall { input: input }
    }

    fn solve(&self) -> Self::Output {
        self.start()
    }

    fn verify(&self, answer: &Self::Output) -> bool {
        let result = self.start();
        result.seed == answer.seed && result.hashcode == answer.hashcode
    }
}

pub struct StructInput {
    pub complexity: u32,
    pub message: String,
}
pub struct StructOutput {
    pub seed: u64,
    pub hashcode: String,
}
impl Md5chall {
    pub fn start(&self) -> StructOutput {
        let mut finish = false;
        let mut seed: u64 = 0;
        let mut hex_seed;
        let mut concat_seed;
        let mut digest;
        let mut hash_code = "".to_string();
        let mut binary_hash;

        while finish == false {
            hex_seed = Self::format_dec_to_hex(seed);
            concat_seed = String::from(
                Self::concat_string(hex_seed.to_string(), self.input.message.to_string()).trim(),
            );
            digest = md5::compute(format!("{}", concat_seed).as_bytes());
            hash_code = Self::format_digest_to_hex(digest);
            binary_hash = Self::format_to_binary(hash_code.clone());
            finish = Self::check_seed(binary_hash, self.input.complexity);
            seed += 1;
        }
        return StructOutput {
            seed: seed,
            hashcode: hash_code.clone(),
        };
    }

    fn concat_string(seed: String, message: String) -> String {
        format!("{}{}\n", seed, message)
    }

    fn format_dec_to_hex(seed: u64) -> String {
        format!("{:016X}", seed)
    }

    fn format_digest_to_hex(digest: Digest) -> String {
        format!("{:032X}", digest)
    }

    fn format_to_binary(hashcode: String) -> String {
        hashcode.chars().map(Self::to_binary).collect()
    }

    fn check_seed(binary_hash: String, complexity: u32) -> bool {
        let mut index = 0;
        for character in binary_hash.chars() {
            if character == '1' && index < complexity {
                return false;
            } else if index >= complexity {
                return true;
            }
            index += 1;
        }
        return false;
    }

    fn to_binary(c: char) -> String {
        let b = match c {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'A' => "1010",
            'B' => "1011",
            'C' => "1100",
            'D' => "1101",
            'E' => "1110",
            'F' => "1111",
            _ => "",
        };
        return String::from(b);
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn format_dec_to_hex_test() {
        let seed = 844;
        let hex_seed = Md5chall::format_dec_to_hex(seed);
        assert_eq!("000000000000034C", hex_seed);
    }

    #[test]
    fn concat_string_test() {
        let seed = String::from("000000000000034C");
        let message = String::from("hello");
        assert_eq!(
            "000000000000034Chello",
            Md5chall::concat_string(seed, message).trim()
        );
    }

    #[test]
    fn md5_test() {
        let concat_seed = "000000000000034Chello";
        let digest = md5::compute(format!("{}", concat_seed).as_bytes());
        assert_eq!(
            String::from("00441745D9BDF8E5D3C7872AC9DBB2C3"),
            Md5chall::format_digest_to_hex(digest)
        );
    }

    #[test]
    fn format_to_binary_test() {
        let hascode = String::from("00441745D9BDF8E5D3C7872AC9DBB2C3");
        assert_eq!(String::from("00000000010001000001011101000101110110011011110111111000111001011101001111000111100001110010101011001001110110111011001011000011"), Md5chall::format_to_binary(hascode))
    }

    #[test]
    fn check_seed_test() {
        let binaire = String::from("00000000010001000001011101000101110110011011110111111000111001011101001111000111100001110010101011001001110110111011001011000011");
        let complexity = 9;
        assert_eq!(true, Md5chall::check_seed(binaire, complexity))
    }

    #[test]
    fn start_test() {
        let structure = StructOutput {
            seed: 845,
            hashcode: String::from("00441745D9BDF8E5D3C7872AC9DBB2C3"),
        };
        let chall = Md5chall {
            input: StructInput {
                complexity: 9,
                message: "hello".to_owned(),
            },
        };
        let res = chall.start();
        assert_eq!(structure.seed, res.seed);
        assert_eq!(structure.hashcode, res.hashcode);
    }
}
