lsystem LSystemName {
    let theta = 90;

    replace F by FF;
    replace a(x,y) by a(1, y+1)b(2,3) when x == 0;

    interpret A B C D E F G as DrawForward(8);
    interpret a b c d e f g as MoveForward(8);
    interpret + as Yaw(90);
    interpret -(x = 90) as Yaw(-x);
    interpret | as Yaw(180);
    interpret ^ as Pitch(90);
    interpret &(x = 90) as Pitch(-x);
    interpret / as Roll(90);
    interpret \(x = 90) as Roll(-x);
    interpret < as StartPolygon;
    interpret . as RecordPolygonVertex;
    interpret > as EndPolygon;
    interpret [ as StartBranch;
    interpret ] as EndBranch;
}

lsystem LSystemName {
    replace F by FF;
    interpret A as Test(5);
}


name      | [a-z, A-Z]+(<operator|numeric>) 
lsystem   | symbol
as        | ident
interpret | express
char      | action     | a-z, A-Z 
symbols    | action     | +, -, |, /, \, ., >, <,
numeric   | int        | [-0-9]+ 
operator  | op         | +, -, /, *
assignment | equal     | =
()        | express
[]        | branching


function call:  interpret <symbol|char> as string(expres)
assignment:     char = <numeric>
operation:      x <operator> number



interprete 
    |
    /\
   X  Y

 replace 
    |
    /\
   X  Y