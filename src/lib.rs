#[test]
fn test_simple() {
    // let program = parse("begin push.1 push.2 add end");
    // println!("program: {:?}", program);
    // assert!(matches!(program[0], Instruction::Push(1)));
    // assert!(matches!(program[1], Instruction::Push(2)));
    // assert!(matches!(program[2], Instruction::Add));
}

#[test]
fn test_if() {
    // let program = parse("begin push.1 if.true push.3 add endif end");
    // println!("{:?}", program);
    // assert!(matches!(program[0], Instruction::Push(1)));
    // match &program[1] {
    //     Instruction::IfElse { t_branch, f_branch } => {
    //         assert!(matches!(t_branch[0], Instruction::Push(3)));
    //         assert!(matches!(t_branch[1], Instruction::Add));
    //         assert_eq!(f_branch.len(), 0);
    //     }
    //     _ => panic!("unexpected instruction: {:?}", program[1]),
    // }
}

#[test]
fn test_if_else() {
    // let program = parse("begin push.1 push.2 add if.true push.3 else push.4 endif mul end");
    // println!("{:?}", program);
    // assert!(matches!(program[0], Instruction::Push(1)));
    // assert!(matches!(program[1], Instruction::Push(2)));
    // assert!(matches!(program[2], Instruction::Add));
    // match &program[3] {
    //     Instruction::IfElse { t_branch, f_branch } => {
    //         assert!(matches!(t_branch[0], Instruction::Push(3)));
    //         assert!(matches!(f_branch[0], Instruction::Push(4)));
    //     }
    //     _ => panic!("unexpected instruction: {:?}", program[1]),
    // }
    // assert!(matches!(program[4], Instruction::Mul));
}
