lsystem FractalBinaryTree {
    axiom 0;

    replace 0 by 1[0]0;
    replace 1 by 11;

    interpret 0 as DrawLeaf(0.05);
    interpret 1 as DrawLine(0.1);
    interpret [ as PushStack(0.785398163);
    interpret ] as PopStack(0.785398163);
}