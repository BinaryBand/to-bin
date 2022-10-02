type Callback = fn(num: &String) -> String;
const ERROR_MESSAGE: &str = "Cannot parse number.";

fn is_float(num: &String) -> bool {
    num.contains('.')
}

fn int_to_bin(int: i32) -> String {
    let binary: String = format!("{:b}", int);
    format!("{:0>32}", binary)
}

fn float_to_bin(float: f32) -> String {
    let binary: String = format!("{:b}", float.to_bits());
    format!("{:0>32}", binary)
}

pub fn to_bin(arg: &String) -> String {
    if is_float(arg) {
        let num: f32 = arg.trim().parse().expect(ERROR_MESSAGE);
        float_to_bin(num)
    }
    else {
        let num: i32 = arg.trim().parse().expect(ERROR_MESSAGE);
        int_to_bin(num)
    }
}

pub fn iterate(args: Vec<String>, c: Callback, start: usize) -> () {
    for i in start..args.len() {
        let arg: &String = &args[i];
        let bin: String = c(arg);
        println!("{}", bin);
    }
}
