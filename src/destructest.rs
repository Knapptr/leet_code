use std::rc::Rc;

struct Person {
    name: String,
}
#[test]
fn wtf() {
    let mut stack = vec![Person {
        name: "Tyler".to_string(),
    }];
    let Person { name } = stack.pop().unwrap();
    println!("{}", name);
}

#[test]
fn rc_counts() {
    let my_rc = Rc::new(Person {
        name: "Tyler".to_string(),
    });
    let mut my_vec = vec![];
    for x in 0..2 {
        my_vec.push(my_rc.clone());
    }
    let strong_count = Rc::strong_count(&my_rc);
    let weak_count = Rc::weak_count(&my_rc);
    println!("Strong: {}\nWeak: {}", strong_count, weak_count);
    my_vec.pop();
    let strong_count = Rc::strong_count(&my_rc);
    let weak_count = Rc::weak_count(&my_rc);
    println!("Strong: {}\nWeak: {}", strong_count, weak_count);
    drop(my_vec);
    let strong_count = Rc::strong_count(&my_rc);
    let weak_count = Rc::weak_count(&my_rc);
    println!("Strong: {}\nWeak: {}", strong_count, weak_count);

    let option = Some(my_rc.clone());
    let cloned_option = option.as_ref().cloned();
    let strong_count = Rc::strong_count(&my_rc);
    let weak_count = Rc::weak_count(&my_rc);
    println!("Strong: {}\nWeak: {}", strong_count, weak_count);
    panic!()
}
