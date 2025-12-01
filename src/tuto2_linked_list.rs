use creusot_contracts::{ghost::PtrOwn, logic::Mapping, prelude::*};

struct ListCell<T> {
    v: T,
    next: *const ListCell<T>,
}

pub struct List<T> {
    // actual data
    first: *const ListCell<T>,
    last: *const ListCell<T>,
    // ghost
    seq: Ghost<Seq<PtrOwn<ListCell<T>>>>,
}

impl<T> Invariant for List<T> {
    #[logic]
    fn invariant(self) -> bool {
        pearlite! {
            true
            // (*self.seq == Seq::empty() &&
            //  self.first.is_null_logic() &&
            //  self.last.is_null_logic())
            // ||
            // (self.seq.len() > 0 &&
            //  self.first == self.seq[0].ptr() &&
            //  self.last  == self.seq[self.seq.len() - 1].ptr() &&
            //  // the cells in `seq` are chained properly
            //  (forall<i> 0 <= i && i < self.seq.len() - 1 ==>
            //      self.seq[i].val().next == self.seq[i+1].ptr()) &&
            //  self.seq[self.seq.len() - 1].val().next.is_null_logic())
        }
    }
}

impl<T> View for List<T> {
    type ViewTy = Seq<T>;

    #[logic]
    fn view(self) -> Self::ViewTy {
        pearlite! {
            // TODO
            seq_map(*self.seq, |ptr_own: PtrOwn<ListCell<T>>| ptr_own.val().v)
        }
    }
}

#[logic]
pub fn seq_map<T, U>(s: Seq<T>, f: Mapping<T, U>) -> Seq<U> {
    Seq::create(s.len(), |i| f.get(s[i]))
}

impl<T> List<T> {
    // #[ensures(result@ == Seq::empty())]
    pub fn new() -> List<T> {
        List { first: std::ptr::null(), last: std::ptr::null(), seq: Seq::new() }
    }

    // #[ensures((^self)@ == (*self)@.push_back(x))]
    pub fn push_back(&mut self, x: T) {
        let cell = Box::new(ListCell { v: x, next: std::ptr::null() });
        // TODO
        let (cell_ptr, cell_own) = PtrOwn::from_box(cell);
        if self.last.is_null() {
            self.first = cell_ptr;
            self.last = cell_ptr;
        } else {
            let cell_last = unsafe {
                PtrOwn::as_mut(
                    self.last as *mut ListCell<T>,
                    todo!(),
                    // ghost! {
                    //     let off = self.seq.len_ghost() - 1int;
                    //     self.seq.get_mut_ghost(off).unwrap()
                    // },
                )
            };
            cell_last.next = cell_ptr;
            self.last = cell_ptr;
        }
        // ghost! { self.seq.push_back_ghost(cell_own.into_inner()) };
    }

    // #[ensures((^self)@ == (*self)@.push_front(x))]
    pub fn push_front(&mut self, x: T) {
        // TODO
        let (cell_ptr, cell_own) = PtrOwn::new(ListCell { v: x, next: self.first });
        self.first = cell_ptr;
        if self.last.is_null() {
            self.last = cell_ptr;
        }
        // ghost! { self.seq.push_front_ghost(cell_own.into_inner()) };
    }
}
