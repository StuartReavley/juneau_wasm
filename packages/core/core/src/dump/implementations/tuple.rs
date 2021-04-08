use crate::Dump;




impl<A:Dump, B:Dump> Dump for (A, B) {
    fn dump(&self) -> String {
        let (a, b) = self;
        format!("({}, {})", a.dump(), b.dump())
    }
}
impl<A:Dump, B:Dump, C:Dump> Dump for (A, B, C) {
    fn dump(&self) -> String {
        let (a, b, c) = self;
        format!("({}, {}, {})", a.dump(), b.dump(), c.dump())
    }
}
impl<A:Dump, B:Dump, C:Dump, D:Dump> Dump for (A, B, C, D) {
    fn dump(&self) -> String {
        let (a, b, c, d) = self;
        format!("({}, {}, {}, {})", a.dump(), b.dump(), c.dump(), d.dump())
    }
}