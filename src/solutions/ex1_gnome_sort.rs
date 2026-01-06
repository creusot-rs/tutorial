use creusot_contracts::{logic::OrdLogic, prelude::*};

#[logic(open)]
pub fn sorted_range<T: OrdLogic>(s: Seq<T>, l: Int, u: Int) -> bool {
    pearlite! {
        forall<i, j> l <= i && i < j && j < u ==> s[i] <= s[j]
    }
}

#[logic(open)]
pub fn sorted<T: OrdLogic>(s: Seq<T>) -> bool {
    sorted_range(s, 0, s.len())
}

#[ensures(sorted((^v).deep_model()))]
#[ensures((^v)@.permutation_of(v@))]
pub fn gnome_sort<T: Ord + DeepModel>(v: &mut [T])
where
    T::DeepModelTy: OrdLogic,
{
    let _old_v = snapshot! { v };
    let mut i = 0;
    #[invariant(sorted_range(v.deep_model(), 0, i@))]
    #[invariant(v@.permutation_of(_old_v@))]
    while i < v.len() {
        if i == 0 || v[i - 1] <= v[i] {
            i += 1;
        } else {
            v.swap(i - 1, i);
            i -= 1;
        }
    }
}

#[test]
fn test_1() {
    let mut v = [4, 2, 1, 3];
    gnome_sort(&mut v);
    assert_eq!(v, [1, 2, 3, 4]);
}

#[test]
fn test_2() {
    let mut v = [(4, 4), (2, 2), (1, 1), (3, 3)];
    gnome_sort(&mut v);
    assert_eq!(v, [(1, 1), (2, 2), (3, 3), (4, 4)]);
}
