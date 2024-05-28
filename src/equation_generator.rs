use rand::prelude::ThreadRng;
use rand::Rng;

pub struct Settings {
    pub max_number: i32,
    pub operations: Vec<MathSign>,
    pub hide_operand: bool,
    pub can_be_negative: bool,
}

impl Settings {
    pub fn new(
        max_number: i32,
        include_add_sub: bool,
        include_mul: bool,
        include_div: bool,
        hide_operand: bool,
        can_be_negative: bool,
    ) -> Self {
        let mut operations_vector = Vec::new();
        if include_add_sub {
            operations_vector.push(MathSign::Plus);
            operations_vector.push(MathSign::Minus);
        }

        if include_mul {
            operations_vector.push(MathSign::Multiply);
        }
        if include_div {
            operations_vector.push(MathSign::Divide);
        }

        //Security operation - if settings set to show nothing
        if operations_vector.is_empty() {
            operations_vector.push(MathSign::Plus);
        }

        Self {
            max_number,
            operations: operations_vector,
            hide_operand,
            can_be_negative,
        }
    }
}

#[derive(Debug, Copy)]
pub enum MathSign {
    Plus,
    Minus,
    Multiply,
    Divide,
}

impl MathSign {
    fn value(&self) -> &str {
        match *self {
            MathSign::Plus => "+",
            MathSign::Minus => "-",
            MathSign::Multiply => "*",
            MathSign::Divide => "/",
        }
    }
}

impl Clone for MathSign {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Debug)]
pub struct Equation {
    pub first_num: i32,
    pub second_num: i32,
    pub sign: MathSign,
    pub result: i32,
    pub correct_answer: i32,
    pub string_representation: String,
}

impl Equation {
    pub fn new(settings: &Settings) -> Self {
        let mut rng = rand::thread_rng();
        let first_num = rng.gen_range(1..=settings.max_number);
        let sign = *settings
            .operations
            .get(rng.gen_range(0..settings.operations.len()))
            .unwrap();

        //Check if result number can be negative
        let second_num = match sign {
            MathSign::Minus => {
                if !settings.can_be_negative {
                    rng.gen_range(1..=first_num)
                } else {
                    rng.gen_range(1..=settings.max_number)
                }
            }
            MathSign::Divide => rng.gen_range(1..=settings.max_number),
            _ => rng.gen_range(1..=settings.max_number),
        };

        let mut temp_self = Self {
            first_num,
            second_num,
            sign,
            result: 0,
            correct_answer: 0,
            string_representation: String::new(),
        };

        temp_self.calculate(&mut rng);
        temp_self.hide_some_values(settings, &mut rng);

        temp_self
    }

    fn hide_some_values(&mut self, settings: &Settings, rng: &mut ThreadRng) {
        if !settings.hide_operand {
            self.correct_answer = self.result;
            self.string_representation.push_str(&format!(
                "{} {} {} = {}",
                self.first_num,
                self.sign.value(),
                self.second_num,
                "?"
            ));
        } else {
            let rng_result = rng.gen_range(0..=2);
            match rng_result {
                0 => {
                    self.correct_answer = self.result;
                    self.string_representation.push_str(&format!(
                        "{} {} {} = {}",
                        self.first_num,
                        self.sign.value(),
                        self.second_num,
                        "?"
                    ));
                }
                1 => {
                    self.correct_answer = self.first_num;
                    self.string_representation.push_str(&format!(
                        "{} {} {} = {}",
                        "?",
                        self.sign.value(),
                        self.second_num,
                        self.result
                    ));
                }
                2 => {
                    //Hiding second operand
                    self.correct_answer = self.second_num;
                    self.string_representation.push_str(&format!(
                        "{} {} {} = {}",
                        self.first_num,
                        self.sign.value(),
                        "?",
                        self.result
                    ));
                }

                _ => {
                    self.correct_answer = 0;
                    self.string_representation = "ERROR IN HIDE_SOME_VALUES PARSING".to_string()
                }
            }
        }
    }

    fn calculate(&mut self, rng: &mut ThreadRng) {
        self.result = match self.sign {
            MathSign::Plus => self.first_num + self.second_num,
            MathSign::Minus => self.first_num - self.second_num,
            MathSign::Multiply => self.first_num * self.second_num,
            MathSign::Divide => self.first_num * self.second_num,
        };

        if let MathSign::Divide = self.sign {
            let x_or_y = rng.gen_range(0..2);
            if x_or_y == 0 {
                let (temp_x, temp_y, temp_r) = (self.result, self.second_num, self.first_num);
                self.first_num = temp_x;
                self.second_num = temp_y;
                self.result = temp_r;
            } else {
                let (temp_x, temp_y, temp_r) = (self.result, self.first_num, self.second_num);
                self.first_num = temp_x;
                self.second_num = temp_y;
                self.result = temp_r;
            }
        }
    }

    pub fn check_answer(&self, x: i32) -> bool {
        x == self.correct_answer
    }
}
