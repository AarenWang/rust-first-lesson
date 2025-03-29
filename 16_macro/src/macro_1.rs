// use macro_rules! <name of macro>{<Body>}
macro_rules! add {
    // macth like arm for macro
    ($a:expr,$b:expr) => {
        // macro expand to this code
        {
            // $a and $b will be templated using the value/variable provided to macro
            $a + $b
        }
    };
}

macro_rules! add_as {
    // using a ty token type for macthing datatypes passed to maccro
    ($a:expr,$b:expr,$typ:ty) => {
        $a as $typ + $b as $typ
    };
}

macro_rules! add_each{
    (
  // repeated block
  $($a:expr)
 // seperator
   ,
// zero or more
   *
   )=>{
       {
   // to handle the case without any arguments
   0
   // block to be repeated
   $(+$a)*
     }
    }
}

fn main() {
    // call to macro, $a=1 and $b=2
    let sum = add!(1, 2);
    assert_eq!(sum, 3);

    let sum = add_as!(1, 2, u8);
    assert_eq!(sum, 3);

    let sum = add_each!(1, 2, 3, 4);
    assert_eq!(sum, 10)
}
