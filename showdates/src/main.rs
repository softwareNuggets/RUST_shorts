use chrono::{Utc, NaiveDate};
use chrono::format::StrftimeItems;




fn main() {
   let now = Utc::now().naive_utc();
   let today = now.date();

    // Example: Jan-01-2023
    let formatted_date_1 = format_date(&today, "%b-%d-%Y"); 

    // Example: 01/01/2023
    let formatted_date_2 = format_date(&today, "%m/%d/%Y"); 

    println!("Formatted date 1: {}", formatted_date_1);
    println!("Formatted date 2: {}", formatted_date_2);
}

fn format_date(date: &NaiveDate, format: &str) -> String {
     /*
      https://docs.rs/chrono/latest/chrono/
				format/strftime/index.html

      	%Y: Year with century (e.g., 2023)
	%y: Year without century (e.g., 23)
	%m: Month number (e.g., 01 for January, 12 for December)
	%b: Abbreviated month name (e.g., Jan, Feb)
	%B: Full month name (e.g., January, February)
	%d: Day of the month (e.g., 01, 02, ..., 31)
	%H: Hour (24-hour format, e.g., 00, 01, ..., 23)
	%I: Hour (12-hour format, e.g., 01, 02, ..., 12)
	%M: Minute (e.g., 00, 01, ..., 59)
	%S: Second (e.g., 00, 01, ..., 59)
	%p: AM/PM designation (e.g., AM, PM)
	%A: Full weekday name (e.g., Sunday, Monday)
	%a: Abbreviated weekday name (e.g., Sun, Mon)
	%j: Day of the year (e.g., 001, 002, ..., 365)
    */

    let items = StrftimeItems::new(format);
    date.format_with_items(items).to_string()
}


