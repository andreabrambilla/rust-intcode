use std::collections::VecDeque;

// -----------------------------------------------------------------------------
// --- Public interface --------------------------------------------------------
// -----------------------------------------------------------------------------

pub fn intcode(memory : &mut Vec<i32>, itape: &mut VecDeque<i32>) -> VecDeque<i32> {
    let mut otape = VecDeque::new();
    let mut ptr : usize = 0;
    while ptr < memory.len() {
        let (opcode, mode) = interpret(memory[ptr]);
        match opcode {
            1 => add             (memory,             &mode, &mut ptr),
            2 => mul             (memory,             &mode, &mut ptr),
            3 => read            (memory,      itape, &mode, &mut ptr),
            4 => write           (memory, &mut otape, &mode, &mut ptr),
            5 => jump_if_not_zero(memory,             &mode, &mut ptr),
            6 => jump_if_zero    (memory,             &mode, &mut ptr),
            7 => less_than       (memory,             &mode, &mut ptr),
            8 => equals          (memory,             &mode, &mut ptr),
            99 => break,
            _  => {},
        }
    } /* loop over memory */
    return otape;
}


// -----------------------------------------------------------------------------
// --- Implementation ----------------------------------------------------------
// -----------------------------------------------------------------------------

// --- INTCODE machinery ---

fn pulldigits(num: i32, magnitude: i32, div: i32) -> (i32, i32)
{
    let dig = num % magnitude;
    return (dig / div, num - dig);
}

fn interpret(intcode : i32) -> (i32, [i32; 3]) {
    let (op,  rest) = pulldigits(intcode,    100,     1);
    let (m0,  rest) = pulldigits(rest   ,   1000,   100);
    let (m1,  rest) = pulldigits(rest   ,  10000,  1000);
    let (m2, _rest) = pulldigits(rest   , 100000, 10000);
    return (op, [m0, m1, m2]);
}

fn evaluate(value: i32, mode: i32, memory : &Vec<i32>) -> i32 {
    if mode == 0 {
        assert!(value >= 0);
        return memory[value as usize];
    }
    return value;
}


// --- INTCODES implementation ---

fn add(memory: &mut Vec<i32>, mode: &[i32; 3], ptr: &mut usize) {
    let a = evaluate(memory[(*ptr)+1], mode[0], memory);
    let b = evaluate(memory[(*ptr)+2], mode[1], memory);
    let target = memory[(*ptr)+3];
    assert!(target >= 0);
    memory[target as usize] = a + b;
    (*ptr) += 4;
}


fn mul(memory: &mut Vec<i32>, mode: &[i32; 3], ptr: &mut usize) {
    let a = evaluate(memory[(*ptr)+1], mode[0], memory);
    let b = evaluate(memory[(*ptr)+2], mode[1], memory);
    let target = memory[(*ptr)+3];
    assert!(target >= 0);
    memory[target as usize] = a * b;
    (*ptr) += 4;
}

fn read(memory: &mut Vec<i32>, tape: &mut VecDeque<i32>, _mode: &[i32; 3], ptr: &mut usize) {
    let opt = tape.pop_front();
    let v = match opt {
        None    => panic!("Cannot read from input tape"),
        Some(v) => v
    };
    let target = memory[(*ptr)+1];
    assert!(target >= 0);
    memory[target as usize] = v;
    (*ptr) += 2;
}


fn write(memory: &Vec<i32>, tape: &mut VecDeque<i32>, mode: &[i32; 3], ptr: &mut usize) {
    tape.push_back(evaluate(memory[(*ptr)+1], mode[0], memory));
    (*ptr) += 2;
}


fn jump_if_not_zero(memory: &Vec<i32>, mode: &[i32; 3], ptr: &mut usize) {
    let a = evaluate(memory[(*ptr)+1], mode[0], memory);
    let b = evaluate(memory[(*ptr)+2], mode[1], memory);
    if a != 0 {
        assert!(b >= 0);
        *ptr = b as usize;
    } else {
        *ptr += 3;
    };
}


fn jump_if_zero(memory: &Vec<i32>, mode: &[i32; 3], ptr: &mut usize) {
    let a = evaluate(memory[(*ptr)+1], mode[0], memory);
    let b = evaluate(memory[(*ptr)+2], mode[1], memory);
    if a == 0 {
        assert!(b >= 0);
        *ptr = b as usize;
    } else {
        *ptr += 3;
    };
}


fn less_than(memory: &mut Vec<i32>, mode: &[i32; 3], ptr: &mut usize) {
    let a = evaluate(memory[(*ptr)+1], mode[0], memory);
    let b = evaluate(memory[(*ptr)+2], mode[1], memory);
    let target = memory[(*ptr)+3];
    assert!(target >= 0);
    memory[target as usize] = if a < b { 1 } else { 0 };
    (*ptr) += 4;
}


fn equals(memory: &mut Vec<i32>, mode: &[i32; 3], ptr: &mut usize) {
  let a = evaluate(memory[(*ptr)+1], mode[0], memory);
  let b = evaluate(memory[(*ptr)+2], mode[1], memory);
  let target = memory[(*ptr)+3];
  assert!(target >= 0);
  memory[target as usize] = if a == b { 1 } else { 0 };
  (*ptr) += 4;
}




#[cfg(test)]
mod test {
    use std::panic::catch_unwind;
    use super::{interpret, pulldigits, evaluate};
    use super::{add};


    #[test]
    fn test_interpret() {
        assert_eq!(interpret(    0), (0, [0,0,0]) );
        assert_eq!(interpret(11105), (5, [1,1,1]) );
        assert_eq!(interpret( 1002), (2, [0,1,0]) );
    }

    #[test]
    fn test_pulldigits() {
      assert_eq!(pulldigits(   0, 1000,  1), ( 0,   0 ) );
      assert_eq!(pulldigits(1234,  100,  1), (34, 1200) );
      assert_eq!(pulldigits(1234,  100, 10), ( 3, 1200) );
    }

    #[test]
    fn test_evaluate() {
        assert_eq!(evaluate(-3, 1, &vec![27]), -3);
        assert_eq!(evaluate( 1, 0, &vec![27, -22]), -22);
        assert!(catch_unwind(|| {  // negative address
                    evaluate(-1, 0, &vec![27, -22]) 
                }).is_err());
        assert!(catch_unwind(|| {  // address out of bounds
                    evaluate(27, 0, &vec![27, -22]) 
                }).is_err());
     }

    #[test]
    fn test_add() { 
        {   // mem[5] = mem[1] + mem[2]
            let mut memory = vec![ 1, 1, 2, 5, 99, 0];
            let mut ptr = 0;
            add(&mut memory, &[0,0,0], &mut ptr);
            assert_eq!(memory[5], 3);
        }
        // TODO: add other tests, including different params modes and negative values
    }

    // TODO: add tests for the other operations, including different params modes and negative values

} // mod test

