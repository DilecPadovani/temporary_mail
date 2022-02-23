
# temporary_mail

### Rust wrapper of [1secmail](https://www.1secmail.com/api) temporary mail service



```rust
use temporary_mail::TempMail;

let temp_mail = TempMail::new();
```
From `TempMail` you can retrieve:

## Email address 
```rust
println!("{}", temp_mail.get_address());
```

## Email inbox 
```rust
let emails: Option<Vec<Email>> = temp_mail.get_inbox()

// print received emails
if let Some(emails) = emails {
    emails.iter().for_each(|mail| println!("{:?}", mail));
}
```





