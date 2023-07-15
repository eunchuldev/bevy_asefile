macro_rules! coalesce {
    ($a:expr) => ( $a );
    ($a:expr , $($c:expr),+) => (
        if let Some(v) = $a {
            v
        } else {
            coalesce!( $($c),+ )
        }
    );
    ($($a:expr),+ ,) => (coalesce!($($a),+))
}

pub(crate) use coalesce;
