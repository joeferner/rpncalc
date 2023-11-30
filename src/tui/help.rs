use crate::functions::Category;
use crate::rpn_calc::RpnCalc;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

pub fn create_help_string(rpn_calc: &RpnCalc) -> String {
    let mut result = "".to_string();

    result.push_str(".SH BASICS\n");
    result.push_str(
        "To enter a negative number, you can either enter the positive number \
        and negate it (neg or _) or enter a space followed by the negative number.\n",
    );
    result.push('\n');
    result.push_str(
        "To enter scientific notation number, enter the number followed by \"e\" then the \
        power (e.g. 1.23e4).\n",
    );
    result.push_str(".RE\n");
    result.push('\n');

    result.push_str(create_function_help_string(rpn_calc, Category::Arithmetic, "ARITHMETIC").as_str());
    result.push_str(create_function_help_string(rpn_calc, Category::Base, "BASE").as_str());
    result.push_str(create_function_help_string(rpn_calc, Category::Stack, "STACK").as_str());
    result.push_str(create_function_help_string(rpn_calc, Category::Trig, "TRIG").as_str());
    result.push_str(create_constants_help_string(rpn_calc).as_str());
    return result;
}

fn create_hash_set_help_string<T, F1, F2, F3, F4>(
    title: &str,
    hash_set: &HashMap<String, T>,
    should_include: F1,
    is_equal: F2,
    get_help: F3,
    get_additional_title: F4,
) -> String
where
    F1: Fn(&T) -> bool,
    F2: Fn(&T, &T) -> bool,
    F3: Fn(&T) -> String,
    F4: Fn(&T) -> Option<String>,
{
    let mut result = "".to_string();

    result.push_str(format!(".SH \"{}\"\n", title).as_str());
    let mut seen_keys: HashSet<String> = HashSet::new();
    let mut hash_set_keys: Vec<&String> = hash_set.keys().collect();
    hash_set_keys.sort();
    let find_equal_hash_set_keys = hash_set_keys.clone();
    for key in hash_set_keys {
        if seen_keys.contains(key.as_str()) {
            continue;
        }
        seen_keys.insert(key.clone().to_string());
        let f: &T = hash_set.get(key.as_str()).unwrap();
        if !should_include(f) {
            continue;
        }
        let mut key_str = key.to_string();

        // find function aliases
        for other_key in &find_equal_hash_set_keys {
            if seen_keys.contains(other_key.to_string().as_str()) {
                continue;
            }
            let other_key = other_key.to_string();
            let other_f: &T = hash_set.get(&other_key).unwrap();
            if is_equal(f, other_f) {
                key_str.push_str(format!(" or {}", other_key).as_str());
                seen_keys.insert(other_key.to_string());
            }
        }

        let fn_help = get_help(f);
        if let Some(additional_title) = get_additional_title(f) {
            key_str.push_str(additional_title.as_str());
        }
        result.push_str(format!(".IP \"{}\"\n", key_str).as_str());
        result.push_str(format!("{}\n", fn_help).as_str());
    }
    result.push_str(".RE\n");
    result.push('\n');

    return result;
}

fn create_function_help_string(rpn_calc: &RpnCalc, category: Category, category_str: &str) -> String {
    let title = format!("{} FUNCTIONS", category_str);
    return create_hash_set_help_string(
        title.as_str(),
        &rpn_calc.functions,
        |f| f.get_category() == category,
        Rc::ptr_eq,
        |a| a.get_help(),
        |_| None,
    );
}

fn create_constants_help_string(rpn_calc: &RpnCalc) -> String {
    return create_hash_set_help_string(
        "CONSTANTS",
        &rpn_calc.constants,
        |_c| true,
        |a, b| a == b,
        |a| a.get_description(),
        |a| Some(format!(" = {}", a.get_value().to_string_format(10, 10))),
    );
}
