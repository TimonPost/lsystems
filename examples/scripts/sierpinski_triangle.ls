lsystem SierpinskiTriangle {
    axiom F-G-G;

    replace F by F-G+F+G-F;
    replace G by GG;

    interpret F as MoveForward(0.05);
    interpret G as MoveForward(0.05);
    interpret - as RotateLeft(2.094);
    interpret + as RotateRight(2.094);
}