use actix_web::web::Json;
use actix_web::{post, App, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct Card {
    number: String,
    ccv: String,
    first_name: String,
    last_name: String,
}

#[post("/api/card")]
async fn process(card: Json<Card>) -> impl Responder {
    if verify_card_number(&card.number)
        && verify_ccv(&card.ccv)
        && !card.first_name.is_empty()
        && !card.last_name.is_empty()
    {
        format!("valid")
    } else {
        format!("invalid")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(process))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

fn verify_card_number(card_number: &String) -> bool {
    if card_number.len() != 16 {
        return false;
    }
    let even_digits: Vec<u32> = card_number
        .chars()
        .step_by(2)
        .map(|char| char.to_digit(10).unwrap())
        .collect();
    let odd_digits: Vec<u32> = card_number[1..]
        .chars()
        .step_by(2)
        .map(|char| char.to_digit(10).unwrap())
        .collect();
    let even_sum: u32 = even_digits
        .into_iter()
        .map(|digit| {
            let digit = digit * 2;
            if digit > 9 {
                (digit % 10) + 1
            } else {
                digit
            }
        })
        .sum();
    let odd_sum: u32 = odd_digits.into_iter().sum();
    let sum = even_sum + odd_sum;
    return (sum % 10) == 0;
}

fn verify_ccv(ccv: &String) -> bool {
    return ccv.len() == 3;
}

#[cfg(test)]
mod tests {
    use crate::verify_card_number;

    #[test]
    fn valid_card_number_works() {
        let valid_card_number = "4640207097262595".to_string();
        assert!(verify_card_number(&valid_card_number));
    }

    #[test]
    fn short_card_number() {
        let short_card_number = "464020709726259".to_string();
        assert_ne!(verify_card_number(&short_card_number));
    }

    #[test]
    fn long_card_number() {
        let short_card_number = "46402070972625950".to_string();
        assert_ne!(verify_card_number(&short_card_number));
    }
}
