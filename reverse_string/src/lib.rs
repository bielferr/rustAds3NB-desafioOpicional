

pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}

fn main(){
    let palavra ="rust";
    let resultado = reverse(palavra);
    println("{}", resultado)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse() {
        assert_eq!(reverse("shadow"), "wodahs");
        assert_eq!(reverse("rust"), "tsur");
        assert_eq!(reverse("12345"), "54321");
    }
}