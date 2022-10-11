
#[macro_export]
macro_rules! lit {
    ($l:literal)=>{
        {
            use std::rc::Rc;
            use $crate::Literal;
            Rc::new(Literal::new($l))
        }
    }
}

#[macro_export]
macro_rules! seq {
    ($($e:expr),*)=>{
        {
            use std::rc::Rc;
            use $crate::Sequence;
            use $crate::Parser;
            let mut items:Vec<Rc<dyn Parser>> = Vec::new();
            $(
                items.push($e);
            )*  
            Rc::new(Sequence::new(items))
        }
    }
}

#[macro_export]
macro_rules! alt {
    ($($e:expr),*) => {
        {
            use std::rc::Rc;
            use $crate::Alternation;
            use $crate::Parser;
            let mut items:Vec<Rc<dyn Parser>> = Vec::new();
            $(
                items.push($e);
            )*  
            Rc::new(Alternation::new(items))
        }
    }
}

#[macro_export]
macro_rules! lbl {
    ($e:expr,$l:literal) => {
        {
            use std::rc::Rc;
            use $crate::Label;
            Rc::new(Label::new($e,$l))
        }
    }
}

#[macro_export]
macro_rules! rul {
    ($l:literal) => {
        {
            use std::rc::Rc;
            use $crate::RuleReference;
            Rc::new(RuleReference::new($l))
        }
    }
}
#[macro_export]
macro_rules! qtt {
    ($e:expr, $min:literal, $max:expr) => {
        {
            use std::rc::Rc;
            use $crate::Quantity;
            Rc::new(Quantity::new($e,$min,$max))
        }
    }
}

//pub fn new(pattern: &str, multi_line:bool, case_insensitive:bool,dot_matches_new_line:bool) -> Self {
macro_rules! reg_helper {
    ($pattern:tt, $multi_line:literal, $case_insensitive:literal, $dot_matches_new_line:literal) => {{
        use std::rc::Rc;
        use $crate::Regex;
        Rc::new(Regex::new($pattern, $multi_line,$case_insensitive,$dot_matches_new_line))
    }}
}

#[macro_export]
macro_rules! reg {
    ($pattern:tt     ) => {reg_helper!($pattern, false , false, false )};
    ($pattern:tt m   ) => {reg_helper!($pattern, true  , false, false )};
    ($pattern:tt i   ) => {reg_helper!($pattern, false , true , false )};
    ($pattern:tt s   ) => {reg_helper!($pattern, false , false, true  )};
    ($pattern:tt mi  ) => {reg_helper!($pattern, true  , true , false )};
    ($pattern:tt im  ) => {reg_helper!($pattern, true  , true , false )};
    ($pattern:tt ms  ) => {reg_helper!($pattern, true  , false, true  )};
    ($pattern:tt sm  ) => {reg_helper!($pattern, true  , false, true  )};
    ($pattern:tt is  ) => {reg_helper!($pattern, false , true , true  )};
    ($pattern:tt si  ) => {reg_helper!($pattern, false , true , true  )};
    ($pattern:tt mis ) => {reg_helper!($pattern, true  , true , true  )};
    ($pattern:tt msi ) => {reg_helper!($pattern, true  , true , true  )};
    ($pattern:tt ims ) => {reg_helper!($pattern, true  , true , true  )};
    ($pattern:tt ism ) => {reg_helper!($pattern, true  , true , true  )};
    ($pattern:tt smi ) => {reg_helper!($pattern, true  , true , true  )};
    ($pattern:tt sim ) => {reg_helper!($pattern, true  , true , true  )};
}