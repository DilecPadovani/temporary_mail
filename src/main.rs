mod lib;

use lib::TempEmail;

fn main() {
    let temp_email = TempEmail::new();
    let adress = temp_email.get_address().to_string();
    println!("{}", adress);

    if let Some(emails) = temp_email.get_inbox() {
        emails.iter().for_each(|mail| println!("{:?}", mail));
    }
}
