use std::collections::HashMap;
use crate::Dump;




impl<K:Dump+Ord, V:Dump> Dump for HashMap<K, V> {
    fn dump(&self) -> String {
        let mut entries = self.iter().collect::<Vec<_>>();
        entries.sort_by(|(a,_), (b,_)|a.cmp(b));
        let entries = entries.into_iter().map(|(k, v)|format!("{}: {}", k.dump(), v.dump())).collect::<Vec<_>>().join(", ");
        format!("{{{}}}", entries)
    }
}
