use num_format::{Locale, ToFormattedString};

pub fn amortized_loan(
  mut loan_amount: f64,
  period: f64,
  interest: f64
) {
  let number_of_month_in_year:f64 = 12.0;
  let principal: f64 = (loan_amount / period).floor();
  let mut total_payed_principal:i32 = 0;
  let mut total_payed_interest:i32 = 0;
  let mut total_payed_amount:i32 = 0;
  for i in 0..period as i64 {
      let interest_this_month: f64 = ((loan_amount * (interest / 100.0)) / number_of_month_in_year).floor();
      let total_need_to_pay:f64 = principal + interest_this_month;
      loan_amount = loan_amount - principal;
      total_payed_principal += principal as i32;
      total_payed_interest += interest_this_month as i32;
      total_payed_amount += total_need_to_pay as i32;
      println!("Month: {}", i + 1);
      println!("Principal: {}", (principal as i32).to_formatted_string(&Locale::en));
      println!("Interest This Month: {}", (interest_this_month as i32).to_formatted_string(&Locale::en));
      println!("Total Need To Pay: {}", (total_need_to_pay as i32).to_formatted_string(&Locale::en));
      println!("Remaining loan amount: {}", (loan_amount as i32).to_formatted_string(&Locale::en));
      println!("----------");
  }
  println!("Total Payed Principal: {}", total_payed_principal.to_formatted_string(&Locale::en));
  println!("Total Payed Interest: {}", total_payed_interest.to_formatted_string(&Locale::en));
  println!("Total Payed Amount: {}", total_payed_amount.to_formatted_string(&Locale::en));
}