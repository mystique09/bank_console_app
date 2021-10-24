use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;
use serde:: {
  Serialize,
  Deserialize
};

#[derive(Serialize, Deserialize, Debug)]
struct Account < 'a > {
  full_name: &'a str,
  pin: &'a str,
  balance: u64,
  over_all_balance: u64
}

fn main() {
  loop {
    bank_system_banner();
    let mut user_choice = String::new();

    io::stdin().read_line(&mut user_choice).expect("Unable to read line.");

    match user_choice.as_str().trim() {
      "q" | "quit" => {
        println!("Signing out...");
        println!("Signing out done.");
        break;
      },
      "d" | "dep" => account_action("deposit"),
      "w" | "withdraw" => account_action("withdraw"),
      "i" | "info" => read_account_data(),
      "c" | "create" => create_account(),
      _ => println!("Invalid choice!")
    }
  }
}

fn bank_system_banner() {
  println!("\nSimple banking console application!");
  println!("\nChoices :\n\t(c | create) to create new account.\n\t(i | info) to log your current accoubt info.\n\t(d | dep) to deposit.\n\t(w | withdraw) to withdraw.\n\t(q | quit) to exit.");
}

fn create_account < 'a > () {
  let mut fn_inp = String::new();
  let mut pin_inp = String::new();

  println!("Enter full name: ");
  io::stdin().read_line(&mut fn_inp).expect("Unable to read line!");

  println!("Enter pin/password: ");
  io::stdin().read_line(&mut pin_inp).expect("Unable to read line!");

  let new_account: Account = Account {
    full_name: fn_inp.as_str().trim(),
    pin: pin_inp.as_str().trim(),
    balance: 0,
    over_all_balance: 0
  };

  let file_path = Path::new(&new_account.full_name);

  // Check if the file doesn't exist
  // if it doesn't exist it means the user doesn't exist.
  // create new account.
  if let Err(_why) = fs::read(&file_path) {

    let new_account: &str = &serde_json::to_string(&new_account).unwrap();

    let mut account_file = match fs::File::create(file_path) {
      Err(why) => panic!("Couldn't create file: {}", why),
      Ok(file) => file
    };

    match account_file.write_all(new_account.as_bytes()) {
      Err(why) => panic!("Couldn't write to file: {}", why),
      Ok(_) => println!("Account added successfuly!")
    }
  } else {
    println!("\nAccount already exist!")
  }
}

fn read_account_data() {

  let mut fn_inp = String::new();

  println!("\nEnter your full name: ");
  io::stdin().read_line(&mut fn_inp).expect("\nUnable to read line.");

  let fullname = fn_inp.as_str().trim();

  match fs::read(&fullname) {
    Err(_why) => println!("Account doesn't exist!"),
    Ok(data) => {
      let content = match String::from_utf8(data) {
        Err(w) => panic!("Error: {}", w),
        Ok(v) => v
      };

      let account: Account = serde_json::from_str:: < Account > (&content).unwrap();

      println!("\nYour Account Information:\n\tFullname: {}\n\tBalance: {}\n\tBalance History: {}\n", &account.full_name, &account.balance, &account.over_all_balance);
    }
  };
}

fn account_action(action: &'static str) {
  let mut full_name_inp = String::new();
  let mut pin_inp = String::new();
  let mut amount_inp: String = String::new();

  println!("Enter full name: ");
  io::stdin().read_line(&mut full_name_inp).expect("Unable to read line");

  println!("Enter your pin/password: ");
  io::stdin().read_line(&mut pin_inp).expect("Unable to read line!");

  match fs::read(&full_name_inp.as_str().trim()) {
    Err(_why) => println!("Account doesn't exist!"),
    Ok(data) => {
      let content = match String::from_utf8(data) {
        Err(w) => panic!("Error: {}", w),
        Ok(v) => v
      };

      let mut account: Account = serde_json::from_str:: < Account > (&content).unwrap();

      if account.pin != pin_inp.as_str().trim() {
        println!("Incorrect Password!");
        return;
      }

      println!("Enter amount to withdraw:  ");
      io::stdin().read_line(&mut amount_inp).expect("Unable to read line!");

      let amount: u64 = match amount_inp.trim().parse() {
        Ok(num) => num,
        Err(_) => {
          0
        }
      };

      if amount == 0 {
        println!("Invalid amount! Must be greater than 0!")
      }

      if action == "deposit" {
        account.balance += amount;
        account.over_all_balance += amount;
        edit_account_data(&account, &full_name_inp.as_str().trim());
      }else if action == "withdraw" {
        if amount > account.balance {
          println!("Insuffiecient Balance!")
        } else {
          account.balance -= amount;
          edit_account_data(&account, &full_name_inp.as_str().trim())
        }
      }
    }
  };
}

fn edit_account_data (account: &Account, full_name_inp: &str) {
  println!("\nYour Current Account Information:\n\tFullname: {}\n\tBalance: {}\n\tBalance History: {}", &account.full_name, &account.balance, &account.over_all_balance);

      let _json_account = serde_json::to_string(&account).unwrap();

      let file = fs::OpenOptions::new()
      .write(true)
      .append(false)
      .open(&full_name_inp)
      .unwrap();

      let mut file = &file;
      match file.write_all(&_json_account.as_bytes()) {
        Err(why) => panic!("Couldn't write to file: {}", why),
        Ok(_) => println!("Success!")
      }println!("\nYour Current Account Information:\n\tFullname: {}\n\tBalance: {}\n\tBalance History: {}", &account.full_name, &account.balance, &account.over_all_balance);

      let _json_account = serde_json::to_string(&account).unwrap();

      let file = fs::OpenOptions::new()
      .write(true)
      .append(false)
      .open(&full_name_inp)
      .unwrap();

      let mut file = &file;
      match file.write_all(&_json_account.as_bytes()) {
        Err(why) => panic!("Couldn't write to file: {}", why),
        Ok(_) => println!("Success!")
      }
}