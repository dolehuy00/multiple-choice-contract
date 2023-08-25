use crate::{Contract, ContractExt};


use near_sdk::near_bindgen;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use std::collections::HashMap;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Question {
  pub question_id: String,
  pub title: String,
  pub answers: HashMap<String, String>,
  pub correct_answer: String,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonQuestion {
  pub question: String,
  pub answers: Vec<String>,
  pub correct: String,
}


pub trait ImplementMutilpleChoice{
  fn create_question(&mut self, question_id: String, title: String, answers: HashMap<String, String>, 
      correct_abc: String) -> Option<Question>;
  fn get_all_question(& self) -> Vec<Question>;
  fn get_questions(&self) -> Vec<JsonQuestion>;
}

#[near_bindgen]
impl ImplementMutilpleChoice for Contract{
  fn create_question(&mut self,
      question_id: String, 
      title: String, 
      answers: HashMap<String, String>, 
      correct_abc: String) -> Option<Question>{
        let exist_shop = self.questions.get(&question_id);
        if exist_shop.is_none() {   
          let correct_answer = answers.get(&correct_abc).unwrap().clone();
          let question = Question{question_id, title, answers, correct_answer};
          self.questions.insert(&question.question_id, &question);
          return Some(question);
        }
        return None; 
  }



  fn get_all_question(&self)-> Vec<Question>{
    let mut all_question = Vec::new();
    for i in self.questions.values() {
      all_question.push(i);
    }
    all_question
  }
  fn get_questions(&self) -> Vec<JsonQuestion>{
    let mut all_question = Vec::new();
    for i in self.questions.values() {
      let answers: Vec<String> = i.answers.into_values().collect();
      let json_question = JsonQuestion{question: i.title, answers, correct: i.correct_answer};
      all_question.push(json_question);
    }
    all_question
  }
}