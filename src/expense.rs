use crate::expense::Category::{Entertainment, Food, Other, Transport};
use chrono::NaiveDate;
use chrono::format::ParseErrorKind;
use chrono::format::ParseErrorKind::Invalid;
use serde::{Deserialize, Serialize};
use std::fs;
use std::str::FromStr;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub(crate) enum Category {
    Food,
    Transport,
    Entertainment,
    Other,
}

#[derive(Serialize, Deserialize, Debug)]
enum SummaryType {
    Category,
    Date,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Expense {
    id: i32,
    description: String,
    amount: f32,
    date: String,
    category: Category,
}

impl Expense {
    pub(crate) fn new(
        description: String,
        amount: f32,
        date: NaiveDate,
        category: Category,
    ) -> Expense {
        Expense {
            id: -1,
            description,
            amount,
            category,
            date: date.to_string(),
        }
    }
    fn print(&self) {
        println!(
            "Expense, id: {}, description: {}, amount: {}, date: {}, category: {:?}",
            self.id, self.description, self.amount, self.date, self.category
        );
    }
}

pub(crate) struct HomeExpense {
    expenses: Vec<Expense>,
    path: String,
}
impl FromStr for Category {
    type Err = ParseErrorKind;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "food" => Ok(Food),
            "transport" => Ok(Transport),
            "entertainment" => Ok(Entertainment),
            "other" => Ok(Other),
            _ => Err(Invalid),
        }
    }
}
impl HomeExpense {
    pub(crate) fn new(path: String) -> HomeExpense {
        let expenses = Self::load_expenses(&path).unwrap_or_else(|_| Vec::new());
        HomeExpense { expenses, path }
    }
    fn load_expenses(path: &String) -> std::io::Result<Vec<Expense>> {
        let json = fs::read_to_string(path.as_str()).unwrap_or_else(|_| String::from("[]"));
        let expenses: Vec<Expense> = serde_json::from_str(json.as_str())?;
        Ok(expenses)
    }
    pub(crate) fn add(&mut self, expense: Expense) {
        let id = self.expenses.len() as i32;
        let new_expense = Expense { id, ..expense };
        self.expenses.push(new_expense);
        self.persist()
    }

    pub(crate) fn list(&self) {
        for e in &self.expenses {
            e.print()
        }
    }

    fn delete(&mut self, id: i32) {}

    fn summarize(self, summary_type: SummaryType) {}

    fn persist(&self) {
        let json = serde_json::to_string(&self.expenses);
        fs::write(&self.path, json.unwrap()).expect("Failed to save");
    }
}
