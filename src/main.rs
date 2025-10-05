use chrono::NaiveDate;

use crate::expense::{Category, Expense, HomeExpense, SummaryType};

mod expense;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <command> [args]", args[0]);
        println!("Commands:");
        println!("  add         Add new expense");
        println!("  list        List all expense");
    }
    let mut home_expense = HomeExpense::new("expenses.json".to_string());

    match args[1].as_str() {
        "add" => {
            let description = args.get(2).expect("Description is required");
            let amount: f32 = args
                .get(3)
                .expect("Amount is required")
                .parse()
                .expect("Valid amount");
            let category: Category = args
                .get(4)
                .expect("Category is required")
                .parse()
                .expect("Valid category.");
            let date =
                NaiveDate::parse_from_str(args.get(5).expect("Date is required"), "%Y-%m-%d")
                    .expect("Valid date");

            let expense = Expense::new(description.clone(), amount, date, category);
            home_expense.add(expense);
        }
        "list" => {
            home_expense.list()
        }
        "delete" => {
            let deleted_id: i32 = args.get(2).unwrap().parse().expect("Id is required");
            home_expense.delete(deleted_id);
        }
        "test" => {
            println!("test 2")
        }
        "summary" => {
            let summary_type: SummaryType = args.get(2).unwrap().parse().expect("SummaryType is required");
            home_expense.summarize(summary_type);
        }
        _ => {}
    }
}
