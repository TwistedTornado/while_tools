/// A macro for creating binary nodes for the [`Ast`]. Used to keep other
/// macros DRYer.
#[macro_export]
macro_rules! binary_node {
    ($operation: ident, $left: expr, $right: expr ) => {
        Ast::$operation {
            left: Box::new($left),
            right: Box::new($right),
        }
    };
}

#[macro_export]
macro_rules! add {
    ( $left:expr, $right: expr ) => {
        binary_node!(Add, $left, $right)
    };
}

#[macro_export]
macro_rules! sub {
    ( $left:expr, $right: expr ) => {
        binary_node!(Sub, $left, $right)
    };
}

#[macro_export]
macro_rules! mul {
    ( $left:expr, $right: expr ) => {
        binary_node!(Mul, $left, $right)
    };
}

#[macro_export]
macro_rules! less_eq {
    ( $left:expr, $right: expr ) => {
        binary_node!(LessEq, $left, $right)
    };
}

#[macro_export]
macro_rules! eq {
    ( $left:expr, $right: expr ) => {
        binary_node!(Eq, $left, $right)
    };
}

#[macro_export]
macro_rules! and {
    ( $left:expr, $right: expr ) => {
        binary_node!(And, $left, $right)
    };
}

#[macro_export]
macro_rules! not {
    ( $expr:expr ) => {
        Ast::Not {
            expr: Box::new($expr),
        }
    };
}

#[macro_export]
macro_rules! ass_stmt {
    ( $ident:expr, $val: expr ) => {
        Ast::Ass {
            ident: $ident,
            value: Box::new($val),
        }
    };
}

#[macro_export]
macro_rules! if_stmt {
    ( $cond:expr, $affirm: expr, $neg: expr ) => {
        Ast::If {
            cond: Box::new($cond),
            true_path: Box::new($affirm),
            false_path: Box::new($neg),
        }
    };
}

#[macro_export]
macro_rules! while_stmt {
    ( $cond:expr, $body: expr ) => {
        Ast::While {
            cond: Box::new($cond),
            body: Box::new($body),
        }
    };
}

#[macro_export]
macro_rules! comp_stmt {
    ( $left:expr, $right: expr ) => {
        Ast::Comp {
            first: Box::new($left),
            second: Box::new($right),
        }
    };
}

#[macro_export]
macro_rules! skip_stmt {
    () => {
        Ast::Skip
    };
}

#[macro_export]
macro_rules! ident {
    ( $x:expr) => {
        Ast::Ident($x.to_string())
    };
}

#[macro_export]
macro_rules! literal {
    ( $x:literal) => {
        Ast::Literal($x)
    };
}
