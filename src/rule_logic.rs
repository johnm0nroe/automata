use rand::Rng;

pub type Automata = Vec<Vec<u8>>;
pub enum InitialPosition {
    Random,
    Left,
    Right,
    Center,
}

/* 
impl From<String> for InitialPosition {
    fn from(item: String) -> Self {
        let item_slice: &str = &item[..]; 
        match item_slice {
            "Random" => InitialPosition::Random,
            "Left" => InitialPosition::Left,
            "Right" => InitialPosition::Right,
            "Center" => InitialPosition::Center,
            _ => InitialPosition::Random 
        }
    }
}
*/

fn initialize_array(initial_position: InitialPosition, arr: &mut Vec<u8>) {
    let l: usize = arr.len();
    match initial_position {
        InitialPosition::Random => {
            let mut rng = rand::thread_rng();
            let r: usize = rng.gen_range(0..l);
            arr[r] = 1;
        },
        InitialPosition::Left => arr[0] = 1,
        InitialPosition::Right => arr[l-1] = 1,
        InitialPosition::Center => arr[l/2] = 1,
    };
}

fn rule_number_to_vector(rule_number: u8) -> Vec<u8> {
    let mut v: Vec<u8> = vec![0;8];
    let mut r: isize = rule_number.try_into().unwrap();
    let two: isize = 2;

    for n in 0..8 {
        let e = 7 - n;
        if (r - two.pow(e.try_into().unwrap())) >= 0 {
            v[e] = 1;
            r = r - two.pow(e.try_into().unwrap());
        }; 
    }    
    v
}
fn array_to_dec_number(t: &Vec<u8>) -> u8 {
    let a = t[0];
    let b = t[1];
    let c = t[2];
    let dec_number:u8 = 4*a + 2*b + 1*c;
    //println!("{}", dec_number);
    dec_number
}

fn determine_point(t: &Vec<u8>, rule_number: u8) -> u8 {
    let i: usize = array_to_dec_number(t).try_into().unwrap();
    let v: Vec<u8> = rule_number_to_vector(rule_number);
    let point: u8 = v[i];
    // how to index into an 8-bit number with a 3-bit(i8) number, so 
    // an i8-indexed into an i8
    //println!("index:{},vector:{:?},point:{}", i,v,point);
    return point
}

fn iterate_rule(rule_number: u8, arr: &Vec<u8>) -> Vec<u8> {
    // iterate over borrowed array with `determine_point`
    // to generate a new array which will be pushed onto the
    // final array in `generate_ecm`
    // generates a new row, which is a clearer definition

    let length: usize = arr.len();
    //println!("{}", length);
    let mut new_row: Vec<u8> = vec![0; length];
    let mut t: Vec<u8> = [0,0,0].to_vec();
    let mut n: usize = 0;
    
    while n < length {
        if n == 0 {
            t[0] = 0;
            t[1] = arr[n];
            t[2] = arr[n+1];
            new_row[n] = determine_point(&t, rule_number);
        }
        else if n == (length - 1) {
            t[0] = arr[n-1];
            t[1] = arr[n];
            t[2] = 0;
            new_row[n] = determine_point(&t, rule_number);          
        } else {
            t[0] = arr[n-1];
            t[1] = arr[n];
            t[2] = arr[n+1];
            new_row[n] = determine_point(&t, rule_number);
        }
        n = n + 1;
        //println!("column {}: {:?}", n, &t);    
    }
    
    return new_row
    
}

pub fn generate_ecm(iter: usize, breadth: usize, rule_number: u8, initial_position: &str) -> Automata {

    let mut arr:Vec<u8>= vec![0; breadth];
    let mut automata: Vec<Vec<u8>> = vec![];
    let mut n: usize = 0;

    let init_pos: InitialPosition = match initial_position {
        "random" => InitialPosition::Random,
        "left" => InitialPosition::Left,
        "right" => InitialPosition::Right,
        "center" => InitialPosition::Center,
        _ => InitialPosition::Center,
    };

    initialize_array(init_pos, &mut arr);
    automata.push(arr);   

    while n < iter {
        //println!("ROW {}", n);
        automata.push(iterate_rule(rule_number, &automata[n]));
        n = n+1;
    };    
    
    println!("rule {}: {:?}", rule_number, rule_number_to_vector(rule_number));
    return automata
    
}