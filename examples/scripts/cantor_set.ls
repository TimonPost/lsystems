lsystem FractalBinaryTree {
    axiom A;

    replace A by ABA;
    replace B by BBB;

    interpret A as MoveForward(0.05);
    interpret B as MoveForward(0.1);
}