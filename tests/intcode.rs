use std::collections::VecDeque;
use rust_intcode::intcode;


#[test]
fn test_intcode() {
    {   // mem[7] = mem[1] + mem[2]; write(mem[7]);
        let mut memory = vec![ 1, 1, 2, 7, 4, 7, 99, 0];
        let mut itape = VecDeque::new();
        let otape = intcode(&mut memory, &mut itape);
        assert_eq!(memory[7], 3);
        assert_eq!(otape.len(), 1);
        assert_eq!(otape[0], 3);
    }

    {   // https://adventofcode.com/2019/day/2 - first test case
        let mut memory = vec![1,9,10,3,2,3,11,0,99,30,40,50];
        let _otape = intcode(&mut memory, &mut VecDeque::new());
        assert_eq!(memory[0], 3500);
    }
    {   // https://adventofcode.com/2019/day/2
        let mut memory = vec![1,0,0,0,99];
        let _otape = intcode(&mut memory, &mut VecDeque::new());
        assert_eq!(memory, [2,0,0,0,99]);
    }
    {   // https://adventofcode.com/2019/day/2
        let mut memory = vec![2,3,0,3,99];
        let _otape = intcode(&mut memory, &mut VecDeque::new());
        assert_eq!(memory,[2,3,0,6,99] );
    }
    {   // https://adventofcode.com/2019/day/2
        let mut memory = vec![2,4,4,5,99,0];
        let _otape = intcode(&mut memory, &mut VecDeque::new());
        assert_eq!(memory, [2,4,4,5,99,9801]);
    }
    {   // https://adventofcode.com/2019/day/2
        let mut memory = vec![1,1,1,4,99,5,6,0,99];
        let _otape = intcode(&mut memory, &mut VecDeque::new());
        assert_eq!(memory, [30,1,1,4,2,5,6,0,99]);
    }

    {   // https://adventofcode.com/2019/day/5
        let mut memory = vec![3,0,4,0,99];
        let mut itape = VecDeque::from(vec![-25]);
        let otape = intcode(&mut memory, &mut itape);
        assert_eq!(memory[0], -25);
        assert_eq!(otape[0], -25);
    }
    {   // https://adventofcode.com/2019/day/5
        let mut memory = vec![1002,4,3,4,33];
        let _otape = intcode(&mut memory, &mut VecDeque::new());
        assert_eq!(memory[4], 99);
    }
    {   // https://adventofcode.com/2019/day/5
        let mut memory = vec![1101,100,-1,4, 0];
        let _otape = intcode(&mut memory, &mut VecDeque::new());
        assert_eq!(memory[4], 99);
    }
    {   //TODO https://adventofcode.com/2019/day/5 part 2
        let mut memory = vec![3,0,4,0,99];
        let mut itape = VecDeque::from(vec![-25]);
        let otape = intcode(&mut memory, &mut itape);
        assert_eq!(memory[0], -25);
        assert_eq!(otape[0], -25);
    }
} // test intcode

