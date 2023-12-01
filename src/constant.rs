use crate::number::Number;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct Constant {
    value: Number,
    description: String,
}

impl Constant {
    pub fn get_value(&self) -> Number {
        return self.value.clone();
    }

    pub fn get_description(&self) -> String {
        return self.description.clone();
    }
}

pub fn get_constants() -> HashMap<String, Rc<Constant>> {
    let mut constants: HashMap<String, Rc<Constant>> = HashMap::new();

    let pi = Rc::new(Constant {
        value: Number::from_str(
            "3.1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679",
        )
        .unwrap(),
        description: "the ratio of a circle's circumference to its diameter".to_string(),
    });

    constants.insert("pi".to_string(), pi.clone());
    constants.insert("π".to_string(), pi.clone());

    let e = Rc::new(Constant {
        value: Number::from_str("2.71828182845904523536028747135266249775724709369995").unwrap(),
        description: "Euler's number".to_string(),
    });
    constants.insert("e".to_string(), e);

    let c = Rc::new(Constant {
        value: Number::from_str("299,792,458 m/s").unwrap(),
        description: "the speed of light in vacuum".to_string(),
    });
    constants.insert("c".to_string(), c);

    return constants;
}
