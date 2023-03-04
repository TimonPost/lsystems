lsystem KochCurve {
    axiom F;

    replace F by F+F-F-F+F;
    interpret F as MoveForward(0.1);
    interpret - as RotateLeft(3.14/2);
    interpret + as RotateRight(3.14/2);
}